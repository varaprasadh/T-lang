#!/bin/bash

# Telugu Version Manager (TelVM) - Installation Script
# Similar to SDKMAN for Java

TELVM_VERSION="0.1.0"
TELVM_DIR="${TELVM_DIR:-$HOME/.telvm}"
TELVM_REPO="https://github.com/YOUR_USERNAME/telugu-lang"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo -e "${BLUE}Telugu Version Manager (TelVM) v${TELVM_VERSION}${NC}"
echo "============================================="
echo ""

# Create TelVM directory structure
mkdir -p "$TELVM_DIR"/{bin,versions,tmp,etc}

# Create TelVM main script
cat > "$TELVM_DIR/bin/telvm" << 'EOF'
#!/bin/bash

TELVM_DIR="${TELVM_DIR:-$HOME/.telvm}"
TELVM_CURRENT="$TELVM_DIR/current"
TELVM_VERSIONS="$TELVM_DIR/versions"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Help function
show_help() {
    echo "Telugu Version Manager (TelVM)"
    echo ""
    echo "Usage: telvm <command> [version]"
    echo ""
    echo "Commands:"
    echo "  install <version>    Install a specific version of Telugu Lang"
    echo "  use <version>        Switch to a specific version"
    echo "  list                 List all installed versions"
    echo "  list-remote          List all available versions"
    echo "  current              Show current version in use"
    echo "  uninstall <version>  Uninstall a specific version"
    echo "  default <version>    Set default version"
    echo "  help                 Show this help message"
}

