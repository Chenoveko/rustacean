#!/bin/bash

: << 'COMMENT'
Useful commands for cargo!
COMMENT

# Cargo version
cargo version

# Cargo help
cargo -h

# See all commands
cargo --list

# Create a new Rust binary package
cargo new my_project
cargo new --bin my_project # explicitly

# Remove generated build artifacts (./target)
cargo clean

# Check the project for compilation errors without producing an executable
cargo check
cargo check --release 

# Compile the project -> 2 modes
cargo build # debug mode (for development)
cargo build --release # release mode (optimized, for final program)

# Compile and run the project
cargo run
cargo run --release

# Create a new Rust library package
cargo new --lib my_library

# Format code 
cargo fmt # rustfmt for a set of files
