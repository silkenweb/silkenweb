//! A library for building reactive web apps
//!
//! # Overview
//!
//! - Pure rust API
//! - Fine grained reactivity using [`futures_signals`]
//! - [Routing]
//! - [Tauri] support
//! - [Server Side Rendering] with hydration
//!
//! # Quick Start
//!
//! First, install the `wasm32` target:
//!
//! ```bash
//! rustup target add wasm32-unknown-unknown
//! ```
//!
//! Then install [trunk]:
//!
//! ```bash
//! cargo install trunk --locked
//! ```
//!
//! To run the example [counter]:
//!
//! ```bash
//! cd examples/counter
//! trunk serve --open
//! ```
//!
//! # Feature Flags
//!
//! ## `weak-refs`
//!
//! Use Javascript weak references to manage event callbacks. This improves
//! performance but must be enabled in `wasm-bindgen`. See the [trunk]
//! documentation for details on how to do this using `data-weak-refs`.
//!
//! See [caniuse](https://caniuse.com/mdn-javascript_builtins_weakref) for
//! current browser support.
//!
//! ## `declarative-shadow-dom`
//!
//! Print [Declarative Shadow DOM] when server side rendering. Hydration will
//! correctly deal with shadow DOM regardless of this flag. See
//! [caniuse](https://caniuse.com/mdn-html_elements_template_shadowroot)
//! for browser support. Polyfills are available.
//!
//! # Learning
//!
//! There's extensive documentation on each module in this crate, along with
//! many other examples in the [examples] folder.
//!
//! Reactivity is provided by [`futures_signals`]. It would be helpful to
//! familiarize yourself using [`futures_signals::tutorial`].
//!
//! [trunk]: https://trunkrs.dev/
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [routing]: https://github.com/silkenweb/silkenweb/tree/main/examples/router
//! [tauri]: https://github.com/silkenweb/tauri-example
//! [Server Side Rendering]: https://github.com/silkenweb/ssr-example
//! [examples]: https://github.com/silkenweb/silkenweb/tree/main/examples
//! [Declarative Shadow DOM]: https://web.dev/declarative-shadow-dom/
use std::{cell::RefCell, collections::HashMap};

