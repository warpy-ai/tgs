#!/bin/bash

# Ensure script fails on error
set -euo pipefail
set -o errexit
set -o nounset
set -o xtrace

# Variables from the GitHub Actions workflow
BIN_NAME=${BIN_NAME}
VERSION=$(git describe --tags --always)
TARGET=${TARGET}
OS=${OS}
DIST_DIR="./output"

echo "Packaging $BIN_NAME v$VERSION for $TARGET on $OS"

# Prepare the distribution directory
mkdir -p "$DIST_DIR"
rm -rf "$DIST_DIR/*"

# Build the project
echo "Building $BIN_NAME..."
cargo build --release --target "$TARGET" --verbose

# Copy binaries to distribution directory
ARCHIVE_NAME=""
if [ "$OS" = "windows-latest" ]; then
    cp "target/$TARGET/release/$BIN_NAME.exe" "$DIST_DIR/"
    cp -r "target/$TARGET/release/model" "$DIST_DIR/"
    cp "target/$TARGET/release/inference_model.py" "$DIST_DIR/"
    cd "$DIST_DIR"
    ARCHIVE_NAME="${BIN_NAME}-${VERSION}-${TARGET}.zip"
    7z a "$ARCHIVE_NAME" *
else
    cp "target/$TARGET/release/$BIN_NAME" "$DIST_DIR/"
    cp -r "target/$TARGET/release/model" "$DIST_DIR/"
    cp "target/$TARGET/release/inference_model.py" "$DIST_DIR/"
    cd "$DIST_DIR"
    ARCHIVE_NAME="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
    tar -czf "$ARCHIVE_NAME" *
fi

echo "Packaging completed: $DIST_DIR"

# Set the output for GitHub Actions
echo "archive=${DIST_DIR}/${ARCHIVE_NAME}" >> $GITHUB_ENV

