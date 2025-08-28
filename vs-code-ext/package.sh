#!/bin/bash

# Package Telugu Language VS Code Extension

echo "Packaging Telugu Language VS Code Extension"
echo "==========================================="

# Check if vsce is installed
if ! command -v vsce &> /dev/null; then
    echo "Installing Visual Studio Code Extension Manager (vsce)..."
    npm install -g vsce
    
    if [ $? -ne 0 ]; then
        echo "❌ Failed to install vsce. Please install Node.js and npm first."
        echo "Visit: https://nodejs.org/"
        exit 1
    fi
fi

# Package the extension
echo "Creating .vsix package..."
vsce package

if [ $? -eq 0 ]; then
    echo "✅ Extension packaged successfully!"
    echo ""
    echo "Installation options:"
    echo "1. Install from file in VS Code:"
    echo "   - Open VS Code"
    echo "   - Press Ctrl+Shift+P (Cmd+Shift+P on Mac)"
    echo "   - Type 'Extensions: Install from VSIX'"
    echo "   - Select the .vsix file"
    echo ""
    echo "2. Install from command line:"
    echo "   code --install-extension telugu-lang-1.0.0.vsix"
    echo ""
    echo "3. Manual installation:"
    echo "   Run the install-vscode-extension.sh script"
else
    echo "❌ Packaging failed!"
    echo "Please check the package.json file and try again"
    exit 1
fi