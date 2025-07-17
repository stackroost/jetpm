#!/bin/bash

set -e

# Build in release mode
cargo build --release

# Define the destination directory
DEST=${1:-"$HOME/.local/bin"}

# Create the directory if it doesn't exist
mkdir -p "$DEST"

# Copy the binary
cp target/release/neonpack "$DEST"

echo "neonpack built and copied to $DEST"
echo "Make sure $DEST is in your PATH"