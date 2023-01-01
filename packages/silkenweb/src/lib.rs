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
//! # use silkenweb::{elements::html::*, prelude::*, value::Sig};
//! #
//! # let count = Mutable::new(0);
//! # let count_text = count.signal().map(|i| format!("{}", i));
//!
//! let inc = move |_, _| {
//!     count.replace_with(|i| *i + 1);
//! };
//!
//! let app: Div = div()
//!     .child(button().on_click(inc).text("+"))
//!     .child(p().text(Sig(count_text)));
//! ```
//!
//! Finally, we [`mount`] our app on the DOM. This will find the element with
//! `id = "app"` and mount `app` there.
//!
//! ```no_run
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*, value::Sig};
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
//! #     .child(p().text(Sig(count_text)));
//! mount("app", app);
//! ```
//!
//! [trunk]: https://trunkrs.dev/
//! [futures-signals tutorial]: https://docs.rs/futures-signals/0.3.24/futures_signals/tutorial/index.html
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [`Mutable`]: futures_signals::signal::Mutable
//! [`Signal`]: futures_signals::signal::Signal
//! [`mount`]: crate::mount
use std::{cell::RefCell, collections::HashMap};

use dom::Wet;
use node::element::GenericElement;
#[doc(inline)]
pub use silkenweb_base::clone;
use silkenweb_base::document as base_document;
/// Newtype derive for [`Element`].
///
/// Only non empty structs are supported. The first field must implement
/// [`Element`].
///
/// [`Element`]: crate::node::element::Element
pub use silkenweb_macros::Element;
pub use silkenweb_macros::{css, css_classes};
#[doc(inline)]
pub use silkenweb_macros::{AriaElement, ElementEvents, HtmlElement, HtmlElementEvents, Value};
use wasm_bindgen::UnwrapThrowExt;

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
pub mod task;

/// Commonly used imports, all in one place.
pub mod prelude {
    pub use crate::{
        clone,
        elements::{ElementEvents, HtmlElement, HtmlElementEvents},
        mount,
        node::element::ParentElement,
    };
}

pub use silkenweb_signals_ext::value;

/// Mount an element on the document.
///
/// `id` is the html element id of the parent element. The element is added as
/// the last child of this element.
///
/// Mounting an `id` that is already mounted will remove that element.
pub fn mount(id: &str, element: impl Into<GenericElement<Wet>>) -> MountHandle {
    let mut element = element.into();

    let mount_point = mount_point(id);
    element.mount(&mount_point);
    MountHandle::new(mount_point, element)
}

pub fn remove_all_mounted() {
    ELEMENTS.with(|elements| {
        for element in elements.take().into_values() {
            element.dom_element().remove()
        }
    });
}

pub struct MountHandle {
    id: u128,
    mount_point: web_sys::Element,
    on_drop: Option<DropAction>,
}

impl MountHandle {
    pub fn new(mount_point: web_sys::Element, element: GenericElement<Wet>) -> Self {
        Self {
            id: insert_element(element),
            mount_point,
            on_drop: None,
        }
    }

    pub fn stop(mut self) {
        self.stop_on_drop();
    }

    pub fn stop_on_drop(&mut self) {
        self.on_drop = Some(DropAction::Stop);
    }

    pub fn unmount(mut self) {
        self.unmount_on_drop();
    }

    pub fn unmount_on_drop(&mut self) {
        self.on_drop = Some(DropAction::Unmount);
    }
}

impl Drop for MountHandle {
    fn drop(&mut self) {
        match self.on_drop {
            Some(DropAction::Stop) => {
                remove_element(self.id);
            }
            Some(DropAction::Unmount) => {
                if let Some(element) = remove_element(self.id) {
                    element
                        .dom_element()
                        .replace_with_with_node_1(&self.mount_point)
                        .unwrap_throw();
                }
            }
            None => (),
        }
    }
}

enum DropAction {
    Stop,
    Unmount,
}

fn mount_point(id: &str) -> web_sys::Element {
    base_document::get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{id}' must exist"))
}

fn insert_element(element: GenericElement<Wet>) -> u128 {
    let id = next_node_handle_id();
    ELEMENTS.with(|elements| elements.borrow_mut().insert(id, element));
    id
}

fn remove_element(id: u128) -> Option<GenericElement<Wet>> {
    ELEMENTS.with(|elements| elements.borrow_mut().remove(&id))
}

fn next_node_handle_id() -> u128 {
    ELEMENT_HANDLE_ID.with(|id| id.replace_with(|id| *id + 1))
}

thread_local!(
    static ELEMENT_HANDLE_ID: RefCell<u128> = RefCell::new(0);
    static ELEMENTS: RefCell<HashMap<u128, GenericElement<Wet>>> = RefCell::new(HashMap::new());
);
