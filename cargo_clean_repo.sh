#!/bin/bash

for dir in */; do
    if [ -f "${dir}Cargo.toml" ]; then
        echo "Cleaning ${dir}"
        (
            cd "$dir" || exit 1
            cargo clean
        )
        echo
    fi
done

echo "All Rust projects cleaned."