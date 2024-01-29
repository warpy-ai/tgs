#!/bin/bash

# Ensure script fails on error
set -euo pipefail
set -o errexit
set -o nounset
set -o xtrace


TARGET=${TARGET}
PYTHON_PATH=${PYTHON_PATH}

# Set up environment variables for cross-compilation
case "${TARGET}" in
    "aarch64-unknown-linux-gnu")
        sudo apt-get update
        sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
        # Install Python for aarch64 from Arch Linux ARM repository
        wget https://github.com/indygreg/python-build-standalone/releases/download/20240107/cpython-3.10.13+20240107-aarch64-unknown-linux-gnu-install_only.tar.gz

        echo "LD_LIBRARY_PATH=$(pwd)/python-aarch64/lib:$LD_LIBRARY_PATH" >> $GITHUB_ENV
        echo "PYTHON_SYS_EXECUTABLE=$(pwd)/python-aarch64/bin/python3.10" >> $GITHUB_ENV
        echo "PYO3_CROSS_LIB_DIR=$(pwd)/python-aarch64/lib" >> $GITHUB_ENV
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
        echo 'musl-gcc "$@" -I/usr/include/x86_64-linux-musl -L/usr/lib/x86_64-linux-musl -lstdc++' | sudo tee -a /usr/local/bin/custom-musl-g++
        sudo chmod +x /usr/local/bin/custom-musl-g++

        # Set environment variables for custom wrapper
        echo 'CXX=g++' >> $GITHUB_ENV
        echo 'CXXFLAGS=-I/usr/include/x86_64-linux-musl' >> $GITHUB_ENV
        echo 'LDFLAGS=-L/usr/lib/x86_64-linux-musl' >> $GITHUB_ENV
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
