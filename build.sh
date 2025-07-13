#!/bin/bash

set -e

# Build in release mode
sudo cargo build --release
# Copy the binary
sudo cp target/release/neonpack /home/mahesh/bin/neonpack

echo "neonpack built and moved to /usr/local/bin/neonpack"
