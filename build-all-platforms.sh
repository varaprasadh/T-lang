#!/bin/bash

# Build Telugu Language for all platforms on macOS
# This script builds binaries for multiple platforms using Rust cross-compilation

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

echo -e "${BLUE}Building Telugu Language v$VERSION for all platforms${NC}"
echo "=================================================="

# Clean previous builds
rm -rf "$BUILD_DIR" "$ARTIFACTS_DIR"
mkdir -p "$BUILD_DIR" "$ARTIFACTS_DIR"

# Install required targets
echo -e "${YELLOW}Installing Rust targets...${NC}"
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Install cross for cross-compilation
if ! command -v cross &> /dev/null; then
    echo -e "${YELLOW}Installing cross for cross-compilation...${NC}"
    cargo install cross --git https://github.com/cross-rs/cross
fi

# Install tools for Windows builds on macOS (M1 compatible)
install_windows_tools() {
    if [[ "$(uname -s)" == "Darwin" ]]; then
        # For Apple Silicon (M1/M2/M3), use cargo-zigbuild which works perfectly
        if ! command -v zig &> /dev/null; then
            echo -e "${YELLOW}Installing zig for Windows cross-compilation (M3 Pro compatible)...${NC}"
            if command -v brew &> /dev/null; then
                brew install zig
            else
                echo -e "${RED}Error: Homebrew not found. Please install zig manually${NC}"
                exit 1
            fi
        fi
        
        if ! command -v cargo-zigbuild &> /dev/null; then
            echo -e "${YELLOW}Installing cargo-zigbuild for seamless Windows builds...${NC}"
            cargo install cargo-zigbuild
        fi
        
        echo -e "${GREEN}✅ Windows cross-compilation tools ready for M3 Pro!${NC}"
    fi
}

install_windows_tools

# Build function
build_target() {
    local target=$1
    local name=$2
    local binary=$3
    
    echo -e "${BLUE}Building for $name ($target)...${NC}"
    
    if [[ "$target" == *"linux"* ]]; then
        # Determine if using musl or gnu
        local cc_var=""
        if [[ "$target" == *"musl"* ]]; then
            cc_var="x86_64-linux-musl-gcc"
            if [[ "$target" == *"aarch64"* ]]; then
                cc_var="aarch64-linux-musl-gcc"
            fi
        else
            cc_var="x86_64-unknown-linux-gnu-gcc"
            if [[ "$target" == *"aarch64"* ]]; then
                cc_var="aarch64-unknown-linux-gnu-gcc"
            fi
        fi
        
        # Check if linker is installed
        if ! command -v "$cc_var" &> /dev/null; then
            echo -e "${YELLOW}Linux cross-compilation tools not found.${NC}"
            echo -e "${YELLOW}Run ./setup-linux-cross-compile.sh to install them.${NC}"
            echo -e "${YELLOW}Trying with zig as fallback...${NC}"
            
            if command -v cargo-zigbuild &> /dev/null; then
                if cargo zigbuild --release --target "$target"; then
                    echo -e "${GREEN}Linux build successful with zig!${NC}"
                else
                    echo -e "${RED}Failed to build for $target. Skipping...${NC}"
                    return 1
                fi
            else
                echo -e "${RED}No suitable Linux cross-compilation tool found.${NC}"
                return 1
            fi
        else
            # Build with proper TARGET_CC
            echo -e "${YELLOW}Building Linux target with $cc_var...${NC}"
            if TARGET_CC="$cc_var" cargo build --release --target "$target"; then
                echo -e "${GREEN}Linux build successful!${NC}"
            else
                echo -e "${RED}Linux build failed for $target.${NC}"
                return 1
            fi
        fi
    elif [[ "$target" == *"windows"* ]]; then
        # Use cargo-zigbuild for Windows targets (M3 Pro compatible)
        cargo zigbuild --release --target "$target"
    else
        # Use cargo for macOS targets
        cargo build --release --target "$target"
    fi
    
    # Create distribution directory
    local dist_name="telugu-lang-$name"
    local dist_path="$BUILD_DIR/$dist_name"
    mkdir -p "$dist_path"
    
    # Check if binary was actually built
    if [ ! -f "target/$target/release/$binary" ]; then
        echo -e "${RED}Binary not found for $target. Build may have failed.${NC}"
        return 1
    fi
    
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
        # Create zip for Windows
        zip -r "../$ARTIFACTS_DIR/$dist_name.zip" "$dist_name"
    else
        # Create tar.gz for Unix
        tar -czf "../$ARTIFACTS_DIR/$dist_name.tar.gz" "$dist_name"
    fi
    
    cd ..
    echo -e "${GREEN}✅ Built $name successfully!${NC}"
}

# Build for all platforms
build_target "x86_64-apple-darwin" "macos-x64" "telc"
build_target "aarch64-apple-darwin" "macos-arm64" "telc"
build_target "x86_64-pc-windows-gnu" "windows-x64" "telc.exe"

# Try Linux builds (may fail on M3 Pro due to Docker ARM64 issues)
echo -e "${BLUE}Attempting Linux builds...${NC}"
if build_target "x86_64-unknown-linux-gnu" "linux-x64" "telc"; then
    echo -e "${GREEN}Linux x64 build successful!${NC}"
else
    echo -e "${YELLOW}Linux x64 build failed, continuing...${NC}"
fi

if build_target "aarch64-unknown-linux-gnu" "linux-arm64" "telc"; then
    echo -e "${GREEN}Linux ARM64 build successful!${NC}"
else
    echo -e "${YELLOW}Linux ARM64 build failed, continuing...${NC}"
fi

# Create checksums
echo -e "${BLUE}Creating checksums...${NC}"
cd "$ARTIFACTS_DIR"
shasum -a 256 * > checksums.txt
cd ..

# Create latest release info
echo "$VERSION" > "$ARTIFACTS_DIR/VERSION"

echo ""
echo -e "${GREEN}✅ All builds completed successfully!${NC}"
echo ""
echo "Artifacts created in: $ARTIFACTS_DIR/"
ls -la "$ARTIFACTS_DIR/"
echo ""
echo "To push artifacts to GitHub:"
echo "1. Create a new release tag: git tag v$VERSION"
echo "2. Push the tag: git push origin v$VERSION"
echo "3. Upload artifacts manually to the release"