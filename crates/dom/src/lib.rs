//! A reactive interface to the DOM.
pub mod render;
use std::{cell::RefCell, collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
use wasm_bindgen_futures::spawn_local;
use web_sys as dom;

pub mod macros;

mod attribute;
mod element;

pub use attribute::{Attribute, AttributeValue, StaticAttribute};
pub use element::{signal, Builder, DomElement, Element, ElementBuilder};

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

    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
        .append_child(elem.dom_element())
        .unwrap();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));
}

/// Unmount an element.
///
/// This is mostly useful for testing and checking for memory leaks
pub fn unmount(id: &str) {
    if let Some(elem) = APPS.with(|apps| apps.borrow_mut().remove(id)) {
        elem.dom_element().remove();
    }
}

pub fn window() -> dom::Window {
    dom::window().expect("Window must be available")
}

pub fn document() -> dom::Document {
    window().document().expect("Window must contain a document")
}

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: &str) -> ElementBuilder {
    ElementBuilder::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: &str, name: &str) -> ElementBuilder {
    ElementBuilder::new_in_namespace(namespace, name)
}

fn spawn_cancelable_future(
    future: impl 'static + Future<Output = ()>,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    spawn_local(cancelable_future);

    handle
}

// TODO: We probably want a better storage API.
// We want to be able to iterator over it like a map using Object::entries and
// Object::keys
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
pub fn local_storage() -> Option<dom::Storage> {
    // TODO: Under what circumstances can these fail?
    window().local_storage().unwrap()
}

/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
pub fn session_storage() -> Option<dom::Storage> {
    window().session_storage().unwrap()
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