use document::{Document, MountHandle};
use dom::{DefaultDom, Wet};
use node::element::{Const, GenericElement};
#[doc(inline)]
pub use silkenweb_base::clone;
use silkenweb_base::document as base_document;
/// Define `&str` constants for each class in a CSS file.
///
/// This defines 2 modules:
///
/// - `mod class` with constants or functions (depending on `auto_mount`) for
///   each CSS class. For a CSS class called `my-css-class`, a constant called
///   `MY_CSS_CLASS` or a function called `my_css_class` will be defined.
/// - `mod stylesheet` with:
///     - An `fn text() -> &'static str` that gets the content of the
///       stylesheet.
///     - An `fn mount()` that lazily calls [`DefaultDom::mount_in_head`] once,
///       to ensure the stylesheet is in the head.
///
/// The macro takes two forms. Firstly it can take a single string literal which
/// is the path to the CSS/SCSS/SASS file. The path is relative to the
/// `$CARGO_MANIFEST_DIR` environment variable.
///
/// Alternatively, named parameters can be specified.
///
/// # Parameters
///
/// Parameters take the form:
///
/// ```
/// # use silkenweb_macros::css;
/// css!(
///     path = "my-css-file.css",
///     prefix = "prefix",
///     include_prefixes = ["included-"],
///     exclude_prefixes = ["excluded-"],
///     validate,
///     auto_mount,
///     transpile = (
///         minify,
///         pretty,
///         modules,
///         nesting,
///         browsers = (
///             android = (1, 0, 0),
///             chrome = (1, 0, 0),
///             edge = (1, 0, 0),
///             firefox = (1, 0, 0),
///             ie = (1, 0, 0),
///             ios_saf = (1, 0, 0),
///             opera = (1, 0, 0),
///             safari = (1, 0, 0),
///             samsung = (1, 0, 0),
///         )
///     )
/// );
/// ```
///
/// All are optional, but one of `path` or `content` must be specified.
///
/// - `path` is the path to the CSS /SCSS/SASS file.
/// - `content` is the css content.
/// - `prefix`: only classes starting with `prefix` should be included. Their
///   Rust names will have the prefix stripped.
/// - `include_prefixes`: a list of prefixes to include, without stripping the
///   prefix. Rust constants will only be defined for classes starting with one
///   or more of these prefixes.
/// - `exclude_prefixes`: a list of prefixes to exclude. No Rust constants will
///   be defined for a class starting with any of these prefixes.
///   `exclude_prefixes` takes precedence over `include_prefixes`.
/// - `validate`: validate the CSS.
/// - `auto_mount`: Generate a function for each CSS class that will call
///   `stylesheet::mount` before returning the class name.
/// - `transpile`: transpile the CSS with [lightningcss].
///
/// ## `transpile`
///
/// - `minify`: Minify the CSS returned by `stylesheet()`. Minification also
///   adds/removes vendor prefixes, so it's a good idea to keep this the same
///   between debug and release builds. Use `pretty` if you want legible CSS in
///   debug.
/// - `pretty`: Pretty print the final output. This is the default unless minify
///   is specified.
/// - `modules`: Enable [CSS Modules] to locally scope class identifiers, via
///   [lightningcss]. Composition is unsupported.
/// - `nesting`: Allow CSS nesting.
/// - `browsers` is a comma seperated list of the minimum supported browser
///   versions. This will add vendor prefixes to the CSS from `stylesheet()`.
///   The version is a paranthesized `,` seperated string of major, minor, and
///   patch versions. For example, to support firefox 110  + and chrome 111+,
///   use `browsers =( firefox = (110, 0, 0), chrome = (111, 0, 0) )`.
///
/// # Examples
///
/// Define private constants for all CSS classes:
///
/// ```
/// # use silkenweb_macros::css;
/// css!("my-css-file.css");
/// assert_eq!(class::MY_CLASS, "my-class");
/// ```
///
/// Define private constants for all content CSS classes:
///
///  ```
/// # use silkenweb_macros::css;
/// css!(content = r#"
///     .my-class {
///         color: hotpink;
///     }
/// "#);
/// assert_eq!(class::MY_CLASS, "my-class");
/// assert_eq!(stylesheet::text(), r#"
///     .my-class {
///         color: hotpink;
///     }
/// "#);
/// ```
/// 
/// Include classes starting with `border-`, except classes starting with
/// `border-excluded-`:
/// ```
/// # use silkenweb_macros::css;
/// css!(
///     path = "my-css-file.css",
///     prefix = "border-",
///     exclude_prefixes = ["border-excluded-"]
/// );
///
/// assert_eq!(class::SMALL, "border-small");
/// ```
/// 
/// This won't compile because `exclude_prefixes` takes precedence over
/// `include_prefixes`:
/// ```compile_fail
/// # use silkenweb_macros::css;
/// css!(
///     path = "my-css-file.css",
///     include_prefixes = ["border-"]
///     exclude_prefixes = ["border-excluded-"]
/// );
///
/// assert_eq!(class::BORDER_EXCLUDED_HUGE, "border-excluded-huge");
/// ```
/// 
/// [lightningcss]: https://lightningcss.dev/
/// [`DefaultDom::mount_in_head`]: crate::dom::DefaultDom::mount_in_head
/// [CSS Modules]: https://github.com/css-modules/css-modules
pub use silkenweb_macros::css;
/// Derive the traits needed for a blanket implmenetation of [`ChildElement`].
///
/// This only works for structs. It will defer to one field for the
/// implementation of the traits. If multiple fields are present, a target field
/// must be specified with `#[child_element(target)]`.
///
/// # Example
///
/// Derive traits for a newtype struct:
///
/// ```
/// # use silkenweb::{ChildElement, dom::InstantiableDom, node::Component};
/// #[derive(ChildElement)]
/// struct MyComponent<D: InstantiableDom>(Component<D>);
/// ```
///
/// Derive traits when the struct has more than 1 field:
///
/// ```
/// # use silkenweb::{ChildElement, dom::InstantiableDom, node::Component};
/// #[derive(ChildElement)]
/// struct MyComponent<D: InstantiableDom, Data> {
///     #[child_element(target)]
///     component: Component<D>,
///     data: Data,
/// }
/// ```
///
/// [`ChildElement`]: crate::node::element::ChildElement
pub use silkenweb_macros::ChildElement;
/// Derive [`Element`].
///
/// This only works for structs. It will defer to one field for the
/// implementation. If multiple fields are present, a target field must be
/// specified with `#[element(target)]`.
///
/// # Example
///
/// Derive traits for a newtype struct:
///
/// ```
/// # use silkenweb::{dom::Dom, elements::html::Div, Element};
/// #
/// #[derive(Element)]
/// struct MyElement<D: Dom>(Div<D>);
/// ```
///
/// When the struct has more than 1 field:
///
/// ```
/// # use silkenweb::{dom::Dom, elements::html::Div, Element};
/// #
/// #[derive(Element)]
/// struct MyElement<D: Dom, Data> {
///     #[element(target)]
///     element: Div<D>,
///     data: Data,
/// }
/// ```
///
/// [`Element`]: crate::node::element::Element
pub use silkenweb_macros::{cfg_browser, Element};
#[doc(inline)]
pub use silkenweb_macros::{AriaElement, ElementEvents, HtmlElement, HtmlElementEvents, Value};

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod animation;
pub mod attribute;
pub mod document;
pub mod dom;
pub mod elements;
pub mod hydration;
pub mod node;
pub mod router;
pub mod storage;
pub mod stylesheet;
pub mod task;

