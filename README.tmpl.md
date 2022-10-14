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
- Routing.
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

There's nothing to stop a macro DSL being built on top of Silkenweb, to complement the builder APIs.

The advantage of using a macro DSL is that syntax is tailored to defining document structure. Rust syntax is fairly well suited to this, so it's not much of a benefit in reality.

## Learning

- `cargo doc --open`
- Check out the [examples](https://github.com/silkenweb/silkenweb/tree/main/examples) folder
- [futures-signals tutorial](https://docs.rs/futures-signals/0.3.24/futures_signals/tutorial/index.html)
- Feel free to ask any questions on our [Discord](https://discord.gg/usSUczZ5ua) channel.

## Pre Built Examples

- [animation](https://silkenweb.netlify.app/examples/animation)
- [async-http-request](https://silkenweb.netlify.app/examples/async-http-request)
- [bootstrap](https://silkenweb.netlify.app/examples/bootstrap)
- [counter](https://silkenweb.netlify.app/examples/counter)
- [counter-list](https://silkenweb.netlify.app/examples/counter-list)
- [element-handle](https://silkenweb.netlify.app/examples/element-handle)
- [hackernews-clone](https://silkenweb.netlify.app/examples/hackernews-clone)
- [hello-world](https://silkenweb.netlify.app/examples/hello-world)
- [hydration](https://silkenweb.netlify.app/examples/hydration)
- [router](https://silkenweb.netlify.app/examples/router)
- [todomvc](https://silkenweb.netlify.app/examples/todomvc)
- [ui5-showcase](https://silkenweb.netlify.app/examples/ui5-showcase)
