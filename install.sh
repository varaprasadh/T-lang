#!/bin/bash

# Telugu Language Compiler Installation Script

echo "Telugu Language Compiler v1.0 - Installation"
echo "============================================="

# Build the project
echo "Building Telugu Language Compiler..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed! Please check your Rust installation."
    exit 1
fi

# Create bin directory in user's home if it doesn't exist
mkdir -p "$HOME/bin"

# Copy the binary
cp target/release/telc "$HOME/bin/telc"

if [ $? -eq 0 ]; then
    echo "✅ Telugu Language Compiler installed successfully!"
    echo ""
    echo "Installation Details:"
    echo "- Binary installed to: $HOME/bin/telc"
    echo "- Make sure $HOME/bin is in your PATH"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc if not already present:"
    echo "export PATH=\"\$HOME/bin:\$PATH\""
    echo ""
    echo "Usage Examples:"
    echo "  telc examples/hello.tel"
    echo "  telc examples/calculator.tel --verbose"
    echo "  telc --help"
    echo ""
    echo "Sample programs available in the 'examples/' directory:"
    echo "- hello.tel      : Simple hello world"
    echo "- calculator.tel : Basic arithmetic operations"
    echo "- loops.tel      : Loop demonstrations"
    echo "- conditions.tel : Conditional statements"
else
    echo "❌ Installation failed!"
    exit 1
fi