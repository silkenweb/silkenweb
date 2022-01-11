//! A library for building reactive single page web apps
//!
//! # Quick Start
//!
//! The best way to get started is to look at the examples. You'll need [trunk]
//! to run them. For example, to run [hello-world]:
//!
//! ```bash
//! cd examples/hello-world
//! trunk serve --open
//! ```
//!
//! - [hello-world] is a minimal example
//! - [counter] is a minimal interactive example
//! - [router] is a simple routing example
//! - [animation] is a simple animation example
//! - [todomvc] is an example of a simple app
//!
//! For a more complete introduction, see
//! [Learning Silkenweb With Entirely Too Many Counters](https://silkenweb.netlify.app/)
//!
//! [trunk]: https://trunkrs.dev/
//! [hello-world]: https://github.com/silkenweb/silkenweb/tree/main/examples/hello-world
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [router]: https://github.com/silkenweb/silkenweb/tree/main/examples/router
//! [animation]: https://github.com/silkenweb/silkenweb/tree/main/examples/animation
//! [todomvc]: https://github.com/silkenweb/silkenweb/tree/main/examples/todomvc
pub use silkenweb_dom::{
    attribute::{Attribute, AttributeValue},
    clone,
    element::{Element, ElementBuilder, GenericElementBuilder},
    mount, render,
    storage::Storage,
    tag, tag_in_namespace, unmount,
};
pub use silkenweb_html::{
    elements, html_element, parent_element, CustomEvent, Effects, ElementEvents, HtmlElement,
    HtmlElementEvents, ParentBuilder,
};
pub use silkenweb_signals_ext::SignalProduct;

pub mod animation;
pub mod router;
