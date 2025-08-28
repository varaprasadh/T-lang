#!/bin/bash

# Telugu Language VS Code Extension Installation Script

echo "Telugu Language VS Code Extension Installation"
echo "=============================================="

# Detect OS and set extensions directory
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    VSCODE_EXTENSIONS_DIR="$HOME/.vscode/extensions"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    VSCODE_EXTENSIONS_DIR="$HOME/.vscode/extensions"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    # Windows
    VSCODE_EXTENSIONS_DIR="$USERPROFILE/.vscode/extensions"
else
    echo "❌ Unsupported operating system: $OSTYPE"
    exit 1
fi

echo "Operating System: $OSTYPE"
echo "VS Code Extensions Directory: $VSCODE_EXTENSIONS_DIR"

# Check if VS Code extensions directory exists
if [ ! -d "$VSCODE_EXTENSIONS_DIR" ]; then
    echo "Creating VS Code extensions directory..."
    mkdir -p "$VSCODE_EXTENSIONS_DIR"
fi

# Extension directory name
EXTENSION_NAME="telugu-lang-1.0.0"
EXTENSION_DIR="$VSCODE_EXTENSIONS_DIR/$EXTENSION_NAME"

# Remove existing extension if it exists
if [ -d "$EXTENSION_DIR" ]; then
    echo "Removing existing Telugu Language extension..."
    rm -rf "$EXTENSION_DIR"
fi

# Copy extension files
echo "Installing Telugu Language extension..."
cp -r "../vscode-telugu-lang" "$EXTENSION_DIR"

if [ $? -eq 0 ]; then
    echo "✅ Telugu Language VS Code Extension installed successfully!"
    echo ""
    echo "Installation Details:"
    echo "- Extension installed to: $EXTENSION_DIR"
    echo "- Extension version: 1.0.0"
    echo ""
    echo "Next Steps:"
    echo "1. Restart VS Code"
    echo "2. Open any .tel file to see syntax highlighting"
    echo "3. Try code snippets by typing:"
    echo "   - 'sankhya' + Tab"
    echo "   - 'cheppu' + Tab"
    echo "   - 'ayite' + Tab"
    echo "   - 'varaku' + Tab"
    echo ""
    echo "Test Files:"
    echo "- Open ../vscode-telugu-lang/test.tel to see syntax highlighting"
    echo "- Open examples/*.tel files to test with real programs"
    echo ""
    echo "Troubleshooting:"
    echo "- If syntax highlighting doesn't work, make sure the file has .tel extension"
    echo "- If VS Code doesn't recognize the extension, try reloading the window (Ctrl+R)"
else
    echo "❌ Installation failed!"
    echo "Please check permissions and try again"
    exit 1
fi