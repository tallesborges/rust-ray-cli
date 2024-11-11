#!/bin/bash

# Array of crate names
CRATES=(
    "event-application-log"
    "event-exception"
    "event-log"
    "event-query"
    "event-table"
)

# Create output directory
mkdir -p wasm-modules

# Loop through each crate and build it
for crate in "${CRATES[@]}"; do
    echo "Building $crate..."
    (cd "crates/$crate" && \
     cargo build --target wasm32-unknown-unknown --release && \
     cp ../../target/wasm32-unknown-unknown/release/${crate//-/_}.wasm ../../wasm-modules/)
    echo "✓ Built $crate"
done

echo "All WASM modules built successfully!"
echo "WASM files are in the wasm-modules directory"
