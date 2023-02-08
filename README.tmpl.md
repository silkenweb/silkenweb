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
- CSS scoping via [CSS Modules](https://github.com/css-modules/css-modules). See the [CSS modules example].
- [Routing][router example] that works on the client or server.
- [Tauri support](https://github.com/silkenweb/tauri-example)
- Server side rendering with hydration, and [compile time pre-rendering](https://github.com/silkenweb/ssr-example).
- Full stack apps using [Arpy]. See the [client-server example].

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

## Comparison With Other Frameworks

[Sycamore] and [Leptos] are 2 other signals based Rust frameworks. They are evolving quickly at the time of writing this comparison, as is [Silkenweb]. Also bear in mind I'm not that familiar with [Sycamore] or [Leptos].

- [Silkenweb] uses plain, non-macro Rust as much as possible, and a lot of effort has been put into making this ergonomic. [Sycamore] and [Leptos] primarily use a macro DSL to define components. I believe [Sycamore] also has a builder API.
- Ecosystem: [Leptos] and [Sycamore] have [`cargo-leptos`] and [Perseus] respectively, whereas [Silkenweb] doesn't have an ecosystem at this point.
- CSS Scoping: Silkenweb supports [CSS Modules]. See the [CSS modules example]. [CSS Modules] support is integrated with SSR and Hydration so that only the CSS required to render the initial page is sent from the server, then progressively enhanced as required on the client. I'm not aware of any CSS scoping support in [Leptos] or [Sycamore].
- Server Functions: [Leptos] supports server functions to seamlessly divide your app between client and server. [Silkenweb] doesn't directly support anything like this, but similar functionality is provided with [Arpy].
- [Sycamore] and [Leptos] both go to some effort to make cloning signals into closures more ergonomic. [Silkenweb] provides a `clone!` macro to make things a little easier, but otherwise doesn't address the problem. I'm not sure what the tradeoffs are for the [Sycamore]/[Leptos] approaches. Do they make cleaning up after derived signals harder? Do they mean more complex lifetime annotations? Do contexts need to be passed around everywhere?
- [Silkenweb] has support for using [third party web components](https://github.com/silkenweb/silkenweb/tree/main/examples/web-components-wrapper). I'm not sure about [Sycamore] or [Leptos].
- [Silkenweb] has support for [shadow roots](https://github.com/silkenweb/silkenweb/tree/main/examples/shadow-root), including Hydration and SSR support with the experimental [Declarative Shadow DOM](https://web.dev/declarative-shadow-dom/). It also has a simple [Component](https://github.com/silkenweb/silkenweb/tree/main/examples/component) wrapper to manage slots. Again, I'm not sure about [Leptos] and [Sycamore] here.
- [Silkenweb] doesn't use any unsafe Rust directly. Some of the underlying Crates do use unsafe, but at least you don't have to put as much trust in my coding skills!
- All of these frameworks support:
  - Static site generation.
  - Progressive enhancement using SSR and hydration.

## Design Tradeoffs

### No VDOM

The use of a signals-based approach can provide better performance because it allows the compiler to know the data dependencies within your application at compile time. This allows changes to be efficiently calculated at runtime. On the other hand, with a basic VDOM based approach, the changes need to be identified at runtime.

The drawback of a signals-based approach is that the code tends to be more complicated. However, in actual implementation, VDOM-based approaches often implement mechanisms to prevent complete re-rendering of the VDOM every time a change occurs, which adds some level of complexity to code using the VDOM approach.

### No Macro DSL

Using plain Rust syntax has numerous benefits, such as:

- No need to learn a macro Domain Specific Language (DSL).
- Improved code completion through `rust-analyser`.
- Familiar documentation structure, thanks to `rustdoc`.
- Code formatting with `rustfmt`. While macro DSLs can be formatted with `rustfmt` if designed with care, the syntax is limited by `rustfmt`'s capabilities.
- Exceptional compiler errors from `rustc`: Although macro DSLs can produce informative errors, a lot of work has been put into making `rustc` error messsages great.
- The ability to utilize Rust's composable, well-designed abstractions.

While a macro DSL could be developed to work with Silkenweb, the syntax in Rust is already well suited for defining document structure.

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
[CSS modules example]: https://github.com/silkenweb/silkenweb/tree/main/examples/css-modules
[router example]: https://github.com/silkenweb/silkenweb/tree/main/examples/router
[client-server example]: https://github.com/silkenweb/silkenweb/tree/main/examples/client-server
