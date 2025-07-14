#!/bin/bash

# Universal Telugu Language Installer
# Works on macOS, Linux, and Windows (Git Bash/WSL)

set -e

# Configuration
REPO_URL="https://github.com/varaprasadh/T-lang"
REPO_RAW_URL="https://raw.githubusercontent.com/varaprasadh/T-lang/main"
VERSION="latest"
INSTALL_DIR="${TELC_INSTALL_DIR:-$HOME/.telugu-lang}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Detect OS and Architecture
detect_platform() {
    local os=""
    local arch=""
    
    # Detect OS
    case "$(uname -s)" in
        Darwin*)    os="macos" ;;
        Linux*)     os="linux" ;;
        MINGW*|MSYS*|CYGWIN*)  os="windows" ;;
        *)          echo -e "${RED}Unsupported OS: $(uname -s)${NC}"; exit 1 ;;
    esac
    
    # Detect Architecture
    case "$(uname -m)" in
        x86_64|amd64)  arch="x64" ;;
        arm64|aarch64) arch="arm64" ;;
        *)             echo -e "${RED}Unsupported architecture: $(uname -m)${NC}"; exit 1 ;;
    esac
    
    echo "${os}-${arch}"
}

# Download file with curl or wget
download() {
    local url=$1
    local output=$2
    
    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget &> /dev/null; then
        wget -q "$url" -O "$output"
    else
        echo -e "${RED}Error: Neither curl nor wget found${NC}"
        exit 1
    fi
}

# Get latest version
get_latest_version() {
    local version_url="$REPO_RAW_URL/artifacts/VERSION"
    local version_file="/tmp/telugu-lang-version"
    
    if download "$version_url" "$version_file" 2>/dev/null; then
        cat "$version_file"
        rm -f "$version_file"
    else
        echo "0.1.0"  # Fallback version
    fi
}

