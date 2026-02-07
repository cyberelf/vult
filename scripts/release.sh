#!/bin/bash
# Release build script for Vult
# Creates release packages for CLI and GUI binaries

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${GREEN}Building Vult v${VERSION}${NC}"

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux-x86_64"
    LIB_EXT="so"
    EXE_EXT=""
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows-x86_64"
    LIB_EXT="dll"
    EXE_EXT=".exe"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos-x86_64"
    LIB_EXT="dylib"
    EXE_EXT=""
else
    echo -e "${RED}Unknown platform: $OSTYPE${NC}"
    exit 1
fi

echo -e "${GREEN}Platform: ${PLATFORM}${NC}"

# Create dist directory
DIST_DIR="dist/vult-${VERSION}-${PLATFORM}"
mkdir -p "$DIST_DIR"

echo -e "${YELLOW}Running pre-build checks...${NC}"

# Run tests
echo "Running tests..."
cargo test --lib || { echo -e "${RED}Tests failed${NC}"; exit 1; }

# Run clippy
echo "Running clippy..."
cargo clippy --lib --features gui -- -D warnings || { echo -e "${RED}Clippy failed${NC}"; exit 1; }

# Run cargo audit if available
if command -v cargo-audit &> /dev/null; then
    echo "Running cargo audit..."
    cargo audit || echo -e "${YELLOW}Warning: cargo audit found issues${NC}"
fi

echo -e "${YELLOW}Building release binaries...${NC}"

# Build library
echo "Building library..."
cargo build --lib --release

# Build CLI
echo "Building CLI..."
cargo build --bin vult --features cli --release

# Build GUI with Tauri (generates installers)
echo "Building GUI with Tauri installers..."
npm install --prefix ui-sveltekit 2>/dev/null || true
cargo tauri build --features gui

echo -e "${YELLOW}Packaging release...${NC}"

# Copy binaries
cp "target/release/vult${EXE_EXT}" "$DIST_DIR/"
cp "target/release/vult-gui${EXE_EXT}" "$DIST_DIR/"

# Copy library (optional)
cp "target/release/libvult.${LIB_EXT}" "$DIST_DIR/" 2>/dev/null || true

# Copy installers based on platform
echo "Copying installer packages..."
if [[ "$PLATFORM" == "windows-x86_64" ]]; then
    # Copy MSI installer
    MSI_FILE=$(find target/release/bundle/msi -name "Vult_${VERSION}_*.msi" 2>/dev/null | head -1)
    if [ -f "$MSI_FILE" ]; then
        cp "$MSI_FILE" "$DIST_DIR/"
        echo "  ✓ MSI installer: $(basename "$MSI_FILE")"
    fi

    # Copy NSIS installer
    NSIS_FILE=$(find target/release/bundle/nsis -name "Vult_${VERSION}_*-setup.exe" 2>/dev/null | head -1)
    if [ -f "$NSIS_FILE" ]; then
        cp "$NSIS_FILE" "$DIST_DIR/"
        echo "  ✓ NSIS installer: $(basename "$NSIS_FILE")"
    fi
elif [[ "$PLATFORM" == "macos-x86_64" ]]; then
    # Copy DMG installer
    DMG_FILE=$(find target/release/bundle/dmg -name "Vult_${VERSION}_*.dmg" 2>/dev/null | head -1)
    if [ -f "$DMG_FILE" ]; then
        cp "$DMG_FILE" "$DIST_DIR/"
        echo "  ✓ DMG installer: $(basename "$DMG_FILE")"
    fi

    # Copy .app bundle (optionally)
    APP_BUNDLE=$(find target/release/bundle/macos -name "Vult.app" 2>/dev/null | head -1)
    if [ -d "$APP_BUNDLE" ]; then
        cp -r "$APP_BUNDLE" "$DIST_DIR/"
        echo "  ✓ App bundle: Vult.app"
    fi
elif [[ "$PLATFORM" == "linux-x86_64" ]]; then
    # Copy Debian package
    DEB_FILE=$(find target/release/bundle/deb -name "vult_${VERSION}_*.deb" 2>/dev/null | head -1)
    if [ -f "$DEB_FILE" ]; then
        cp "$DEB_FILE" "$DIST_DIR/"
        echo "  ✓ Debian package: $(basename "$DEB_FILE")"
    fi

    # Copy AppImage
    APPIMAGE_FILE=$(find target/release/bundle/appimage -name "vult_${VERSION}_*.AppImage" 2>/dev/null | head -1)
    if [ -f "$APPIMAGE_FILE" ]; then
        cp "$APPIMAGE_FILE" "$DIST_DIR/"
        echo "  ✓ AppImage: $(basename "$APPIMAGE_FILE")"
    fi
fi

# Copy documentation
cp README.md "$DIST_DIR/"
cp LICENSE "$DIST_DIR/"
cp CHANGELOG.md "$DIST_DIR/"
cp docs/CLI_GUIDE.md "$DIST_DIR/" 2>/dev/null || true

# Create archive
ARCHIVE_NAME="vult-${VERSION}-${PLATFORM}.tar.gz"
echo "Creating archive: ${ARCHIVE_NAME}"
cd dist
tar -czf "$ARCHIVE_NAME" "vult-${VERSION}-${PLATFORM}"
cd ..

# Generate checksums
echo "Generating checksums..."
cd dist
if command -v sha256sum &> /dev/null; then
    sha256sum "$ARCHIVE_NAME" > "${ARCHIVE_NAME}.sha256"
elif command -v shasum &> /dev/null; then
    shasum -a 256 "$ARCHIVE_NAME" > "${ARCHIVE_NAME}.sha256"
fi
cd ..

echo -e "${GREEN}✓ Release build complete!${NC}"
echo ""
echo "Artifacts:"
echo "  - dist/${ARCHIVE_NAME}"
echo "  - dist/${ARCHIVE_NAME}.sha256"
echo ""
echo "Package contents:"
ls -lh "$DIST_DIR/" | grep -E "\.(exe|msi|dmg|deb|AppImage)$" || true
echo ""
echo "Binary sizes:"
ls -lh "$DIST_DIR/vult${EXE_EXT}" "$DIST_DIR/vult-gui${EXE_EXT}"
echo ""

# Verify binaries work
echo -e "${YELLOW}Verifying binaries...${NC}"
"$DIST_DIR/vult${EXE_EXT}" --version || { echo -e "${RED}CLI binary failed${NC}"; exit 1; }
echo -e "${GREEN}✓ CLI binary works${NC}"

# GUI verification requires display, skip in headless
if [ -n "$DISPLAY" ] || [[ "$OSTYPE" == "msys" ]]; then
    timeout 2 "$DIST_DIR/vult-gui${EXE_EXT}" 2>/dev/null || echo -e "${YELLOW}GUI binary check skipped (requires display)${NC}"
fi

echo ""
echo -e "${GREEN}Release ready for distribution!${NC}"
echo "Next steps:"
echo "  1. Test the binaries in dist/${ARCHIVE_NAME}"
echo "  2. Create GitHub release"
echo "  3. Upload artifacts"
echo "  4. Update documentation"
