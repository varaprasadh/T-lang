#!/bin/bash

# Telugu Language Compiler - Linux Installation Script
# Supports multiple package managers and distributions

set -e

REPO_URL="https://github.com/YOUR_USERNAME/telugu-lang"
VERSION="0.1.0"
INSTALL_DIR="/usr/local"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$NAME
        VER=$VERSION_ID
    elif type lsb_release >/dev/null 2>&1; then
        OS=$(lsb_release -si)
        VER=$(lsb_release -sr)
    elif [ -f /etc/lsb-release ]; then
        . /etc/lsb-release
        OS=$DISTRIB_ID
        VER=$DISTRIB_RELEASE
    else
        OS=$(uname -s)
        VER=$(uname -r)
    fi
    
    # Convert to lowercase
    OS_LOWER=$(echo "$OS" | tr '[:upper:]' '[:lower:]')
}

# Install Rust if not present
install_rust() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

# Install from source
install_from_source() {
    echo -e "${GREEN}Installing Telugu Language Compiler from source...${NC}"
    
    # Create temporary directory
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"
    
    # Download source
    echo "Downloading source code..."
    curl -L "${REPO_URL}/archive/refs/tags/v${VERSION}.tar.gz" -o telugu-lang.tar.gz
    tar -xzf telugu-lang.tar.gz
    cd "telugu-lang-${VERSION}"
    
    # Build
    echo "Building Telugu Language Compiler..."
    cargo build --release
    
    # Install binary
    sudo install -m 755 target/release/telc "$INSTALL_DIR/bin/telc"
    
    # Install examples and documentation
    sudo mkdir -p "$INSTALL_DIR/share/telugu-lang/examples"
    sudo cp -r examples/* "$INSTALL_DIR/share/telugu-lang/examples/"
    sudo install -m 644 README.md "$INSTALL_DIR/share/doc/telugu-lang/"
    
    # Cleanup
    cd /
    rm -rf "$TMP_DIR"
    
    echo -e "${GREEN}✅ Installation complete!${NC}"
}

# Package manager specific installations
install_debian() {
    echo -e "${GREEN}Installing for Debian/Ubuntu...${NC}"
    
    # Install dependencies
    sudo apt-get update
    sudo apt-get install -y curl build-essential
    
    install_rust
    install_from_source
}

install_fedora() {
    echo -e "${GREEN}Installing for Fedora/RHEL...${NC}"
    
    # Install dependencies
    sudo dnf install -y curl gcc make
    
    install_rust
    install_from_source
}

install_arch() {
    echo -e "${GREEN}Installing for Arch Linux...${NC}"
    
    # Install dependencies
    sudo pacman -Sy --noconfirm curl base-devel
    
    install_rust
    install_from_source
}

install_suse() {
    echo -e "${GREEN}Installing for openSUSE...${NC}"
    
    # Install dependencies
    sudo zypper install -y curl gcc make
    
    install_rust
    install_from_source
}

# Main installation logic
main() {
    echo "Telugu Language Compiler - Linux Installation"
    echo "============================================="
    
    detect_distro
    echo "Detected: $OS $VER"
    
    case "$OS_LOWER" in
        *ubuntu*|*debian*|*mint*)
            install_debian
            ;;
        *fedora*|*rhel*|*centos*|*alma*|*rocky*)
            install_fedora
            ;;
        *arch*|*manjaro*)
            install_arch
            ;;
        *suse*|*opensuse*)
            install_suse
            ;;
        *)
            echo -e "${YELLOW}Distribution not directly supported. Attempting generic installation...${NC}"
            install_rust
            install_from_source
            ;;
    esac
    
    # Verify installation
    if command -v telc &> /dev/null; then
        echo -e "${GREEN}✅ Telugu Language Compiler installed successfully!${NC}"
        echo ""
        echo "Version: $(telc --version)"
        echo "Location: $(which telc)"
        echo ""
        echo "Usage Examples:"
        echo "  telc /usr/local/share/telugu-lang/examples/hello.tel"
        echo "  telc --help"
    else
        echo -e "${RED}❌ Installation verification failed!${NC}"
        exit 1
    fi
}

# Run main function
main