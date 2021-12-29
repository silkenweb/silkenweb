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
use std::pin::Pin;

use futures_signals::signal::{Broadcaster, Signal, SignalExt};
pub use silkenweb_dom::{
    clone, mount, product,
    render::{after_render, render_updates},
    signal, tag, tag_in_namespace, unmount, AttributeValue, Builder, DomElement, Element,
    ElementBuilder, Storage,
};
pub use silkenweb_html::{
    children_allowed, elements, html_element, CustomEvent, Effects, HtmlElement, ParentBuilder,
};

pub mod animation;
pub mod router;

// TODO: Is this a sensible way to erase the type of the Signal?
// TODO: Put this in a `silkenweb-signal-util` crate?
pub struct BroadcastBoxed<T>(Broadcaster<Pin<Box<dyn Signal<Item = T>>>>);

impl<T: 'static> BroadcastBoxed<T> {
    pub fn new(sig: impl 'static + Signal<Item = T>) -> Self {
        Self(Broadcaster::new(sig.boxed_local()))
    }

    pub fn signal_ref<B, F>(&self, f: F) -> impl 'static + Signal<Item = B>
    where
        F: 'static + FnMut(&T) -> B,
    {
        self.0.signal_ref(f)
    }

    pub fn signal(&self) -> impl 'static + Signal<Item = T>
    where
        T: Copy,
    {
        self.0.signal()
    }

    pub fn signal_cloned(&self) -> impl 'static + Signal<Item = T>
    where
        T: Clone,
    {
        self.0.signal_cloned()
    }
}
