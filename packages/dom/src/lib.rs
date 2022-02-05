//! A reactive interface to the DOM.
use std::{cell::RefCell, collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
use wasm_bindgen::UnwrapThrowExt;

use crate::{global::document, node::element::Element};

mod event;
mod macros;

pub mod attribute;
pub mod global;
pub mod hydration;
pub mod node;
pub mod render;

/// Mount an element on the document.
///
/// `id` is the html element id of the parent element. The element is added as
/// the last child of this element.
///
/// Mounting an `id` that is already mounted will remove that element.
///
/// An [`Element`] can only appear once in the document. Adding an [`Element`]
/// to the document a second time will move it. It will still require
/// unmounting from both places to free up any resources.
pub fn mount(id: &str, elem: impl Into<Element>) {
    unmount(id);
    let elem = elem.into();

    mount_point(id)
        .append_child(&elem.eval_dom_element())
        .unwrap_throw();
    insert_component(id, elem);
}

fn mount_point(id: &str) -> web_sys::Element {
    document::get_element_by_id(id).unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
}

fn insert_component(id: &str, elem: Element) {
    COMPONENTS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));
}

/// Unmount an element.
///
/// This is mostly useful for testing and checking for memory leaks
pub fn unmount(id: &str) {
    if let Some(elem) = COMPONENTS.with(|apps| apps.borrow_mut().remove(id)) {
        elem.eval_dom_element().remove();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn intern_str(s: &str) -> &str {
    use wasm_bindgen::intern;
    intern(s)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn intern_str(s: &str) -> &str {
    s
}

fn spawn_cancelable_future(
    future: impl Future<Output = ()> + 'static,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    render::spawn_local(cancelable_future);

    handle
}

thread_local!(
    static COMPONENTS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
