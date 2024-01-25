#!/bin/bash

# Ensure script fails on error
set -euo pipefail

# Variables from the GitHub Actions workflow
BIN_NAME=${BIN_NAME}
VERSION=$(git describe --tags --always)
TARGET=${TARGET}
OS=${OS:-"default_os"}
DIST_DIR="./output"

echo "Packaging $BIN_NAME v$VERSION for $TARGET on $OS"

# Prepare the distribution directory
mkdir -p "$DIST_DIR"
rm -rf "$DIST_DIR/*"

# Build the project
echo "Building $BIN_NAME..."
PYO3_PRINT_CONFIG=1 cargo build --release --target "$TARGET"

# Copy binaries to distribution directory
ARCHIVE_NAME=""
if [ "$OS" = "windows-latest" ]; then
    cp "target/$TARGET/release/$BIN_NAME.exe" "$DIST_DIR/"
    cd "$DIST_DIR"
    ARCHIVE_NAME="${BIN_NAME}-${VERSION}-${TARGET}.zip"
    7z a "$ARCHIVE_NAME" "$BIN_NAME.exe"
else
    cp "target/$TARGET/release/$BIN_NAME" "$DIST_DIR/"
    cd "$DIST_DIR"
    ARCHIVE_NAME="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
    tar -czf "$ARCHIVE_NAME" "$BIN_NAME"
fi

echo "Packaging completed: $DIST_DIR"

# Set the output for GitHub Actions
echo "::set-output name=archive::${DIST_DIR}/${ARCHIVE_NAME}"
