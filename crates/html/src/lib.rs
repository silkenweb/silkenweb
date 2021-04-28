#![allow(clippy::must_use_candidate)]
//! Builders for HTML elements.
//!
//! Each HTML element has a function, a struct and a builder struct. The
//! function is a constructor for the builder. The builder has methods for each
//! attribute for that element, as well as methods for each event. For example:
//!
//! ```no_run
//! # use silkenweb_html::elements::{a, A, ABuilder};
//! use web_sys as dom;
//! let link: ABuilder = a()
//!     .href("https://example.com/")
//!     .on_click(|event: dom::MouseEvent, link: dom::HtmlAnchorElement| {});
//! ```

#[macro_use]
pub mod macros;

#[rustfmt::skip]
pub mod elements;
