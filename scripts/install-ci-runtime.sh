#!/usr/bin/env bash

set -euo pipefail

cargo-install() {
    if ! command -v "$1" &> /dev/null; then
        cargo install "$2" --version "$3"
    fi
}

cargo-install wasm-bindgen wasm-bindgen-cli 0.2.73
cargo-install wasm-pack wasm-pack 0.9.1
cargo-install trunk trunk 0.10.0
cargo-install cargo-udeps cargo-udeps 0.1.20
cargo-install mdbook mdbook 0.4.7
