#!/usr/bin/env bash
# Wrapper script to build with cargo and then apply UPX compression

echo "Building release version with cargo..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful, now running UPX compression..."
    cargo run --release --bin post_build
else
    echo "Build failed, skipping UPX compression"
    exit 1
fi