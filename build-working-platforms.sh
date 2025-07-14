#!/bin/bash

# Build Telugu Language for platforms that work reliably on M3 Pro
# Focuses on macOS and Windows (which work great with zig)

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

echo -e "${BLUE}Building Telugu Language v$VERSION for M3 Pro${NC}"
echo "============================================="

# Clean previous builds
rm -rf "$BUILD_DIR" "$ARTIFACTS_DIR"
mkdir -p "$BUILD_DIR" "$ARTIFACTS_DIR"

# Install required tools
echo -e "${YELLOW}Checking required tools...${NC}"
if ! command -v zig &> /dev/null; then
    echo -e "${YELLOW}Installing zig...${NC}"
    brew install zig
fi

if ! command -v cargo-zigbuild &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-zigbuild...${NC}"
    cargo install cargo-zigbuild
fi

# Install targets
echo -e "${YELLOW}Installing Rust targets...${NC}"
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Build function
build_target() {
    local target=$1
    local name=$2
    local binary=$3
    
    echo -e "${BLUE}Building for $name ($target)...${NC}"
    
    if [[ "$target" == *"windows"* ]]; then
        cargo zigbuild --release --target "$target"
    else
        cargo build --release --target "$target"
    fi
    
    # Create distribution directory
    local dist_name="telugu-lang-$name"
    local dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    # Copy binary
    cp "target/$target/release/$binary" "$dist_path/"
    
    # Copy examples and docs
    cp -r examples "$dist_path/"
    cp README.md "$dist_path/"
    cp INSTALLATION.md "$dist_path/" 2>/dev/null || true
    
    # Create archive
    echo -e "${YELLOW}Creating archive for $name...${NC}"
    cd "$BUILD_DIR"
    
    if [[ "$name" == *"windows"* ]]; then
        zip -r "../$ARTIFACTS_DIR/$dist_name.zip" "$dist_name"
    else
        tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    fi
    
    cd ..
    echo -e "${GREEN}✅ Built $name successfully!${NC}"
}

# Build for reliable platforms
echo -e "${BLUE}Building for reliable platforms...${NC}"
build_target "x86_64-apple-darwin" "macos-x64" "telc"
build_target "aarch64-apple-darwin" "macos-arm64" "telc"
build_target "x86_64-pc-windows-gnu" "windows-x64" "telc.exe"

# Try Linux with zig as fallback
echo -e "${BLUE}Attempting Linux builds with zig...${NC}"
rustup target add x86_64-unknown-linux-gnu

if cargo zigbuild --release --target x86_64-unknown-linux-gnu; then
    echo -e "${GREEN}Linux build successful with zig!${NC}"
    
    # Create Linux distribution
    local dist_name="telugu-lang-linux-x64"
    local dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    cp "target/x86_64-unknown-linux-gnu/release/telc" "$dist_path/"
    cp -r examples "$dist_path/"
    cp README.md "$dist_path/"
    cp INSTALLATION.md "$dist_path/" 2>/dev/null || true
    
    cd "$BUILD_DIR"
    tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    cd ..
    
    echo -e "${GREEN}✅ Built Linux x64 successfully!${NC}"
else
    echo -e "${YELLOW}Linux build failed, but that's okay - we have the main platforms!${NC}"
fi

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
echo -e "${GREEN}Ready to commit and push:${NC}"
echo "  git add artifacts/"
echo "  git commit -m \"Add pre-built binaries\""
echo "  git push"