# Install Telugu Language
install_telugu_lang() {
    echo -e "${BLUE}Telugu Language Compiler - Universal Installer${NC}"
    echo "=============================================="
    
    # Detect platform
    local platform=$(detect_platform)
    echo -e "Detected platform: ${GREEN}$platform${NC}"
    
    # Get version
    if [ "$VERSION" = "latest" ]; then
        VERSION=$(get_latest_version)
    fi
    echo -e "Installing version: ${GREEN}$VERSION${NC}"
    
    # Set binary name
    local binary_name="telc"
    local archive_ext="tar.gz"
    if [[ "$platform" == "windows"* ]]; then
        binary_name="telc.exe"
        archive_ext="zip"
    fi
    
    # Create temp directory
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    # Download artifact
    local artifact_name="telugu-lang-${platform}.${archive_ext}"
    local artifact_url="$REPO_RAW_URL/artifacts/$artifact_name"
    
    echo -e "${YELLOW}Downloading Telugu Language...${NC}"
    if ! download "$artifact_url" "$artifact_name"; then
        echo -e "${RED}Failed to download from artifacts. Trying GitHub releases...${NC}"
        artifact_url="$REPO_URL/releases/download/v$VERSION/$artifact_name"
        if ! download "$artifact_url" "$artifact_name"; then
            echo -e "${RED}Failed to download Telugu Language${NC}"
            exit 1
        fi
    fi
    
    # Extract archive
    echo -e "${YELLOW}Extracting...${NC}"
    if [[ "$archive_ext" == "zip" ]]; then
        unzip -q "$artifact_name"
    else
        tar -xzf "$artifact_name"
    fi
    
    # Find extracted directory
    local extract_dir=$(find . -name "telugu-lang-*" -type d | head -1)
    
    # Create installation directory
    mkdir -p "$INSTALL_DIR/bin"
    mkdir -p "$INSTALL_DIR/examples"
    mkdir -p "$INSTALL_DIR/doc"
    
    # Copy files
    echo -e "${YELLOW}Installing files...${NC}"
    cp "$extract_dir/$binary_name" "$INSTALL_DIR/bin/"
    chmod +x "$INSTALL_DIR/bin/$binary_name"
    
    if [ -d "$extract_dir/examples" ]; then
        cp -r "$extract_dir/examples"/* "$INSTALL_DIR/examples/"
    fi
    
    if [ -f "$extract_dir/README.md" ]; then
        cp "$extract_dir/README.md" "$INSTALL_DIR/doc/"
    fi
    
    # Clean up
    cd /
    rm -rf "$temp_dir"
    
    # Add to PATH
    add_to_path
    
    echo ""
    echo -e "${GREEN}✅ Telugu Language installed successfully!${NC}"
    echo ""
    echo "Installation location: $INSTALL_DIR"
    echo ""
    echo "To use Telugu Language, add this to your shell profile:"
    echo -e "${BLUE}export PATH=\"\$HOME/.telugu-lang/bin:\$PATH\"${NC}"
    echo ""
    echo "Then reload your shell or run:"
    echo -e "${BLUE}source ~/.bashrc${NC} (or ~/.zshrc)"
    echo ""
    echo "Usage examples:"
    echo "  telc $INSTALL_DIR/examples/hello.tel"
    echo "  telc --help"
}

# Add to PATH in shell profiles
add_to_path() {
    local path_line="export PATH=\"\$HOME/.telugu-lang/bin:\$PATH\""
    local added=false
    
    # Add to various shell profiles
    for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.profile"; do
        if [ -f "$profile" ]; then
            if ! grep -q ".telugu-lang/bin" "$profile"; then
                echo "" >> "$profile"
                echo "# Telugu Language" >> "$profile"
                echo "$path_line" >> "$profile"
                added=true
                echo -e "${GREEN}Added to $profile${NC}"
            fi
        fi
    done
    
    if [ "$added" = false ]; then
        echo -e "${YELLOW}Could not add to PATH automatically${NC}"
    fi
}

# Uninstall function
uninstall() {
    echo -e "${YELLOW}Uninstalling Telugu Language...${NC}"
    
    # Remove installation directory
    if [ -d "$INSTALL_DIR" ]; then
        rm -rf "$INSTALL_DIR"
        echo -e "${GREEN}Removed $INSTALL_DIR${NC}"
    fi
    
    # Remove from PATH in shell profiles
    for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.profile"; do
        if [ -f "$profile" ]; then
            # Remove Telugu Language lines
            sed -i.bak '/# Telugu Language/,+1d' "$profile" 2>/dev/null || \
            sed -i '' '/# Telugu Language/,+1d' "$profile" 2>/dev/null || true
            
            # Remove PATH line
            sed -i.bak '/\.telugu-lang\/bin/d' "$profile" 2>/dev/null || \
            sed -i '' '/\.telugu-lang\/bin/d' "$profile" 2>/dev/null || true
        fi
    done
    
    echo -e "${GREEN}✅ Telugu Language uninstalled successfully!${NC}"
}

# Parse arguments
case "${1:-}" in
    --uninstall|-u)
        uninstall
        ;;
    --version|-v)
        VERSION="$2"
        install_telugu_lang
        ;;
    --help|-h)
        echo "Telugu Language Universal Installer"
        echo ""
        echo "Usage:"
        echo "  curl -fsSL $REPO_RAW_URL/install.sh | bash        # Install latest"
        echo "  curl -fsSL $REPO_RAW_URL/install.sh | bash -s -- --version 0.1.0  # Specific version"
        echo "  curl -fsSL $REPO_RAW_URL/install.sh | bash -s -- --uninstall     # Uninstall"
        echo ""
        echo "Options:"
        echo "  --version, -v VERSION    Install specific version"
        echo "  --uninstall, -u         Uninstall Telugu Language"
        echo "  --help, -h              Show this help"
        ;;
    *)
        install_telugu_lang
        ;;
esac