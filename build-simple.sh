#!/bin/bash

# Simple build script for Telugu Language on macOS
# Builds only for the current platform initially

set -e

VERSION="0.1.0"
BUILD_DIR="dist"
ARTIFACTS_DIR="artifacts"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Building Telugu Language v$VERSION${NC}"
echo "======================================"

# Clean previous builds
rm -rf "$BUILD_DIR" "$ARTIFACTS_DIR"
mkdir -p "$BUILD_DIR" "$ARTIFACTS_DIR"

# Detect current platform
detect_platform() {
    local os=""
    local arch=""
    
    case "$(uname -s)" in
        Darwin*) os="macos" ;;
        Linux*) os="linux" ;;
        *) echo -e "${RED}Unsupported OS: $(uname -s)${NC}"; exit 1 ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="x64" ;;
        arm64|aarch64) arch="arm64" ;;
        *) echo -e "${RED}Unsupported architecture: $(uname -m)${NC}"; exit 1 ;;
    esac
    
    echo "${os}-${arch}"
}

# Build for current platform
build_current() {
    local platform=$(detect_platform)
    local binary="telc"
    
    echo -e "${BLUE}Building for current platform: $platform${NC}"
    
    # Build
    cargo build --release
    
    # Create distribution directory
    local dist_name="telugu-lang-$platform"
    local dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    # Copy files
    cp "target/release/$binary" "$dist_path/"
    cp -r examples "$dist_path/"
    cp README.md "$dist_path/"
    cp INSTALLATION.md "$dist_path/" 2>/dev/null || true
    
    # Create archive
    echo -e "${YELLOW}Creating archive for $platform...${NC}"
    cd "$BUILD_DIR"
    tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    cd ..
    
    echo -e "${GREEN}✅ Built for $platform successfully!${NC}"
}

# Build macOS targets if on macOS
build_macos_targets() {
    if [[ "$(uname -s)" != "Darwin" ]]; then
        echo -e "${YELLOW}Skipping macOS builds (not on macOS)${NC}"
        return
    fi
    
    echo -e "${BLUE}Building macOS targets...${NC}"
    
    # Install targets
    rustup target add x86_64-apple-darwin
    rustup target add aarch64-apple-darwin
    
    # Build Intel
    echo -e "${YELLOW}Building macOS Intel...${NC}"
    cargo build --release --target x86_64-apple-darwin
    
    local dist_name="telugu-lang-macos-x64"
    local dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    cp "target/x86_64-apple-darwin/release/telc" "$dist_path/"
    cp -r examples "$dist_path/"
    cp README.md "$dist_path/"
    cp INSTALLATION.md "$dist_path/" 2>/dev/null || true
    
    cd "$BUILD_DIR"
    tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    cd ..
    
    # Build ARM64
    echo -e "${YELLOW}Building macOS ARM64...${NC}"
    cargo build --release --target aarch64-apple-darwin
    
    dist_name="telugu-lang-macos-arm64"
    dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    cp "target/aarch64-apple-darwin/release/telc" "$dist_path/"
    cp -r examples "$dist_path/"
    cp README.md "$dist_path/"
    cp INSTALLATION.md "$dist_path/" 2>/dev/null || true
    
    cd "$BUILD_DIR"
    tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    cd ..
    
    echo -e "${GREEN}✅ Built macOS targets successfully!${NC}"
}

# Main build process
echo -e "${BLUE}Starting build process...${NC}"

# Build for current platform
build_current

# Build additional macOS targets if on macOS
build_macos_targets

# Create checksums
echo -e "${BLUE}Creating checksums...${NC}"
cd "$ARTIFACTS_DIR"
shasum -a 256 * > checksums.txt
cd ..

# Create version file
echo "$VERSION" > "$ARTIFACTS_DIR/VERSION"

echo ""
echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo ""
echo "Artifacts created in: $ARTIFACTS_DIR/"
ls -la "$ARTIFACTS_DIR/"
echo ""
echo "To build for all platforms, install the required tools:"
echo "  brew install mingw-w64          # For Windows"
echo "  cargo install cross             # For Linux"
echo "Then run: ./build-all-platforms.sh"