/// Commonly used imports, all in one place.
pub mod prelude {
    pub use futures_signals::{
        signal::{Mutable, Signal, SignalExt},
        signal_vec::{MutableVec, SignalVec, SignalVecExt},
    };

    pub use crate::{
        clone,
        elements::{html, svg, AriaElement, ElementEvents, HtmlElement, HtmlElementEvents},
        mount,
        node::{
            element::{Element, ParentElement, ShadowRootParent},
            Node,
        },
        value::Sig,
    };
}

pub use silkenweb_signals_ext::value;

/// Shorthand for [`DefaultDom::mount`]
pub fn mount(id: &str, element: impl Into<GenericElement<DefaultDom, Const>>) -> MountHandle {
    DefaultDom::mount(id, element)
}

fn mount_point(id: &str) -> web_sys::Element {
    base_document::get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{id}' must exist"))
}

fn insert_element(element: GenericElement<Wet, Const>) -> u128 {
    let id = next_node_handle_id();
    ELEMENTS.with(|elements| elements.borrow_mut().insert(id, element));
    id
}

fn remove_element(id: u128) -> Option<GenericElement<Wet, Const>> {
    ELEMENTS.with(|elements| elements.borrow_mut().remove(&id))
}

fn next_node_handle_id() -> u128 {
    ELEMENT_HANDLE_ID.with(|id| id.replace_with(|id| *id + 1))
}

thread_local!(
    static ELEMENT_HANDLE_ID: RefCell<u128> = RefCell::new(0);
    static ELEMENTS: RefCell<HashMap<u128, GenericElement<Wet, Const>>> =
        RefCell::new(HashMap::new());
);

#[cfg_browser(true)]
pub fn intern_str(s: &str) -> &str {
    wasm_bindgen::intern(s)
}

#[cfg_browser(true)]
pub fn empty_str() -> &'static str {
    thread_local! {
        static EMPTY: &'static str = intern_str("");
    }

    EMPTY.with(|empty| *empty)
}

#[cfg_browser(false)]
pub fn intern_str(s: &str) -> &str {
    s
}

#[cfg_browser(false)]
pub fn empty_str() -> &'static str {
    ""
}
