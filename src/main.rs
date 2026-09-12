use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use gitvault::{
    archive::{pack_directory, unpack_directory},
    crypto::{decrypt_stream, encrypt_stream},
    git::GitManager,
    manifest::Manifest,
};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "gitvault",
    version,
    about = "Encrypted Git-backed directory backup"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Back up a directory to a Git repository as an encrypted archive.
    Backup {
        source: PathBuf,
        #[arg(short, long)]
        name: String,
        #[arg(short, long, env = "GITVAULT_REPO")]
        repo: String,
        #[arg(short, long, env = "GITVAULT_PASSPHRASE", hide_env_values = true)]
        passphrase: Option<String>,
        #[arg(short, long, env = "GITVAULT_BRANCH", default_value = "gitvault-backups")]
        branch: String,
    },
    /// Restore a directory from a backup archive.
    Restore {
        name: String,
        #[arg(short, long)]
        target: PathBuf,
        #[arg(short, long, env = "GITVAULT_REPO")]
        repo: String,
        #[arg(short, long, env = "GITVAULT_PASSPHRASE", hide_env_values = true)]
        passphrase: Option<String>,
        #[arg(short, long, env = "GITVAULT_BRANCH", default_value = "gitvault-backups")]
        branch: String,
        #[arg(long)]
        tag: Option<String>,
    },
    /// List backups available in the repository.
    List {
        #[arg(short, long, env = "GITVAULT_REPO")]
        repo: String,
        #[arg(short, long, env = "GITVAULT_BRANCH", default_value = "gitvault-backups")]
        branch: String,
    },
}

fn resolve_passphrase(pass: Option<String>, confirm: bool) -> Result<String> {
    if let Some(p) = pass {
        if p.is_empty() {
            bail!("passphrase is empty");
        }
        return Ok(p);
    }
    let p =
        rpassword::prompt_password("gitvault passphrase: ").context("failed to read passphrase")?;
    if p.is_empty() {
        bail!("passphrase is empty");
    }
    if confirm {
        let again = rpassword::prompt_password("confirm passphrase: ")
            .context("failed to confirm passphrase")?;
        if again != p {
            bail!("passphrases do not match");
        }
    }
    Ok(p)
}

fn do_backup(
    source: PathBuf,
    name: String,
    repo: String,
    passphrase: Option<String>,
    branch: String,
) -> Result<()> {
    if !source.exists() {
        bail!("source path does not exist: {}", source.display());
    }
    let pass = resolve_passphrase(passphrase, true)?;

    // 1. Pack directory into tar.zst in memory.
    let mut packed = Vec::new();
    let src_dir = if source.is_dir() {
        source.clone()
    } else {
        // Single-file backup: stage into a temp dir preserving the filename.
        let staging = tempfile::tempdir()?;
        let staged = staging.path().join(source.file_name().unwrap());
        fs::copy(&source, &staged)?;
        // Repack from staging dir; keep the tempdir alive by leaking to end of fn.
        staging.keep()
    };
    pack_directory(&src_dir, &mut packed).context("failed to pack source directory")?;

    // 2. Encrypt.
    let mut ciphertext = Vec::new();
    encrypt_stream(&packed[..], &mut ciphertext, &pass).context("encryption failed")?;

    // 3. Build manifest.
    let manifest = Manifest::new(&name, &ciphertext);

    // 4. Prepare working git repo.
    let workdir = tempfile::tempdir()?;
    let git = GitManager::new(&repo, &branch);
    git.prepare_workdir(workdir.path())?;

    // 5. Write payload + manifest.
    let backup_dir = workdir.path().join("backups").join(&name);
    fs::create_dir_all(&backup_dir)?;
    fs::write(backup_dir.join("latest.tar.zst.age"), &ciphertext)?;
    fs::write(
        backup_dir.join("manifest.json"),
        manifest.to_json_pretty()?,
    )?;

    // 6. Commit, tag, push.
    let tag = git.commit_tag_push(workdir.path(), &name)?;
    println!(
        "gitvault: pushed backup '{}' ({} bytes ciphertext, sha256={})\n  tag: {}",
        name,
        ciphertext.len(),
        manifest.sha256,
        tag
    );
    Ok(())
}

fn do_restore(
    name: String,
    target: PathBuf,
    repo: String,
    passphrase: Option<String>,
    branch: String,
    tag: Option<String>,
) -> Result<()> {
    let pass = resolve_passphrase(passphrase, false)?;
    let workdir = tempfile::tempdir()?;
    let git = GitManager::new(&repo, &branch);
    let git_ref = tag.as_deref().unwrap_or(&branch);
    git.fetch_ref(workdir.path(), git_ref)?;

    let backup_dir = workdir.path().join("backups").join(&name);
    let payload_path = backup_dir.join("latest.tar.zst.age");
    let manifest_path = backup_dir.join("manifest.json");
    if !payload_path.exists() {
        bail!("backup '{}' not found in repository", name);
    }

    let ciphertext = fs::read(&payload_path)?;
    if manifest_path.exists() {
        let manifest = Manifest::from_json(&fs::read_to_string(&manifest_path)?)?;
        if !manifest.verify(&ciphertext) {
            bail!(
                "manifest sha256 mismatch for backup '{}': archive may be corrupt",
                name
            );
        }
    }

    let mut plaintext = Vec::new();
    decrypt_stream(Cursor::new(ciphertext), &mut plaintext, &pass)?;
    unpack_directory(&plaintext[..], &target)?;
    println!(
        "gitvault: restored backup '{}' -> {}",
        name,
        target.display()
    );
    Ok(())
}

fn do_list(repo: String, branch: String) -> Result<()> {
    let workdir = tempfile::tempdir()?;
    let git = GitManager::new(&repo, &branch);
    git.fetch_ref(workdir.path(), &branch)?;
    let backups_dir = workdir.path().join("backups");
    if !backups_dir.exists() {
        println!("(no backups yet)");
        return Ok(());
    }
    for entry in fs::read_dir(&backups_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let manifest_path = entry.path().join("manifest.json");
        if let Ok(s) = fs::read_to_string(&manifest_path) {
            if let Ok(m) = Manifest::from_json(&s) {
                println!(
                    "{:<24} {}  sha256={}  {}b",
                    name, m.timestamp, m.sha256, m.ciphertext_bytes
                );
                continue;
            }
        }
        println!("{}", name);
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Backup {
            source,
            name,
            repo,
            passphrase,
            branch,
        } => do_backup(source, name, repo, passphrase, branch),
        Commands::Restore {
            name,
            target,
            repo,
            passphrase,
            branch,
            tag,
        } => do_restore(name, target, repo, passphrase, branch, tag),
        Commands::List { repo, branch } => do_list(repo, branch),
    }
}
