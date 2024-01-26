#!/bin/bash

# Ensure script fails on error
set -euo pipefail

TARGET=${TARGET}
PYTHON_PATH=${PYTHON_PATH}

# Set up environment variables for cross-compilation
case "${TARGET}" in
    "aarch64-unknown-linux-gnu")
        sudo apt-get update
        sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
        ;;
    "x86_64-unknown-linux-gnu")
        sudo apt-get update
        sudo apt-get install -y gcc-x86-64-linux-gnu g++-x86-64-linux-gnu
        ;;
    "x86_64-unknown-linux-musl")
        sudo apt-get update
        sudo apt-get install -y musl-tools musl-dev
        # Create a custom wrapper script for musl-g++
        echo '#!/bin/bash' | sudo tee /usr/local/bin/custom-musl-g++
        echo 'musl-gcc "$@" -I/usr/local/musl/include -L/usr/local/musl/lib -lstdc++' | sudo tee -a /usr/local/bin/custom-musl-g++
        sudo chmod +x /usr/local/bin/custom-musl-g++

        # Set environment variables for custom wrapper
        echo 'CXX=/usr/local/bin/custom-musl-g++' >> $GITHUB_ENV
        echo 'CXXFLAGS=-I/usr/local/musl/include' >> $GITHUB_ENV
        echo 'LDFLAGS=-L/usr/local/musl/lib' >> $GITHUB_ENV
        ;;
    "aarch64-apple-darwin")
        # macOS specific setup (if necessary)
        ;;
    "x86_64-apple-darwin")
        # macOS specific setup (if necessary)
        ;;
esac

# Set PYO3 environment variables
echo "PYO3_CROSS=1" >> $GITHUB_ENV
echo "PYO3_CROSS_PYTHON_VERSION=3.10" >> $GITHUB_ENV

# Dynamically set the Python library directory
# Assuming the lib directory is at the same level as bin
PYTHON_LIB_DIR="$(dirname ${PYTHON_PATH})/lib"
echo "PYO3_CROSS_LIB_DIR=$PYTHON_LIB_DIR" >> $GITHUB_ENV

# Debug: List contents of the Python installation directory
echo "Listing contents of Python installation directory..."
ls -l "$(dirname $(which python))"
ls -l "$PYTHON_LIB_DIR"

# Verify if the directory exists
if [ ! -d "$PYTHON_LIB_DIR" ]; then
    echo "Python library directory for cross-compilation does not exist: $PYTHON_LIB_DIR"
    exit 1
fi
