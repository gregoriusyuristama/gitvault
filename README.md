# gitvault

**Encrypted Git-backed directory backup CLI.**

`gitvault` compresses a directory into `tar.zst`, encrypts with an [`age`](https://age-encryption.org) passphrase, and pushes the ciphertext to a Git repository. Only you can decrypt — the Git repo can be public without leaking data.

Inspired by [Fastlane Match](https://docs.fastlane.tools/actions/match/), for anything you want to back up (Homebridge configs, n8n workflows, dotfiles, small archives).

## Features

- **Client-side encryption**: `age` (ChaCha20-Poly1305 AEAD, scrypt passphrase KDF).
- **Compression**: `zstd` streaming.
- **Version history**: every backup is a Git commit + annotated tag. Roll back to any snapshot.
- **Public-repo-safe**: even the filenames inside your directory are opaque to GitHub.
- **Single static binary**: no runtime dependencies (except system `git`).
- **Agent Skill**: automate backups from any AI agent that supports Hermes/Claude skills.

## Install

### Homebrew
```bash
brew tap gregoriusyuristama/tap
brew install gitvault
```

### Shell installer
```bash
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install.sh | bash
```

### From source
```bash
git clone https://github.com/gregoriusyuristama/gitvault.git
cd gitvault
cargo build --release
```

## Usage

```bash
# Set credentials (never commit these).
export GITVAULT_REPO="git@github.com:you/vault.git"
export GITVAULT_PASSPHRASE="a-long-random-passphrase"

# Back up a directory.
gitvault backup ~/.homebridge --name homebridge

# List all backups.
gitvault list

# Restore latest.
gitvault restore homebridge --target ~/homebridge-restored

# Restore a specific snapshot.
gitvault restore homebridge --target ~/rollback --tag homebridge-20260912-170458
```

## Security

- **Your passphrase never leaves your machine.** All encryption is client-side.
- **Losing the passphrase = losing the backup.** Store it somewhere safe (password manager, hardware token).
- The Git repo shows only:
  - Commit history / timestamps.
  - Backup names (e.g. `homebridge`, `n8n`).
  - Ciphertext blob size + SHA-256.
- Everything inside your backup — filenames, contents, structure — is encrypted.

## Repository layout

```
<your-vault-repo>/
└── (branch: gitvault-backups)
    └── backups/
        ├── homebridge/
        │   ├── latest.tar.zst.age
        │   └── manifest.json
        └── n8n/
            ├── latest.tar.zst.age
            └── manifest.json
```

Every push also creates a tag: `<name>-<YYYYMMDD-HHMMSS>`.

## License

MIT
