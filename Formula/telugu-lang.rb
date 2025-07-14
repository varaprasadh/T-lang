class TeluguLang < Formula
  desc "Telugu Programming Language Compiler"
  homepage "https://github.com/varaprasadh/T-lang"
  url "https://github.com/varaprasadh/T-lang/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--locked"
    bin.install "target/release/telc"
    
    # Install examples
    (share/"telugu-lang/examples").install Dir["examples/*"]
    
    # Install documentation
    doc.install "README.md"
  end

  test do
    # Create a simple test program
    (testpath/"test.tel").write <<~EOS
      ప్రింట్("హలో ప్రపంచం!");
    EOS
    
    # Run the compiler
    system "#{bin}/telc", "test.tel"
  end
end