# Silkenweb

[![tests](https://github.com/silkenweb/silkenweb/actions/workflows/tests.yml/badge.svg)](https://github.com/silkenweb/silkenweb/actions/workflows/tests.yml)
[![crates.io](https://img.shields.io/crates/v/silkenweb.svg)](https://crates.io/crates/silkenweb)
[![Documentation](https://docs.rs/silkenweb/badge.svg)](https://docs.rs/silkenweb)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/silkenweb)](./LICENSE-APACHE)
[![Discord](https://img.shields.io/discord/881942707675729931)](https://discord.gg/usSUczZ5ua)

A library for building reactive web apps.

## Features

- No VDOM. Fine grained reactivity using signals to minimize DOM API calls.
- Uses plain Rust syntax rather than a macro DSL.
- [Tauri support](https://github.com/silkenweb/tauri-example)
- Server side rendering with hydration, and [compile time pre-rendering](https://github.com/silkenweb/ssr-example).
- Downcasts Js objects for you, where the type is known at compile time. For example, `button().on_click(...)` passes your event handler a `web_sys::HtmlInputElement` and a `web_sys::MouseEvent`.

## Example: A Simple Counter

```rust
{{{ include "examples/counter/src/main.rs" }}}
```

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cd examples/counter
trunk serve --open
```

## Design Tradeoffs

### No VDOM

There are potential performance advantages to using a signals based approach, as you're telling the compiler explicitly what the dependencies within your app are, at compile time. With a simple VDOM based approach, you have to figure out what changed at runtime. The tradeoff here is slightly more complex code when using a signals based approach.

What tends to happen in practice is that VDOM based approaches will add some mechanism so that every time someting changes, your app doesn't need to completely re-render the VDOM. This inevitably adds some complexity to VDOM based approaches.

### No Macro DSL

There are many advantages to using plain Rust syntax:

- No macro DSL to learn.
- Code completion with `rust-analyser`.
- The documentation is structured in a familiar manner, courtesy of `rustdoc`.
- Code formatting with `rustfmt`.
- Excellent compiler errors, courtesy of `rustc`.
- Use Rust's well thought out, composable abstractions. Need control flow in your components? Use `if`, `match`, `dyn traits`, or whatever else Rust provides.

Of course the downside is that syntax is unlikely to be quite as good as a dedicated macro DSL. It's also possible that a sufficiently well thought out and mature macro DSL could also provide great compiler errors and abstractions that compose well with Rust.

There's nothing to stop a macro DSL being built on top of Silkenweb, to complement the builder APIs.

## Learning

- `cargo doc --open`
- Check out the [examples](https://github.com/silkenweb/silkenweb/tree/main/examples) folder
- [futures-signals tutorial](https://docs.rs/futures-signals/0.3.24/futures_signals/tutorial/index.html)
- Feel free to ask any questions on our [Discord](https://discord.gg/usSUczZ5ua) channel.
