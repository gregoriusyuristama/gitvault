# gitvault

**Encrypted Git-backed directory backup CLI & AI Agent Skill.**

`gitvault` compresses a directory into `tar.zst`, encrypts with an [`age`](https://age-encryption.org) passphrase, and pushes the ciphertext to a Git repository. Only you can decrypt — the Git repo can be public without leaking data.

Inspired by [Fastlane Match](https://docs.fastlane.tools/actions/match/), for anything you want to back up (Homebridge configs, n8n workflows, dotfiles, small archives).

---

## Installation

### 1. Install CLI Tool

#### Homebrew (macOS & Linux)
```bash
brew tap gregoriusyuristama/tap
brew install gitvault
```

#### One-Line Shell Installer
```bash
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install.sh | bash
```

#### From Source
```bash
git clone https://github.com/gregoriusyuristama/gitvault.git
cd gitvault
cargo build --release
```

---

## 🤖 Install as AI Agent Skill (Hermes, Claude Code, etc.)

`gitvault` comes with an official Agent Skill (`SKILL.md`) that teaches autonomous coding and operations agents how to safely back up, restore, and verify directories without leaking passphrases.

### One-Line Skill Install (Auto-Detect)

Runs auto-detection for **Hermes Agent** (`~/.hermes/skills/`) and **Claude Code** (`~/.claude/skills/`):

```bash
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash
```

### Framework-Specific Install

#### Hermes Agent
```bash
# Target active profile or global skills:
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash -s -- hermes

# Or manually:
mkdir -p ~/.hermes/skills/gitvault
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/skills/gitvault/SKILL.md -o ~/.hermes/skills/gitvault/SKILL.md
```

#### Claude Code CLI
```bash
# Target global Claude Code skills:
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash -s -- claude

# Or for a specific project repo:
mkdir -p .claude/skills/gitvault
curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/skills/gitvault/SKILL.md -o .claude/skills/gitvault/SKILL.md
```

### Prompting Your Agent

Once installed, simply prompt your agent naturally:

> **"Back up my ~/.homebridge configuration to gitvault under name homebridge"**

> **"Restore n8n workflows from the latest gitvault snapshot to ./n8n-data"**

> **"List all backups currently stored in my gitvault repository"**

---

## CLI Usage

```bash
# 1. Set credentials (recommended via env vars to avoid shell history leakage)
export GITVAULT_REPO="git@github.com:you/vault.git"
export GITVAULT_PASSPHRASE="a-long-random-passphrase"

# 2. Back up a directory
gitvault backup ~/.homebridge --name homebridge

# 3. List all backups
gitvault list

# 4. Restore latest
gitvault restore homebridge --target ~/homebridge-restored

# 5. Restore a specific snapshot tag
gitvault restore homebridge --target ~/rollback --tag homebridge-20260912-170458
```

---

## Security Model

- **Zero-knowledge Git storage:** Encryption is 100% client-side before any Git interaction.
- **Passphrase safety:** Passphrase is never written to disk, never committed to git, and never echoed to stdout.
- **Public Git repositories are safe:** GitHub only sees an encrypted `.age` blob and a high-level `manifest.json` (metadata timestamp, sha256 checksum, uncompressed size). Directory structure and file contents are completely unreadable without your passphrase.
- **Rollback & Immutability:** Every backup creates an annotated Git tag (`<name>-<timestamp>`), allowing point-in-time recovery.

---

## Repository Layout

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

---

## License

MIT © Hermes Corporation & Gregorius Yuristama
