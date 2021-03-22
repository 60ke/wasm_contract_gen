#!/bin/bash
cargo +nightly -Z unstable-options build --release --out-dir . --target wasm32-unknown-unknown
cp ./target/json/abi.json ./
cargo clean