#!/bin/bash

: << 'COMMENT'
Useful commands for rustup!
COMMENT

# Print version of rustup
rustup --version

# Show the active and installed toolchains or profiles
rustup show

# Check for updates to Rust toolchains and rustup
rustup check

# Update Rust toolchains and rustup
rustup update