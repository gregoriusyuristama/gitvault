#!/usr/bin/env bash
# install-skill.sh: Installs gitvault skill into Hermes, Claude Code, or custom directory.
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash -s -- claude
#   curl -fsSL https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/scripts/install-skill.sh | bash -s -- hermes
set -euo pipefail

SKILL_URL="https://raw.githubusercontent.com/gregoriusyuristama/gitvault/main/skills/gitvault/SKILL.md"
TARGET="${1:-auto}"

install_to() {
  local dir="$1"
  local app_name="$2"
  echo "==> Installing gitvault skill for ${app_name} to ${dir}..."
  mkdir -p "${dir}"
  if [[ -f "skills/gitvault/SKILL.md" ]]; then
    cp "skills/gitvault/SKILL.md" "${dir}/SKILL.md"
  else
    curl -fsSL "${SKILL_URL}" -o "${dir}/SKILL.md"
  fi
  echo "    ✓ Installed to ${dir}/SKILL.md"
}

installed_any=0

if [[ "$TARGET" == "hermes" || "$TARGET" == "auto" ]]; then
  if [[ "$TARGET" == "hermes" || -d "$HOME/.hermes" ]]; then
    install_to "$HOME/.hermes/skills/gitvault" "Hermes Agent"
    installed_any=1
  fi
fi

if [[ "$TARGET" == "claude" || "$TARGET" == "auto" ]]; then
  if [[ "$TARGET" == "claude" || -d "$HOME/.claude" ]]; then
    install_to "$HOME/.claude/skills/gitvault" "Claude Code"
    installed_any=1
  fi
fi

# If neither directory existed and target was auto, default install to both standard directories
if [[ $installed_any -eq 0 && "$TARGET" == "auto" ]]; then
  install_to "$HOME/.hermes/skills/gitvault" "Hermes Agent (default path)"
  install_to "$HOME/.claude/skills/gitvault" "Claude Code (default path)"
elif [[ "$TARGET" != "auto" && "$TARGET" != "hermes" && "$TARGET" != "claude" ]]; then
  # Custom path specified
  install_to "$TARGET" "Custom Path"
fi

echo ""
echo "✨ gitvault Agent Skill installation complete!"
echo "You can now prompt your agent:"
echo "  - 'Back up ~/.homebridge to gitvault under name homebridge'"
echo "  - 'Restore n8n workflows from gitvault'"
echo "  - 'List my backups in gitvault'"