# List installed versions
list_versions() {
    echo -e "${BLUE}Installed Telugu Lang versions:${NC}"
    if [ -d "$TELVM_VERSIONS" ] && [ "$(ls -A "$TELVM_VERSIONS" 2>/dev/null)" ]; then
        for version in "$TELVM_VERSIONS"/*; do
            if [ -d "$version" ]; then
                version_name=$(basename "$version")
                if [ -L "$TELVM_CURRENT" ] && [ "$(readlink "$TELVM_CURRENT")" = "$version" ]; then
                    echo -e "  ${GREEN}* $version_name (current)${NC}"
                else
                    echo "    $version_name"
                fi
            fi
        done
    else
        echo "  No versions installed"
    fi
}

# List remote versions
list_remote() {
    echo -e "${BLUE}Available Telugu Lang versions:${NC}"
    # This would normally fetch from a version API
    # For now, showing example versions
    echo "  0.1.0 (latest)"
    echo "  0.0.9"
    echo "  0.0.8"
}

# Install a version
install_version() {
    local version=$1
    if [ -z "$version" ]; then
        echo -e "${RED}Error: Please specify a version${NC}"
        return 1
    fi
    
    local version_dir="$TELVM_VERSIONS/$version"
    
    if [ -d "$version_dir" ]; then
        echo -e "${YELLOW}Version $version is already installed${NC}"
        return 0
    fi
    
    echo -e "${BLUE}Installing Telugu Lang v$version...${NC}"
    
    mkdir -p "$version_dir"
    
    # Download and install (simplified for example)
    local temp_dir="$TELVM_DIR/tmp/install-$$"
    mkdir -p "$temp_dir"
    
    cd "$temp_dir" || return 1
    
    # Download binary based on OS
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    local binary_name="telugu-lang-${os}-${arch}.tar.gz"
    
    echo "Downloading $binary_name..."
    # curl -L "https://github.com/YOUR_USERNAME/telugu-lang/releases/download/v$version/$binary_name" -o "$binary_name"
    # tar -xzf "$binary_name" -C "$version_dir"
    
    # For now, just create a placeholder
    echo "#!/bin/bash" > "$version_dir/telc"
    echo "echo 'Telugu Lang v$version'" >> "$version_dir/telc"
    chmod +x "$version_dir/telc"
    
    # Cleanup
    rm -rf "$temp_dir"
    
    echo -e "${GREEN}✅ Telugu Lang v$version installed successfully!${NC}"
}

# Use a version
use_version() {
    local version=$1
    if [ -z "$version" ]; then
        echo -e "${RED}Error: Please specify a version${NC}"
        return 1
    fi
    
    local version_dir="$TELVM_VERSIONS/$version"
    
    if [ ! -d "$version_dir" ]; then
        echo -e "${RED}Error: Version $version is not installed${NC}"
        echo "Run 'telvm install $version' to install it"
        return 1
    fi
    
    # Update symlink
    rm -f "$TELVM_CURRENT"
    ln -s "$version_dir" "$TELVM_CURRENT"
    
    echo -e "${GREEN}Now using Telugu Lang v$version${NC}"
}

# Show current version
show_current() {
    if [ -L "$TELVM_CURRENT" ] && [ -d "$(readlink "$TELVM_CURRENT")" ]; then
        local current_version=$(basename "$(readlink "$TELVM_CURRENT")")
        echo -e "${GREEN}Current version: $current_version${NC}"
    else
        echo -e "${YELLOW}No version currently selected${NC}"
        echo "Run 'telvm use <version>' to select a version"
    fi
}

# Uninstall a version
uninstall_version() {
    local version=$1
    if [ -z "$version" ]; then
        echo -e "${RED}Error: Please specify a version${NC}"
        return 1
    fi
    
    local version_dir="$TELVM_VERSIONS/$version"
    
    if [ ! -d "$version_dir" ]; then
        echo -e "${RED}Error: Version $version is not installed${NC}"
        return 1
    fi
    
    # Check if it's the current version
    if [ -L "$TELVM_CURRENT" ] && [ "$(readlink "$TELVM_CURRENT")" = "$version_dir" ]; then
        echo -e "${YELLOW}Warning: Uninstalling the currently active version${NC}"
        rm -f "$TELVM_CURRENT"
    fi
    
    rm -rf "$version_dir"
    echo -e "${GREEN}Telugu Lang v$version uninstalled${NC}"
}

# Set default version
set_default() {
    local version=$1
    if [ -z "$version" ]; then
        echo -e "${RED}Error: Please specify a version${NC}"
        return 1
    fi
    
    use_version "$version"
    echo "$version" > "$TELVM_DIR/etc/default"
    echo -e "${GREEN}Default version set to $version${NC}"
}

# Main command handler
case "$1" in
    install)
        install_version "$2"
        ;;
    use)
        use_version "$2"
        ;;
    list|ls)
        list_versions
        ;;
    list-remote|lr)
        list_remote
        ;;
    current)
        show_current
        ;;
    uninstall|rm)
        uninstall_version "$2"
        ;;
    default)
        set_default "$2"
        ;;
    help|--help|-h|"")
        show_help
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        show_help
        exit 1
        ;;
esac
EOF

chmod +x "$TELVM_DIR/bin/telvm"

# Create shell initialization script
cat > "$TELVM_DIR/init.sh" << 'EOF'
#!/bin/bash

# Telugu Version Manager initialization
export TELVM_DIR="${TELVM_DIR:-$HOME/.telvm}"
export PATH="$TELVM_DIR/bin:$TELVM_DIR/current:$PATH"

# Auto-switch to default version if set
if [ -f "$TELVM_DIR/etc/default" ]; then
    default_version=$(cat "$TELVM_DIR/etc/default")
    if [ -d "$TELVM_DIR/versions/$default_version" ] && [ ! -L "$TELVM_DIR/current" ]; then
        telvm use "$default_version" > /dev/null 2>&1
    fi
fi
EOF

# Add initialization to shell profiles
add_to_profile() {
    local profile=$1
    local init_line='[ -s "$HOME/.telvm/init.sh" ] && source "$HOME/.telvm/init.sh"'
    
    if [ -f "$profile" ]; then
        if ! grep -q "telvm/init.sh" "$profile"; then
            echo "" >> "$profile"
            echo "# Telugu Version Manager" >> "$profile"
            echo "$init_line" >> "$profile"
            echo -e "${GREEN}✅ Added TelVM initialization to $profile${NC}"
        else
            echo -e "${YELLOW}TelVM already initialized in $profile${NC}"
        fi
    fi
}

# Add to common shell profiles
add_to_profile "$HOME/.bashrc"
add_to_profile "$HOME/.zshrc"
add_to_profile "$HOME/.bash_profile"

echo ""
echo -e "${GREEN}✅ Telugu Version Manager installed successfully!${NC}"
echo ""
echo "To get started:"
echo "1. Restart your shell or run:"
echo -e "   ${BLUE}source ~/.telvm/init.sh${NC}"
echo ""
echo "2. Install a version:"
echo -e "   ${BLUE}telvm install 0.1.0${NC}"
echo ""
echo "3. Use a version:"
echo -e "   ${BLUE}telvm use 0.1.0${NC}"
echo ""
echo "4. List available commands:"
echo -e "   ${BLUE}telvm help${NC}"
echo ""