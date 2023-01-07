# Changelog

## Next

<!-- TODO: Reword and link to examples -->
- Signal and value method variants, such as `class` and `class_signal`, have been merged into a single method that accepts either a value or a signal. The only exception to this is `children_signal`, which still exists.
- Added a work-in-progress bootstrap component library, and associated example.
- TODO: Document removal of builder types.
- Split `class` into `class` and `classes`. `class` adds a single class and `classes` adds a collection of classes. `class` and `classes` can be called multiple times to add more classes.
- Add a `weak-refs` cargo feature flag to enable some event handling optimizations when weak refs are available.
- Hydrate and mount over the mount point, rather than a child of the mount point.
- Add templates, which allow you to pre render the DOM structure and instantiate it with more detail later. This can improve the performance of hot code paths.
- Event handlers can be installed on the document.
- `#[derive(...)]` macros for `Value`, `HtmlElement`, `AriaElement`, `HtmlElementEvents`, and `ElementEvents`.
- `#[derive(Element)]` accepts an `element_dom_type` parameter to specify the dom type.
- Explicit low level DOM abstraction with types for `Wet` (client only), `Dry` (server only), `Hydro` (client or server, with hydration), and `Template`.
- Rename `html_element` to `custom_html_element`
- `UrlPath` supports URL hashes.
- `impl From<String> for UrlPath`
- Fix animation timestamps. Previously they would be relative to the previous animation frame, now they start at 0.
- Add `on_animation_frame`
- Add `handle` method to `Element`
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
