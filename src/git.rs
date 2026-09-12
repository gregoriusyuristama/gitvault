//! Git module: shell out to system `git` for repository operations.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GitManager {
    repo_url: String,
    branch: String,
}

impl GitManager {
    pub fn new(repo_url: impl Into<String>, branch: impl Into<String>) -> Self {
        Self {
            repo_url: repo_url.into(),
            branch: branch.into(),
        }
    }

    /// Prepare a working directory with the target branch checked out.
    /// - If the branch exists on the remote, shallow-clone it.
    /// - If not, clone the default branch and create an orphan branch locally.
    pub fn prepare_workdir(&self, workdir: &Path) -> Result<()> {
        std::fs::create_dir_all(workdir)?;
        // Try to clone the specific branch (shallow).
        let clone = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--branch",
                &self.branch,
                &self.repo_url,
                ".",
            ])
            .current_dir(workdir)
            .output()
            .context("failed to invoke git clone")?;

        if clone.status.success() {
            return Ok(());
        }

        // Branch doesn't exist yet: clone default HEAD, then create orphan branch.
        std::fs::read_dir(workdir)
            .ok()
            .into_iter()
            .flatten()
            .flatten()
            .for_each(|e| {
                let _ = std::fs::remove_dir_all(e.path());
            });

        let clone_default = Command::new("git")
            .args(["clone", "--depth", "1", &self.repo_url, "."])
            .current_dir(workdir)
            .output()
            .context("failed to invoke git clone (default branch)")?;

        if !clone_default.status.success() {
            // Repository is empty or unreachable: init a fresh one.
            run(workdir, &["init"])?;
            run(workdir, &["remote", "add", "origin", &self.repo_url])?;
            run(workdir, &["checkout", "--orphan", &self.branch])?;
            return Ok(());
        }

        run(workdir, &["checkout", "--orphan", &self.branch])?;
        // Remove any inherited files: fresh branch begins with a clean tree.
        let entries: Vec<PathBuf> = std::fs::read_dir(workdir)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().and_then(|s| s.to_str()) != Some(".git"))
            .collect();
        for p in entries {
            if p.is_dir() {
                std::fs::remove_dir_all(&p)?;
            } else {
                std::fs::remove_file(&p)?;
            }
        }
        run(workdir, &["rm", "-rf", "--cached", "--ignore-unmatch", "."])?;
        Ok(())
    }

    /// Commit all staged changes and push (with tags) to the remote.
    /// Returns the created tag name.
    pub fn commit_tag_push(&self, workdir: &Path, backup_name: &str) -> Result<String> {
        let ts = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();
        let tag = format!("{}-{}", backup_name, ts);
        let msg = format!("[gitvault] Backup {} at {}", backup_name, ts);

        // Ensure identity is configured locally (safe defaults for CI/agents).
        let _ = run(workdir, &["config", "user.email", "gitvault@local"]);
        let _ = run(workdir, &["config", "user.name", "gitvault"]);

        run(workdir, &["add", "-A"])?;
        run(workdir, &["commit", "-m", &msg])?;
        run(workdir, &["tag", "-a", &tag, "-m", &msg])?;
        run(workdir, &["push", "--set-upstream", "origin", &self.branch])?;
        run(workdir, &["push", "origin", &tag])?;
        Ok(tag)
    }

    /// Fetch a specific ref (branch or tag) into the workdir.
    pub fn fetch_ref(&self, workdir: &Path, git_ref: &str) -> Result<()> {
        std::fs::create_dir_all(workdir)?;
        let clone = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--branch",
                git_ref,
                &self.repo_url,
                ".",
            ])
            .current_dir(workdir)
            .output()
            .context("failed to git clone ref")?;
        if !clone.status.success() {
            bail!(
                "git clone --branch {} failed: {}",
                git_ref,
                String::from_utf8_lossy(&clone.stderr)
            );
        }
        Ok(())
    }
}

fn run(workdir: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .args(args)
        .current_dir(workdir)
        .output()
        .with_context(|| format!("failed to invoke git {:?}", args))?;
    if !out.status.success() {
        bail!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}
