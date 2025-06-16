#!/bin/bash

# Build oxy_wasm crate
cd crates/oxy_wasm
$HOME/.cargo/bin/wasm-pack build --target web --out-dir ../../pkg
cd ../..
