//! A library for building reactive web apps
//!
//! # Quick Start
//!
//! First off, we'll need [trunk] to build our app. Install it with:
//!
//! ```bash
//! cargo install trunk
//! ```
//!
//! Once that's completed, lets jump right in and have a play around with the
//! example counter app. The full code is available [here][counter]. To run it:
//!
//! ```bash
//! cd examples/counter
//! trunk serve --open
//! ```
//!
//! It's not the most complex app, but it'll serve as a good example to show how
//! we can build an interactive web app. Lets go through the code, step by step.
//!
//! Firstly, we create a new [`Mutable`] and an associated [`Signal`].
//!
//! ```rust
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//!
//! let count = Mutable::new(0);
//! let count_text = count.signal().map(|i| format!("{}", i));
//! ```
//!
//! [`Mutable`] represents a variable, and [`Signal`] represents values of that
//! variable across time. Here we `map` a function over values of `count`, to
//! convert it to text. See the [futures-signals tutorial] for more detail on
//! [`Mutable`]s and [`Signal`]s.
//!
//! Next, we create a closure, `inc`, to increment `count`. Then we build the
//! DOM for our counter. `on_click` installs `inc` as an event handler to
//! increment the counter.
//!
//! ```no_run
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//! #
//! # let count = Mutable::new(0);
//! # let count_text = count.signal().map(|i| format!("{}", i));
//!
//! let inc = move |_, _| {
//!     count.replace_with(|i| *i + 1);
//! };
//!
//! let app = div()
//!     .child(button().on_click(inc).text("+"))
//!     .child(p().text_signal(count_text));
//! ```
//!
//! Finally, we [`mount`] our app on the DOM. This will find the element with
//! `id = "app"` and mount `app` there.
//!
//! ```no_run
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//! #
//! # let count = Mutable::new(0);
//! # let count_text = count.signal().map(|i| format!("{}", i));
//! #
//! # let inc = move |_, _| {
//! #     count.replace_with(|i| *i + 1);
//! # };
//! #
//! # let app = div()
//! #     .child(button().on_click(inc).text("+"))
//! #     .child(p().text_signal(count_text));
//! mount("app", app);
//! ```
//!
//! # Cargo Features
//!
//! - `client-side-render` enables client side rendering on wasm32 targets
//! - `server-side-render` enables server side rendering on all targets
//! - `hydration` enables hydration on wasm32 clients
//!
//! If no features are specified, `client-side-rendering` will be enabled.
//!
//! [trunk]: https://trunkrs.dev/
//! [futures-signals tutorial]: https://docs.rs/futures-signals/0.3.24/futures_signals/tutorial/index.html
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [`Mutable`]: futures_signals::signal::Mutable
//! [`Signal`]: futures_signals::signal::Signal
//! [`mount`]: crate::mount
use std::{cell::RefCell, collections::HashMap};

use hydration::node::WetNode;
use node::Node;
#[doc(inline)]
pub use silkenweb_base::clone;
use silkenweb_base::document;
/// Newtype derive for [`ElementBuilder`].
///
/// Only non empty structs are supported. The first field must implement
/// [`ElementBuilder`].
///
/// [`ElementBuilder`]: crate::node::element::ElementBuilder
pub use silkenweb_macros::ElementBuilder;
pub use silkenweb_macros::{css, css_classes};
use wasm_bindgen::UnwrapThrowExt;

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod animation;
pub mod attribute;
pub mod elements;
pub mod hydration;
pub mod node;
pub mod router;
pub mod storage;
pub mod task;

/// Commonly used imports, all in one place.
pub mod prelude {
    pub use crate::{
        clone,
        elements::{ElementEvents, HtmlElement, HtmlElementEvents},
        mount,
        node::element::ParentBuilder,
    };
}

/// Mount an element on the document.
///
/// `id` is the html element id of the parent element. The element is added as
/// the last child of this element.
///
/// Mounting an `id` that is already mounted will remove that element.
pub fn mount(id: &str, node: impl Into<Node>) {
    let node = node.into();

    let mount_point = mount_point(id);
    mount_point
        .append_child(&node.eval_dom_node())
        .unwrap_throw();
    insert_component(id, mount_point.into(), node);
}

fn mount_point(id: &str) -> web_sys::Element {
    document::get_element_by_id(id).unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
}

fn insert_component(id: &str, parent: web_sys::Node, child: Node) {
    if let Some((parent, child)) =
        COMPONENTS.with(|apps| apps.borrow_mut().insert(id.to_owned(), (parent, child)))
    {
        parent.remove_child(&child.dom_node()).unwrap_throw();
    }
}

/// Unmount an element.
///
/// This is mostly useful for testing and checking for memory leaks
pub fn unmount(id: &str) {
    if let Some((parent, child)) = COMPONENTS.with(|apps| apps.borrow_mut().remove(id)) {
        parent.remove_child(&child.eval_dom_node()).unwrap_throw();
    }
}

thread_local!(
    static COMPONENTS: RefCell<HashMap<String, (web_sys::Node, Node)>> =
        RefCell::new(HashMap::new());
);
