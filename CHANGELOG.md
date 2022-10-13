# Changelog

## Next

- `impl From<String> for UrlPath`
- Use `serde_wasm_bindgen` for Tauri serialization to fix warnings about serde_json.
- Rename `html_element` to `custom_html_element`
- Fix animation timestamps
- Add `on_animation_frame`
- Add `handle` method to `Element`
- `class` and `class_signal` methods moved from `HtmlElement`/`SvgElement` to `ElementBuilder`
- Change `class_signal` to update the classes rather than overwrite them

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
