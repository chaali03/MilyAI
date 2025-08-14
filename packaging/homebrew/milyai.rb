class Milyai < Formula
  desc "Modular AI assistant CLI"
  homepage "https://example.com/milyai"
  version "0.1.0"
  # TODO: replace with real tarball and sha256
  url "https://example.com/milyai/releases/0.1.0/milyai-macos-universal.tar.gz"
  sha256 "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

  def install
    bin.install "milyai"
  end

  test do
    assert_match "MilyAI", shell_output("#{bin}/milyai --help")
  end
end 