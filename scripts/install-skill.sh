#!/usr/bin/env bash
# install-skill.sh: Installs gitvault skill into active Hermes or Claude profile
set -euo pipefail

SKILL_DIR="${1:-$HOME/.hermes/skills/gitvault}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Installing gitvault skill to ${SKILL_DIR}..."
mkdir -p "${SKILL_DIR}"
cp "${REPO_ROOT}/skills/gitvault/SKILL.md" "${SKILL_DIR}/SKILL.md"

echo "Skill installed successfully!"
echo "Verify with: hermes skills list | grep gitvault"
