# Silkenweb

[![tests](https://github.com/silkenweb/silkenweb/actions/workflows/tests.yml/badge.svg)](https://github.com/silkenweb/silkenweb/actions/workflows/tests.yml)
[![crates.io](https://img.shields.io/crates/v/silkenweb.svg)](https://crates.io/crates/silkenweb)
[![Documentation](https://docs.rs/silkenweb/badge.svg)](https://docs.rs/silkenweb)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/silkenweb)](./LICENSE-APACHE)

A library for building reactive single page web apps.

## Features

- Fine grained reactivity using signals to minimize DOM API calls
- No VDOM. Calls to the DOM API and your rendering code are minimized using signals.
- Uses plain Rust syntax rather than a macro DSL
- Downcasts Js objects for you where the type is known at compile time. For example:
  - `input().dom_element()` returns a `web_sys::HtmlInputElement`
  - `button().on_click(...)` passes your event handler a `web_sys::HtmlInputElement` and a `web_sys::MouseEvent`.

## Example: A Simple Counter

```rust
use futures_signals::signal::Mutable;
use silkenweb::{
    dom::mount,
    elements::{
        html::{button, div, p},
        ElementEvents, ParentBuilder,
    },
};

fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text_signal(count_text));

    mount("app", app);
}
```

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-pack
cargo install wasm-bindgen-cli --version 0.2.78
cd examples/counter
trunk serve --open
```

## Learning

- Check out the [examples](https://github.com/silkenweb/silkenweb/tree/main/examples) folder
