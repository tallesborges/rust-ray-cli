#!/bin/bash

mkdir -p wasm-modules

for crate in crates/event-*; do
    if [ -d "$crate" ]; then
        (cd "$crate" && cargo build --target wasm32-unknown-unknown --release)
        cp target/wasm32-unknown-unknown/release/event_*.wasm wasm-modules/
    fi
done
