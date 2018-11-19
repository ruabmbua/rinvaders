#!/bin/bash

cargo build 
wasm-bindgen target/wasm32-unknown-unknown/debug/rinvaders.wasm --out-dir res --no-modules
