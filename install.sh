#!/bin/bash

# Ensure the script fails on error
set -euo pipefail

# Define the GitHub repository
REPO="warpy-ai/tgs"

# Check if a version argument was provided
if [ "$#" -eq 1 ]; then
    VERSION="$1"
    echo "User specified version: $VERSION"
else
    # Fetch the latest release tag from the GitHub API
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    echo "No version specified, using latest: $VERSION"
fi

# Define variables
INSTALL_DIR="/usr/local/bin"
TMP_DIR=$(mktemp -d)

# Function to identify the OS and architecture, then construct the download URL
set_download_url() {
    OS=$(uname -s)
    ARCH=$(uname -m)
    BASE_URL="https://github.com/$REPO/releases/download/$VERSION"

    case "$OS" in
        "Darwin")
            case "$ARCH" in
                "arm64")
                    # Apple Silicon
                    FILE_NAME="tgs-${VERSION}-aarch64-apple-darwin.tar.gz"
                    ;;
                "x86_64")
                    # Intel Mac
                    FILE_NAME="tgs-${VERSION}-x86_64-apple-darwin.tar.gz"
                    ;;
                *)
                    echo "Unsupported architecture: $ARCH"
                    exit 1
                    ;;
            esac
            ;;
        "Linux")
            # Assuming x86_64 for Linux, adjust if supporting other architectures
            FILE_NAME="tgs-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
            ;;
        *)
            echo "Unsupported operating system: $OS"
            exit 1
            ;;
    esac

    BIN_URL="${BASE_URL}/${FILE_NAME}"
}

# Download and install
download_and_install() {
    echo "Downloading $BIN_URL"
    curl -L $BIN_URL -o "$TMP_DIR/build.tar.gz"

    echo "Extracting..."
    tar -xzvf "$TMP_DIR/build.tar.gz" -C "$TMP_DIR"

    echo "Installing..."
    # Assuming the binary name is 'tgs', adjust if necessary
    mv "$TMP_DIR/tgs" "$INSTALL_DIR"

    echo "Cleanup..."
    rm -rf "$TMP_DIR"

    echo "Installation completed successfully."
}

# Main
set_download_url
download_and_install
