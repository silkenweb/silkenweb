# Installation

## Rust

First, install Rust by following the [official instructions](https://www.rust-lang.org/tools/install).

## OS Setup

On a fresh Ubuntu install, you'll need to install some basic packages for `trunk` and `wasm-bindgen`:

```bash
sudo apt install gcc-multilib libssl-dev git g++
```

## Wasm Tooling

You'll need to install the wasm32 target and some tools to build your apps:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-pack
cargo install wasm-bindgen-cli --version 0.2.78
```

[`trunk`] is needed to build and serve your apps. [`trunk`] uses [`wasm-pack`] to package your app for the web, and [`wasm-bindgen`] to generate javascript bindings. [`wasm-bindgen`] is rapidly changing, so we install a specific version.

[`trunk`]: https://trunkrs.dev/
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/
[`wasm-bindgen`]: https://rustwasm.github.io/docs/wasm-bindgen/
