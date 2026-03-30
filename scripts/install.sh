#!/bin/bash
set -e

# BetterCurl installer
# This script downloads and installs the latest BetterCurl release

echo "Installing BetterCurl..."

# Determine platform and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    MINGW*|MSYS*|CYGWIN*)    OS="windows";;
    *)          echo "Unsupported OS: ${OS}"; exit 1;;
esac

case "${ARCH}" in
    x86_64|amd64)    ARCH="x86_64";;
    arm64|aarch64)   ARCH="aarch64";;
    *)               echo "Unsupported architecture: ${ARCH}"; exit 1;;
esac

# Build asset name
if [ "${OS}" = "windows" ]; then
    BINARY_NAME="bettercurl.exe"
    ARCHIVE_NAME="bettercurl-${OS}-${ARCH}.tar.gz"
else
    BINARY_NAME="bettercurl"
    ARCHIVE_NAME="bettercurl-${OS}-${ARCH}.tar.gz"
fi

# Get latest release version (default)
VERSION="${BETTERCURL_VERSION:-0.1.0}"
# You can also set repository as an environment variable
REPO="${BETTERCURL_REPO:-YOUR_USERNAME/bettercurl}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE_NAME}"

echo "Platform: ${OS}-${ARCH}"
echo "Version: ${VERSION}"
echo "Downloading from: ${DOWNLOAD_URL}"

# Download to temporary directory
TMPDIR="$(mktemp -d)"
cd "${TMPDIR}"

echo "Downloading..."
if command -v curl &> /dev/null; then
    curl -fsSL -o "${ARCHIVE_NAME}" "${DOWNLOAD_URL}"
elif command -v wget &> /dev/null; then
    wget -q "${DOWNLOAD_URL}"
else
    echo "Error: curl or wget required"
    exit 1
fi

# Extract
tar -xzf "${ARCHIVE_NAME}"

# Determine install location
INSTALL_DIR="/usr/local/bin"
if [ ! -w "${INSTALL_DIR}" ]; then
    echo "⚠️  /usr/local/bin is not writable. Installing to ~/.local/bin instead."
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
fi

# Install
echo "Installing to ${INSTALL_DIR}..."
cp "${BINARY_NAME}" "${INSTALL_DIR}/"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Cleanup
cd - > /dev/null
rm -rf "${TMPDIR}"

echo "✅ BetterCurl installed successfully!"
echo "Version: ${VERSION}"
echo "Location: ${INSTALL_DIR}/${BINARY_NAME}"
echo ""
echo "Run: bettercurl --help"
