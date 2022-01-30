//! A reactive interface to the DOM.
use std::{cell::RefCell, collections::HashMap, fmt, future::Future};

use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{
    global::document,
    node::element::{Element, ElementBuilderBase},
    render::queue_update,
};

mod event;
mod hydration;
mod macros;

pub mod attribute;
pub mod global;
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

pub trait HydrationTracker {
    fn node_added(&mut self, elem: &web_sys::Node);

    fn node_removed(&mut self, node: &web_sys::Node);

    fn attribute_set(&mut self, elem: &web_sys::Element, name: &str, value: &str);

    fn attribute_removed(&mut self, elem: &web_sys::Element, name: &str);

    fn finished(self);
}

#[derive(Default)]
pub struct HydrationStats {
    nodes_added: u64,
    nodes_removed: u64,
    empty_text_removed: u64,
    attributes_set: u64,
    attributes_removed: u64,
}

impl HydrationStats {
    pub fn only_whitespace_diffs(&self) -> bool {
        self.nodes_added == 0
            && self.nodes_removed == 0
            && self.attributes_set == 0
            && self.attributes_removed == 0
    }

    pub fn exact_match(&self) -> bool {
        self.empty_text_removed == 0 && self.only_whitespace_diffs()
    }
}

impl HydrationTracker for HydrationStats {
    fn node_added(&mut self, _elem: &web_sys::Node) {
        self.nodes_added += 1;
    }

    fn node_removed(&mut self, node: &web_sys::Node) {
        match node
            .dyn_ref::<web_sys::Text>()
            .and_then(|t| t.text_content())
        {
            Some(text) if text.trim().is_empty() => self.empty_text_removed += 1,
            _ => self.nodes_removed += 1,
        }
    }

    fn attribute_set(&mut self, _elem: &web_sys::Element, _name: &str, _value: &str) {
        self.attributes_set += 1;
    }

    fn attribute_removed(&mut self, _elem: &web_sys::Element, _name: &str) {
        self.attributes_removed += 1;
    }

    fn finished(self) {
        web_log::println!("{}", self);
    }
}

impl fmt::Display for HydrationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Hydration stats:")?;
        writeln!(f, "    nodes added = {}", self.nodes_added)?;
        writeln!(f, "    nodes removed = {}", self.nodes_removed)?;
        writeln!(f, "    empty text removed = {}", self.empty_text_removed)?;
        writeln!(f, "    attributes set = {}", self.attributes_set)?;
        writeln!(f, "    attributes removed = {}", self.attributes_removed)
    }
}

pub fn hydrate_tracked(
    id: &str,
    elem: impl Into<Element>,
    mut tracker: impl HydrationTracker + 'static,
) {
    let id = id.to_owned();
    let elem = elem.into();

    queue_update(move || {
        unmount(&id);

        let mount_point = mount_point(&id);

        if let Some(hydration_point) = mount_point.first_child() {
            let node: web_sys::Node = elem
                .hydrate_child(&mount_point, &hydration_point, &mut tracker)
                .into();

            remove_following_siblings(&hydration_point, &node);
        } else {
            let new_elem = elem.eval_dom_element();
            tracker.node_added(&new_elem);
            mount_point.append_child(&new_elem).unwrap_throw();
        }

        insert_component(&id, elem);
        tracker.finished();
    });
}

pub fn hydrate(id: &str, elem: impl Into<Element>) {
    hydrate_tracked(id, elem, EmptyHydrationTracker)
}

struct EmptyHydrationTracker;

impl HydrationTracker for EmptyHydrationTracker {
    fn node_added(&mut self, _elem: &web_sys::Node) {}

    fn node_removed(&mut self, _node: &web_sys::Node) {}

    fn attribute_set(&mut self, _elem: &web_sys::Element, _name: &str, _value: &str) {}

    fn attribute_removed(&mut self, _elem: &web_sys::Element, _name: &str) {}

    fn finished(self) {}
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

#[cfg(target_arch = "wasm32")]
pub fn intern_str(s: &str) -> &str {
    use wasm_bindgen::intern;
    intern(s);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn intern_str(s: &str) -> &str {
    s
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
mod tasks {
    use std::future::Future;

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future)
    }
}
