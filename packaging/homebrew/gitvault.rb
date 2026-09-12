# Homebrew formula template for gitvault.
# Placed in a companion tap repo: <user>/homebrew-tap/Formula/gitvault.rb
# Replace <OWNER>, <VERSION>, and the sha256 values on each release.
#
# Install:
#   brew tap <OWNER>/tap
#   brew install gitvault
class Gitvault < Formula
  desc "Encrypted Git-backed directory backup CLI"
  homepage "https://github.com/<OWNER>/gitvault"
  version "<VERSION>"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/<OWNER>/gitvault/releases/download/v#{version}/gitvault-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "<SHA256_MAC_ARM64>"
    else
      url "https://github.com/<OWNER>/gitvault/releases/download/v#{version}/gitvault-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "<SHA256_MAC_X86_64>"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/<OWNER>/gitvault/releases/download/v#{version}/gitvault-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "<SHA256_LINUX_ARM64>"
    else
      url "https://github.com/<OWNER>/gitvault/releases/download/v#{version}/gitvault-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "<SHA256_LINUX_X86_64>"
    end
  end

  depends_on "git"

  def install
    bin.install "gitvault"
  end

  test do
    assert_match "gitvault", shell_output("#{bin}/gitvault --version")
  end
end
