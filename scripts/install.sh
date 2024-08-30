#!/bin/bash

set -e

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" = "x86_64" ]; then
  ARCH="amd64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
  ARCH="arm64"
else
  echo "Unsupported architecture: $ARCH"
  exit 1
fi

# Set installation directory
if [ "$OS" = "darwin" ]; then
  INSTALL_DIR="$HOME/.crabodex"
  OS="macos"
else
  INSTALL_DIR="$HOME/.local/bin"
fi

# Create installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Determine latest release
LATEST_RELEASE=$(curl -s https://api.github.com/repos/fabien-h/crabodex/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Download binary
BINARY_NAME="crabodex-${OS}-${ARCH}"
DOWNLOAD_URL="https://github.com/fabien-h/crabodex/releases/download/${LATEST_RELEASE}/${BINARY_NAME}"

echo "Downloading Crabodex from $DOWNLOAD_URL"
curl -L "$DOWNLOAD_URL" -o "$INSTALL_DIR/crabodex"

# Make binary executable
chmod +x "$INSTALL_DIR/crabodex"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> ~/.bashrc
  echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> ~/.zshrc
  echo "Crabodex has been added to your PATH. Please restart your terminal or run 'source ~/.bashrc' (or ~/.zshrc) to update your current session."
fi

echo "Crabodex has been installed to $INSTALL_DIR/crabodex"
echo "You can now use the 'crabodex' command."
