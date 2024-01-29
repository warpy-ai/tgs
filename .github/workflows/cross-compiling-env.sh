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
        sudo apt-get install libxcb-composite0-dev -y
        sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
        wget https://github.com/indygreg/python-build-standalone/releases/download/20240107/cpython-3.10.13+20240107-aarch64-unknown-linux-gnu-install_only.tar.gz
        tar -xzvf cpython-3.10.13+20240107-aarch64-unknown-linux-gnu-install_only.tar.gz

        PYTHON_LIB_DIR=$(pwd)/python/lib
        export LD_LIBRARY_PATH=$PYTHON_LIB_DIR:${LD_LIBRARY_PATH:-}

        # Set PKG_CONFIG_PATH, handling the case where it might not be set
        if [ -z "${PKG_CONFIG_PATH+x}" ]; then
            # PKG_CONFIG_PATH is not set
            export PKG_CONFIG_PATH=$PYTHON_LIB_DIR/pkgconfig
        else
            # PKG_CONFIG_PATH is set, append to it
            export PKG_CONFIG_PATH=$PYTHON_LIB_DIR/pkgconfig:$PKG_CONFIG_PATH
        fi

        export RUSTFLAGS="-C link-arg=-Wl,-rpath,$PYTHON_LIB_DIR"
        echo "Listing contents of Python installation directory for aarch64..."
        ls -l "$PYTHON_LIB_DIR"
        ;;
    "x86_64-unknown-linux-gnu")
        sudo apt-get update
        sudo apt-get install libxcb-composite0-dev -y
        sudo apt-get install -y gcc-x86-64-linux-gnu g++-x86-64-linux-gnu
        # Dynamically set the Python library directory
        # Assuming the lib directory is at the same level as bin
        PYTHON_LIB_DIR="$(dirname ${PYTHON_PATH})/lib"
        echo "PYO3_CROSS_LIB_DIR=$PYTHON_LIB_DIR" >> $GITHUB_ENV
        ls -l "$(dirname $(which python))"
        ;;
    "x86_64-unknown-linux-musl")
        sudo apt-get update
        sudo apt-get install libxcb-composite0-dev -y
        sudo apt-get install -y musl-tools musl-dev
        # Create a custom wrapper script for musl-g++
        echo '#!/bin/bash' | sudo tee /usr/local/bin/custom-musl-g++

        # Set environment variables for custom wrapper
        echo 'CXX=g++' >> $GITHUB_ENV
        echo 'CXXFLAGS=-I/usr/include/x86_64-linux-musl' >> $GITHUB_ENV
        echo 'LDFLAGS=-L/usr/lib/x86_64-linux-musl' >> $GITHUB_ENV
        # Dynamically set the Python library directory
        # Assuming the lib directory is at the same level as bin
        PYTHON_LIB_DIR="$(dirname ${PYTHON_PATH})/lib"
        echo "PYO3_CROSS_LIB_DIR=$PYTHON_LIB_DIR" >> $GITHUB_ENV
        ;;
    "aarch64-apple-darwin")
        # macOS specific setup (if necessary)
        # Dynamically set the Python library directory
        # Assuming the lib directory is at the same level as bin
        PYTHON_LIB_DIR="$(dirname ${PYTHON_PATH})/lib"
        echo "PYO3_CROSS_LIB_DIR=$PYTHON_LIB_DIR" >> $GITHUB_ENV
        ;;
    "x86_64-apple-darwin")
        # macOS specific setup (if necessary)
        # Dynamically set the Python library directory
        # Assuming the lib directory is at the same level as bin
        PYTHON_LIB_DIR="$(dirname ${PYTHON_PATH})/lib"
        echo "PYO3_CROSS_LIB_DIR=$PYTHON_LIB_DIR" >> $GITHUB_ENV
        ;;
esac

# Set PYO3 environment variables
echo "PYO3_CROSS=1" >> $GITHUB_ENV
echo "PYO3_CROSS_PYTHON_VERSION=3.10" >> $GITHUB_ENV

# Debug: List contents of the Python installation directory
echo "Listing contents of Python installation directory..."
ls -l "$(dirname $(which python))"
ls -l "$PYTHON_LIB_DIR"

# Verify if the directory exists
if [ ! -d "$PYTHON_LIB_DIR" ]; then
    echo "Python library directory for cross-compilation does not exist: $PYTHON_LIB_DIR"
    exit 1
fi
