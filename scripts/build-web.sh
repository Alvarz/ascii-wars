#!/bin/bash
rustup target add wasm32-unknown-unknown
# cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/boss-rush.wasm --out-dir wasm --no-modules --no-typescript
mv wasm/boss-rush.js wasm/blob.js
mv wasm/boss-rush_bg.wasm wasm/blob.wasm
cp -r assets wasm/
