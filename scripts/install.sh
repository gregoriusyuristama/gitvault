#!/usr/bin/env bash
# gitvault installer: downloads the appropriate release binary and installs to /usr/local/bin.
# Usage: curl -fsSL https://raw.githubusercontent.com/<OWNER>/gitvault/main/scripts/install.sh | bash
set -euo pipefail

OWNER="${GITVAULT_OWNER:-gregoriusyuristama}"
REPO="gitvault"
INSTALL_DIR="${GITVAULT_INSTALL_DIR:-/usr/local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)   os_target="unknown-linux-gnu" ;;
  Darwin)  os_target="apple-darwin" ;;
  *) echo "unsupported OS: $os" >&2; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64) arch_target="x86_64" ;;
  arm64|aarch64) arch_target="aarch64" ;;
  *) echo "unsupported arch: $arch" >&2; exit 1 ;;
esac

target="${arch_target}-${os_target}"

echo "Fetching latest gitvault release for ${target}..."
latest_url="https://api.github.com/repos/${OWNER}/${REPO}/releases/latest"
version="$(curl -fsSL "$latest_url" | grep -oE '"tag_name":\s*"[^"]+"' | head -1 | cut -d'"' -f4)"
if [[ -z "$version" ]]; then
  echo "failed to resolve latest version" >&2
  exit 1
fi

tarball="gitvault-${version}-${target}.tar.gz"
download_url="https://github.com/${OWNER}/${REPO}/releases/download/${version}/${tarball}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Downloading ${download_url}..."
curl -fsSL -o "${tmp}/${tarball}" "$download_url"
tar -xzf "${tmp}/${tarball}" -C "$tmp"

binary="${tmp}/gitvault-${version}-${target}/gitvault"
chmod +x "$binary"

if [[ -w "$INSTALL_DIR" ]]; then
  mv "$binary" "${INSTALL_DIR}/gitvault"
else
  sudo mv "$binary" "${INSTALL_DIR}/gitvault"
fi

echo "gitvault ${version} installed to ${INSTALL_DIR}/gitvault"
gitvault --version
