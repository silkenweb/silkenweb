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

use std::marker::PhantomData;

use wasm_bindgen::JsCast;
use web_sys as dom;

#[macro_use]
pub mod macros;
pub mod elements;

/// Wrap a [`web_sys::CustomEvent`] and cast detail.
#[derive(Clone)]
pub struct CustomEvent<T>(dom::CustomEvent, PhantomData<T>);

impl<T: JsCast> CustomEvent<T> {
    /// The original event.
    pub fn event(&self) -> &dom::CustomEvent {
        &self.0
    }

    /// The event detail, downcast into `T`.
    ///
    /// # Panics
    ///
    /// If the downcast fails.
    pub fn detail(&self) -> T {
        self.0.detail().dyn_into().unwrap()
    }
}

impl<T> From<dom::CustomEvent> for CustomEvent<T> {
    fn from(src: dom::CustomEvent) -> Self {
        Self(src, PhantomData)
    }
}
