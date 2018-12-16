#!/bin/bash

cargo build --release
wasm-bindgen target/wasm32-unknown-unknown/release/rinvaders.wasm --out-dir docs --no-modules
