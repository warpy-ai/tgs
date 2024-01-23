#!/bin/bash

# Ensure script fails on error
set -e

# Variables
# Variables from the GitHub Actions workflow
BIN_NAME=${BIN_NAME:-"default_binary_name"} # Default name if not provided
VERSION=$(git describe --tags --always)     # Assumes tags are used for versioning
TARGET=$TARGET                             # From GitHub Actions env
OS=$OS                                     # From GitHub Actions env
DIST_DIR="./output"                        # Directory to store the output

echo "Packaging $BIN_NAME v$VERSION for $TARGET on $OS"


# Check if necessary tools are installed (e.g., cargo, tar, zip, etc.)
command -v cargo >/dev/null 2>&1 || { echo >&2 "Cargo is required but not installed. Aborting."; exit 1; }
# Add checks for other tools like 7z or aria2c if needed

# Prepare the distribution directory
mkdir -p "$DIST_DIR"
rm -rf "$DIST_DIR/*"

# Build the project
echo "Building $BIN_NAME..."
cargo build --release --target "$TARGET" --all
# Add any additional build flags or steps specific to your project

# Prepare the binaries for distribution
cd "target/$TARGET/release"
if [ "$OS" = "windows-latest" ]; then
    # Windows-specific packaging, e.g., zip or msi
    cp "$BIN_NAME.exe" "$DIST_DIR/"
    cd "$DIST_DIR"
    zip "${BIN_NAME}-${VERSION}-${TARGET}.zip" "$BIN_NAME.exe"
else
    # Unix-like OS packaging, e.g., tar.gz
    cp "$BIN_NAME" "$DIST_DIR/"
    cd "$DIST_DIR"
    tar -czf "${BIN_NAME}-${VERSION}-${TARGET}.tar.gz" "$BIN_NAME"
fi

echo "Packaging completed: ${DIST_DIR}"

# Add any additional steps like uploading to a release, etc.
