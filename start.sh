#!/bin/bash

# Download Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the application
cargo build --release

# Run Linutils as sudo
sudo ./target/release/linutils
