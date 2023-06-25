# Changelog

## Next

- SSR Scopes: Each tokio task now has a local task queue to emulate the browsers microtask queue.
- htmx integration using Axum.
- Reactive stylesheets.
- Drag and drop events.

### Fixes

- SSR now escapes text.
- Event handlers work for children of `Component`.

## 0.4.0 - 2023-01-19

### New Features

- CSS Modules support. See `examples/css-modules`.
- Components: A lightweight way to encapsulate HTML and CSS using shadow DOM. See `silkenweb::node::Component`.
- Event handlers can be installed at the document level.
- Elements can be mounted as a child of `<head>`.
- Templates, which allow you to pre render the DOM structure and instantiate it with more detail later. This can improve the performance of hot code paths.
- `css!` optionally supports nesting, validation, minification and vendor prefixes.
- `declarative-shadow-dom` feature flag to support [Declarative Shadow DOM](https://web.dev/declarative-shadow-dom/) with SSR.
- `on_animation_frame` will run a closure on the next animation frame.
- `UrlPath` can extract the URL's hash.
- `impl From<String> for UrlPath`
- A `handle` method on `Element`
- `#[derive(...)]` macros for `Value`, `HtmlElement`, `AriaElement`, `HtmlElementEvents`, and `ElementEvents`.
- `weak-refs` cargo feature flag to enable event handling optimizations when weak refs are available.
- A work-in-progress bootstrap component library.

### Improvements

- Merge `css!` and `css_classes!` macros into `css!`, which now supports inline CSS.
- Merge element builder types and their targets. For example `DivBuilder` and `Div` are merged into `Div`. Each element type has a marker type for mutablility which defaults to mutable.
- Merge signal and value method variants, such as `class` and `class_signal`, into a single method that accepts either a value or a signal. The only exception to this is `children_signal`, which still exists.
- Split `class` into `class` and `classes`. `class` adds a single class and `classes` adds a collection of classes. `class` and `classes` can be called multiple times to add more classes.
- Hydrate and mount directly onto the mount point, rather than onto a child of the mount point.
- Explicit low level DOM abstraction with types for `Wet` (client only), `Dry` (server only), `Hydro` (client or server, with hydration), and `Template`.
- Rename `html_element` to `custom_html_element`

### Fixes

- Fix animation timestamps. Previously they were relative to the previous animation frame, now they start at 0.
- Use `serde_wasm_bindgen` for Tauri serialization to fix warnings about serde_json.

## 0.3.0 - 2022-10-05

- [Tauri support](https://github.com/silkenweb/tauri-example)
- [Server side routing and pre-rendering](https://github.com/silkenweb/ssr-example)
- Routing improvements:
  - `UrlPath` struct with methods to destructure the URL
  - `router::anchor` and `router::link_clicked` convenience methods
- Compile time constants for CSS classes can be generated with `css_classes!`
- Compilation of SCSS and checking of inline CSS with `css!`
- Webcomponents:
  - Support for shadow root (`attach_shadow_children`)
  - Support for HTML `slot` elements
  - `html_element!` syntax works better with `rustfmt`
- Element handles, so you can reference an element from elsewhere in the DOM tree. See the [Element Handle](examples/element-handle) example
- DOM:
  - SVG support
  - Aria support

## 0.2.0 - 2021-02-09

- Use `futures-signals` for reactivity
- Support for server side rendering with hydration

## 0.1.1 - 2021-04-30
