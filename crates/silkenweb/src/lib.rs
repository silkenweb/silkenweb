//! Silkenweb is a reactive web library for writing single page apps
//!
//! # Features
//!
//! - Fine grained reactivity using signals
//! - No VDOM
//! - Uses plain Rust syntax rather than a macro DSL
//!
//! # Example: A Simple Counter
//!
//! ```
//! use silkenweb::{
//!     elements::{button, div, p},
//!     mount,
//!     signal::Signal,
//! };
//!
//! # fn no_exec() {
//! fn main() {
//!     let count = Signal::new(0);
//!     let set_count = count.write();
//!     let inc = move |_, _| set_count.replace(|&i| i + 1);
//!     let count_text = count.read().map(|i| format!("{}", i));
//!
//!     let app = div()
//!         .child(button().on_click(inc).text("+"))
//!         .child(p().text(count_text));
//!
//!     mount("app", app);
//! }
//! # }
//! ```
pub use silkenweb_dom::{
    after_render, element_list, mount, render_updates, tag, unmount, Builder, DomElement, Element,
    ElementBuilder,
};
pub use silkenweb_html::elements;
pub use silkenweb_reactive::{accumulators, clone, memo, signal};
