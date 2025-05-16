# Changelog

## Next

- *Breaking change*: `silkenweb::prelude` has been removed. It wasn't maintained and caused issues with Rust Analyser prefering to generate prelude imports.
- *Breaking change*: `TextParentElement` has been split out from `ParentElement`. This should just mean you need to import `silkenweb::node::element::TextParentElement` in some cases.
- Add a mutation API based on `MutationObserver`. See `examples/mutation-observer`.
- Add `element_slot` and `element_slot_single` to generate methods for adding slotted children to custom HTML elements.
- *Breaking change*: `custom_html_element!` no longer has a `custom_events` section. You can now just use `elements::CustomEvent` in the `events` section. See `examples/web-components-wrapper`.
- `StrAttribute` derive macro to `impl Attribute` for types that implement `AsRef<str>`

## 0.9.0 - 2025-01-14

- Add SVG elements:
  - `<animateMotion>`
  - `<animateTransform>`
  - `<feDropShadow>`
  - `<image>`
  - `<linearGradient>`
  - `<radialGradient>`
- Migrate to Tauri v2

## 0.8.0 - 2024-05-02

- `dbg` macro.
- `Namespace` owns its name string instead of referencing a `'static str`, so it doesn't need to be known at compile time.
- `MountHandle` has been removed. It never quite worked properly and was a misfeature. Use `Document::unmount_all` for testing instead.
- Routing: if there's no `<base href="..."/>` set, routing will use the origin of the current URL. Previously it used the whole URL.

## 0.7.1 - 2023-11-07

- Canvas example (see `examples/canvas`).
- Update `web-sys` and `wasm-bindgen` to latest version.
- Compile with latest version of `syn`.

## 0.7.0 - 2023-11-04

- `Attribute` defines an associated type `Text`, meaning we no longer need to return `Cow<str>`. The attribute can decide if it's text type is a reference or value.
- Rename `map_element` to `map_element_signal` and add a `map_element` method.
- Lock the version of `parcel_selectors` to stop builds being broken by `lightningcss` updates.
- Add Playwright example tests for TodoMVC.

## 0.6.0 - 2023-10-15

- `Element::DomType` was renamed to `Element::DomElement`.
- Added `task::TaskSignal*` traits.
- Add the `silkenweb_task` crate to help you split business logic and user interface code.
- Added `css-transpile` as an opt-in feature, as it can significatly increase build time.
- Add `Element::map_element` that maps a function over the javascript element for each signal value.
- Add signal based setters for `<input>` `value` and `checked` properties.
- Move `ssr-example` from it's own repo into the main Silkenweb repo.

### Fixes

- `ElementHandle` works with `Hydro` DOM.

## 0.5.0 - 2023-07-17

- SSR Scopes: Each tokio task now has a local task queue to emulate the browsers microtask queue.
- htmx integration using Axum.
- Reactive stylesheets.
- Drag and drop events.
- `css!` supports Sass.
- Portable timers with `time::{sleep, interval}`.
- Window events.
- Better support for testing via `silkenweb-test`.

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
