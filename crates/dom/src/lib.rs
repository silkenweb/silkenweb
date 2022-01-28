//! A reactive interface to the DOM.
use std::{cell::RefCell, collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use element::{Element, ElementBuilderBase};
use futures_signals::{cancelable_future, CancelableFutureHandle};
use global::document;
use wasm_bindgen::UnwrapThrowExt;

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

    if let Some(hydration_point) = mount_point.first_child() {
        let node: web_sys::Node = elem.hydrate_child(&mount_point, &hydration_point).into();

        remove_following_siblings(&hydration_point, &node);
    } else {
        mount_point
            .append_child(&elem.eval_dom_element())
            .unwrap_throw();
    }

    insert_component(id, elem);
}

/// Remove all siblings after `child`
///
/// `child` itself is not removed, only the siblings following.
fn remove_following_siblings(parent: &web_sys::Node, child: &web_sys::Node) {
    if let Some(mut node) = child.next_sibling() {
        while let Some(next_node) = node.next_sibling() {
            parent.remove_child(&node).unwrap_throw();
            node = next_node;
        }
    }
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

    tasks::spawn_local(cancelable_future);

    handle
}

thread_local!(
    static COMPONENTS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);

#[cfg(feature = "server-render")]
mod tasks {
    use std::{cell::RefCell, future::Future};

    use futures::{
        executor::{LocalPool, LocalSpawner},
        task::LocalSpawnExt,
    };

    thread_local!(
        static EXECUTOR: RefCell<LocalPool> = RefCell::new(LocalPool::new());
        static SPAWNER: LocalSpawner = EXECUTOR.with(|executor| executor.borrow().spawner());
    );

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        SPAWNER.with(|spawner| {
            spawner.spawn_local(future).unwrap();
        });
    }

    pub fn run() {
        EXECUTOR.with(|executor| executor.borrow_mut().run_until_stalled())
    }
}

#[cfg(not(feature = "server-render"))]
mod tasks {
    use std::future::Future;

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future)
    }
}
