use std::{
    cell::{RefCell, RefMut},
    future::Future,
    mem,
    rc::Rc,
};

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use super::event::EventCallback;
use crate::{
    attribute::Attribute,
    clone,
    global::document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

pub struct StrictElement {
    dom_element: StrictNode<web_sys::Element>,
    handles: Rc<RefCell<ElementHandles>>,
}

impl StrictElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document::create_element_ns(namespace, tag))
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
        Self {
            dom_element: StrictNode(dom_element),
            handles: Rc::new(RefCell::new(ElementHandles {
                event_callbacks: Vec::new(),
                futures: Vec::new(),
            })),
        }
    }

    pub fn shrink_to_fit(&mut self) {
        let mut data = self.handles_mut();
        data.event_callbacks.shrink_to_fit();
        data.futures.shrink_to_fit();
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.handles_mut()
            .futures
            .push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        let mut data = self.handles_mut();
        let dom_element = self.dom_element().clone();
        data.event_callbacks
            .push(EventCallback::new(dom_element.into(), name, f));
    }

    pub fn store_child(&self, child: Self) {
        let mut data = self.handles_mut();
        let mut child_data = child.handles_mut();
        data.event_callbacks
            .extend(mem::take(&mut child_data.event_callbacks));
        data.futures.extend(mem::take(&mut child_data.futures));
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.dom_element().clone()
    }

    fn handles_mut(&self) -> RefMut<ElementHandles> {
        self.handles.as_ref().borrow_mut()
    }

    fn dom_element(&self) -> &web_sys::Element {
        &self.dom_element.0
    }
}

#[derive(Clone)]
pub struct StrictNode<T>(T);

impl<T: AsRef<web_sys::Node> + Clone + 'static> StrictNode<T> {
    pub fn append_child_now(&self, child: &impl StrictNodeRef) {
        self.dom_node()
            .append_child(child.as_node_ref().dom_node())
            .unwrap_throw();
    }

    pub fn insert_child_before(&self, child: StrictNodeBase, next_child: Option<StrictNodeBase>) {
        let parent = self.clone();

        queue_update(move || {
            parent.insert_child_before_now(&child, next_child.as_ref());
        });
    }

    pub fn insert_child_before_now(
        &self,
        child: &impl StrictNodeRef,
        next_child: Option<&impl StrictNodeRef>,
    ) {
        if let Some(next_child) = next_child {
            self.dom_node()
                .insert_before(
                    child.as_node_ref().dom_node(),
                    Some(next_child.as_node_ref().dom_node()),
                )
                .unwrap_throw();
        } else {
            self.append_child_now(child);
        }
    }

    pub fn replace_child(&self, new_child: StrictNodeBase, old_child: StrictNodeBase) {
        let parent = self.dom_node().clone();
        clone!(new_child, old_child);

        queue_update(move || {
            parent
                .replace_child(&new_child.0, &old_child.0)
                .unwrap_throw();
        });
    }

    pub fn remove_child_now(&self, child: &impl StrictNodeRef) {
        self.dom_node()
            .remove_child(child.as_node_ref().dom_node())
            .unwrap_throw();
    }

    pub fn remove_child(&self, child: StrictNodeBase) {
        let parent = self.dom_node().clone();

        queue_update(move || {
            parent.remove_child(&child.0).unwrap_throw();
        });
    }

    pub fn clear_children(&self) {
        let parent = self.dom_node().clone();

        queue_update(move || {
            // TODO: Is this the same as `set_inner_html`?
            parent.set_text_content(None);
        })
    }

    fn dom_node(&self) -> &web_sys::Node {
        self.0.as_ref()
    }
}

impl StrictNode<web_sys::Element> {
    pub fn attribute<A: Attribute>(&self, name: &str, value: A) {
        value.set_attribute(name, &self.0);
    }

    pub fn effect(&self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.0.clone();
        after_render(move || f(&dom_element));
    }
}

pub type StrictNodeBase = StrictNode<web_sys::Node>;

#[derive(Clone)]
pub struct StrictText(StrictNode<web_sys::Text>);

impl StrictText {
    pub fn new(text: &str) -> Self {
        Self(StrictNode(document::create_text_node(text)))
    }

    pub fn set_text(&self, text: String) {
        let node = self.0.clone();

        queue_update(move || node.0.set_text_content(Some(&text)));
    }
}

pub trait StrictNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> &StrictNode<Self::Node>;

    fn clone_into_node(&self) -> StrictNodeBase {
        StrictNode(self.as_node_ref().dom_node().clone())
    }
}

impl<T> StrictNodeRef for StrictNode<T>
where
    T: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static,
{
    type Node = T;

    fn as_node_ref(&self) -> &StrictNode<Self::Node> {
        self
    }
}

impl StrictNodeRef for StrictText {
    type Node = web_sys::Text;

    fn as_node_ref(&self) -> &StrictNode<Self::Node> {
        &self.0
    }
}

impl StrictNodeRef for StrictElement {
    type Node = web_sys::Element;

    fn as_node_ref(&self) -> &StrictNode<Self::Node> {
        &self.dom_element
    }
}

struct ElementHandles {
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}
