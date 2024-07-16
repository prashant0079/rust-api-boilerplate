#!/bin/bash

# Run cargo clean to remove build artifacts
cargo clean

# Run cargo fmt to format the code
cargo fmt

# Run clippy to lint the code
cargo clippy -- -D warnings

# Remove target directory to free up space
rm -rf target

# Check for unused dependencies (requires nightly)
# cargo +nightly udeps

echo "Project cleaned up!"
