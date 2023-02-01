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
- CSS encapsulation via [CSS Modules](https://github.com/css-modules/css-modules).
- Routing.
- [Tauri support](https://github.com/silkenweb/tauri-example)
- Server side rendering with hydration, and [compile time pre-rendering](https://github.com/silkenweb/ssr-example).
- Downcasts Js objects for you, where the type is known at compile time. For example, `button().on_click(...)` passes your event handler a `web_sys::HtmlInputElement` and a `web_sys::MouseEvent`.

## Example: A Simple Counter

```rust
use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{elements::html::*, prelude::*, value::Sig};

fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal().map(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)));

    mount("app", app);
}

```

## Quick Start

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cd examples/counter
trunk serve --open
```

## Comparison With Other Frameworks

[Sycamore] and [Leptos] are 2 other signals based Rust frameworks. They are evolving quickly at the time of writing this comparison, as is [Silkenweb]. Also bear in mind I'm not that familiar with [Sycamore] or [Leptos].

- [Silkenweb] uses plain, non-macro Rust as much as possible, and a lot of effort has been put into making this ergonomic. I believe [Sycamore] has a builder API.
- Ecosystem: [Leptos] and [Sycamore] have [`cargo-leptos`] and [Perseus] respectively, whereas [Silkenweb] doesn't have an ecosystem at this point.
- CSS: Silkenweb supports [CSS Modules]. See this [example](https://github.com/silkenweb/silkenweb/tree/main/examples/css-modules). [CSS Modules] support is integrated with SSR and Hydration so that only the CSS required to render the initial page is sent from the server, then progressively enhanced as required on the client. I'm not aware of any CSS scoping in [Leptos] or [Sycamore].
- Server Functions: [Leptos] supports server functions to seamlessly divide your app between client and server. [Silkenweb] doesn't directly support anything like this, but similar functionality is provided with [Arpy].
- [Sycamore] and [Leptos] both go to some effort to make cloning signals into closures more ergonomic. [Silkenweb] provides a `clone!` macro to make things a little easier, but otherwise doesn't address the problem. I'm not sure what the tradeoffs are for the [Sycamore]/[Leptos] solutions. Does it make cleaning up after derived signals harder? Does it mean more complex lifetime annotations? Do contexts need to be passed around everywhere?
- [Silkenweb] has support for using [third party web components](https://github.com/silkenweb/silkenweb/tree/main/examples/web-components-wrapper). I'm not sure about [Sycamore] or [Leptos].
- [Silkenweb] has support for [shadow roots](https://github.com/silkenweb/silkenweb/tree/main/examples/shadow-root), including Hydration and SSR support with the experimental [Declarative Shadow DOM](https://web.dev/declarative-shadow-dom/). It also has a simple [Component](https://github.com/silkenweb/silkenweb/tree/main/examples/component) wrapper to manage slots. Again, I'm not sure about [Leptos] and [Sycamore] here.
- [Silkenweb] doesn't use any unsafe Rust directly. Some of the underlying Crates do use unsafe, but at least you don't have to put as much trust in my coding skills!
- All of these frameworks support:
  - Static site generation.
  - Progressive enhancement using SSR and hydration.

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
- [web-components-wrapper](https://silkenweb.netlify.app/examples/web-components-wrapper)

[Silkenweb]: https://github.com/silkenweb/silkenweb
[Sycamore]: https://github.com/sycamore-rs/sycamore
[Leptos]: https://github.com/leptos-rs/leptos
[`cargo-leptos`]: https://github.com/leptos-rs/cargo-leptos
[Perseus]: https://github.com/framesurge/perseus
[Arpy]: https://github.com/simon-bourne/arpy
[CSS Modules]: https://github.com/silkenweb/silkenweb/tree/main/examples/css-modules
