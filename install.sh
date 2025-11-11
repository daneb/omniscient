#!/bin/bash
# Omniscient Installation Script
# Install omniscient CLI command history tracker

set -e

REPO="daneb/omniscient"
VERSION="v1.0.0"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "🔍 Omniscient Installer"
echo "======================="
echo

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS names
case "$OS" in
    darwin)
        OS="macos"
        ;;
    linux)
        OS="linux"
        ;;
    *)
        echo "❌ Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Check if Rust is installed
if command -v cargo &> /dev/null; then
    echo "✓ Found Rust installation"
    echo
    echo "Installing from source..."

    # Clone or update repository
    if [ -d "omniscient" ]; then
        echo "→ Updating existing repository..."
        cd omniscient
        git pull
    else
        echo "→ Cloning repository..."
        git clone "https://github.com/${REPO}.git"
        cd omniscient
    fi

    # Build and install
    echo "→ Building omniscient..."
    cargo build --release

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Copy binary
    echo "→ Installing to $INSTALL_DIR..."
    cp target/release/omniscient "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/omniscient"

    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo
        echo "⚠️  Add $INSTALL_DIR to your PATH:"
        echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
        echo
        echo "   Add this line to your ~/.bashrc, ~/.zshrc, or equivalent"
    fi

    cd ..
else
    echo "❌ Rust not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo
echo "✅ Installation complete!"
echo
echo "Next steps:"
echo "1. Ensure $INSTALL_DIR is in your PATH"
echo "2. Run: omniscient init >> ~/.zshrc"
echo "3. Reload your shell: source ~/.zshrc"
echo "4. Start using: omniscient --help"
echo
echo "Documentation: https://github.com/${REPO}#readme"
