//! A reactive interface to the DOM.
use std::{cell::RefCell, collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use element::{Element, ElementBuilderBase};
use futures_signals::{cancelable_future, CancelableFutureHandle};
use global::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;

mod macros;

pub mod attribute;
pub mod element;
pub mod global;
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

pub fn hydrate(id: &str, elem: impl Into<Element>) {
    unmount(id);
    let elem = elem.into();

    let mount_point = mount_point(id);

    // TODO: Ignore whitespace text nodes?
    // TODO: Empty text is a problem. `b().text("")` and `b()` both render as
    // `<b></b>', but the former contains an empty text node in the dom, whereas the
    // latter doesn't.
    if let Some(hydration_point) = mount_point.first_child() {
        // TODO: Replace first child if it's the wrong type
        // TODO: Remove any other children
        elem.hydrate(hydration_point.dyn_into().expect("Unexpected node type"));
    } else {
        mount_point
            .append_child(&elem.eval_dom_element())
            .unwrap_throw();
    }

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

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: &str) -> ElementBuilderBase {
    ElementBuilderBase::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: &str, name: &str) -> ElementBuilderBase {
    ElementBuilderBase::new_in_namespace(namespace, name)
}

fn spawn_cancelable_future(
    future: impl Future<Output = ()> + 'static,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    spawn_local(cancelable_future);

    handle
}

thread_local!(
    static COMPONENTS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
