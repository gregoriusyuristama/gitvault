---
name: gitvault
description: Use to back up or restore directories via `gitvault` — encrypted (`age`) Git-backed snapshots. Trigger on "back up X", "restore X from vault", "list gitvault backups".
---

# gitvault: Encrypted Git-backed Backups

`gitvault` is a Rust CLI that packs a directory into `tar.zst`, encrypts with an `age` passphrase, and pushes to a Git repo. Only the user's passphrase can decrypt — repo can be public.

## When to use

- User asks to back up a config directory (Homebridge, n8n, dotfiles) to Git.
- User asks to restore or roll back from a `gitvault` snapshot.
- User asks to list existing backups.

## Prerequisites

1. `gitvault` on `PATH` (`brew install <tap>/gitvault` or downloaded release binary).
2. Git installed with credentials able to push to the target repo (SSH key or token).
3. Environment variables (recommended — never inline the passphrase in chat/logs):
   - `GITVAULT_REPO` — Git repo URL (SSH `git@github.com:user/vault.git` or file path).
   - `GITVAULT_PASSPHRASE` — user's encryption passphrase.
   - `GITVAULT_BRANCH` — optional, default `gitvault-backups`.

## Commands

### Back up a directory

```bash
gitvault backup <SOURCE_PATH> --name <BACKUP_NAME>
# Uses GITVAULT_REPO + GITVAULT_PASSPHRASE from env.
```

Explicit form (avoid — leaks passphrase to shell history):

```bash
gitvault backup ~/.homebridge --name homebridge \
  --repo git@github.com:user/vault.git \
  --passphrase "$PASS"
```

Output on success:
```
gitvault: pushed backup 'homebridge' (<N> bytes ciphertext, sha256=<hex>)
  tag: homebridge-<YYYYMMDD-HHMMSS>
```

### Restore a directory

```bash
gitvault restore <BACKUP_NAME> --target <DEST_PATH>
# Latest snapshot. Add --tag <tag> for a specific historical snapshot.
```

### List backups

```bash
gitvault list
# Shows one row per backup with timestamp, sha256, size.
```

## Agent workflow

Follow these steps end-to-end when the user asks for a backup:

1. **Verify prerequisites.** Run `gitvault --version` and `command -v git`. If missing, tell user how to install (brew tap or release binary URL). Do NOT proceed.
2. **Confirm target.** Ask user for source path and `--name` label if not clear. Warn if source is huge (>1 GB) — will be slow.
3. **Get credentials.** Check `GITVAULT_REPO` and `GITVAULT_PASSPHRASE` in env. If missing:
   - Ask user for repo URL.
   - **Never store or echo the passphrase.** Instruct user to `export GITVAULT_PASSPHRASE=...` themselves before invoking, or pipe via stdin.
4. **Run backup.** Capture stdout and exit code. On non-zero exit, surface stderr verbatim.
5. **Verify.** Report tag name and sha256 back to user. Optionally run `gitvault list` and confirm the new entry appears.
6. **Restore workflow** mirrors this: verify prerequisites → confirm target dir (warn if it will overwrite) → decrypt → confirm success via file listing.

## Security rules for the agent

- **NEVER put the passphrase in a chat message, shell history, or log.** Always source from env or user's interactive prompt.
- **NEVER commit or push the passphrase** to any repo (source or backup).
- The Git repo can be public: the ciphertext is `age`-protected. But the manifest reveals backup names, timestamps, and sizes — mention this if the user asks about privacy.
- Wrong passphrase = permanent data loss. Confirm with user that they have their passphrase stored safely before a first backup.

## Pitfalls

- **Empty repo without target branch.** `gitvault` creates an orphan `gitvault-backups` branch automatically on first push, but the remote repo itself must already exist.
- **Multiple machines pushing at once.** Concurrent backups may race on the same branch. Serialize backups per repo, or add a per-machine `--branch` suffix.
- **Large directories.** Everything is packed in memory. For >1 GB sources, consider splitting into multiple `--name`d backups.
- **Symlinks.** `tar` preserves symlinks, but they will be restored as symlinks — validate targets exist post-restore.

## Verification

After every backup, verify the round-trip locally before trusting the snapshot:

```bash
TMP=$(mktemp -d)
gitvault restore <NAME> --target "$TMP/verify"
diff -r <ORIGINAL_SOURCE> "$TMP/verify" && echo "OK: backup verified" || echo "MISMATCH"
rm -rf "$TMP"
```
