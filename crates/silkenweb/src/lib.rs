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
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::option_if_let_else
)]

pub use silkenweb_dom::{
    element_list, local_storage, mount,
    render::{after_render, render_updates},
    session_storage, tag, tag_in_namespace, unmount, Builder, DomElement, Element, ElementBuilder,
};
pub use silkenweb_html::elements;
pub use silkenweb_reactive::{accumulators, clone, memo, signal};

pub mod animation;

pub mod router {
    //! URL based routing.
    //!
    //! Get the URL with [`url()`], and set it however you want to. For example:
    //! - with an anchor element like `<a href="/some/link">Some link</a>`
    //! - with [`set_url_path`].
    //!
    //! # Example
    //!
    //! ```no_run
    //! # use silkenweb::{
    //! #     elements::{button, div, p},
    //! #     mount, router,
    //! # };
    //! div()
    //!     .child(
    //!         button()
    //!             .on_click(|_, _| router::set_url_path("/route_1"))
    //!             .text("Go to route 1"),
    //!     )
    //!     .child(
    //!         button()
    //!             .on_click(|_, _| router::set_url_path("/route_2"))
    //!             .text("Go to route 2"),
    //!     )
    //!     .child(p().text(router::url().map(|url| format!("URL Path is: {}", url.pathname()))));
    //! ```
    pub use silkenweb_dom::router::{set_url_path, url};
}
