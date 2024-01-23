#!/bin/bash

# Ensure script fails on error
set -euo pipefail

# Variables from the GitHub Actions workflow
BIN_NAME=${BIN_NAME:-"default_binary_name"}
VERSION=$(git describe --tags --always)
TARGET=${TARGET:-"default_target"}
OS=${OS:-"default_os"}
DIST_DIR="./output"

echo "Packaging $BIN_NAME v$VERSION for $TARGET on $OS"

# Prepare the distribution directory
mkdir -p "$DIST_DIR"
rm -rf "$DIST_DIR/*"

# Build the project
echo "Building $BIN_NAME..."
cargo build --release --target "$TARGET"
# Add additional build flags if necessary

# Copy binaries to distribution directory
if [ "$OS" = "windows-latest" ]; then
    cp "target/$TARGET/release/$BIN_NAME.exe" "$DIST_DIR/"
    cd "$DIST_DIR"
    7z a "${BIN_NAME}-${VERSION}-${TARGET}.zip" "$BIN_NAME.exe"
else
    cp "target/$TARGET/release/$BIN_NAME" "$DIST_DIR/"
    cd "$DIST_DIR"
    tar -czf "${BIN_NAME}-${VERSION}-${TARGET}.tar.gz" "$BIN_NAME"
fi

echo "Packaging completed: $DIST_DIR"
