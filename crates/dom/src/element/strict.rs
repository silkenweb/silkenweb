use std::{
    cell::{Ref, RefCell, RefMut},
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
    global::document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

#[derive(Clone)]
pub struct StrictElement(Rc<RefCell<StrictElementData>>);

struct StrictElementData {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

impl StrictElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document::create_element_ns(namespace, tag))
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
        Self(Rc::new(RefCell::new(StrictElementData {
            dom_element,
            event_callbacks: Vec::new(),
            futures: Vec::new(),
        })))
    }

    fn data(&self) -> Ref<StrictElementData> {
        self.0.borrow()
    }

    fn data_mut(&mut self) -> RefMut<StrictElementData> {
        self.0.borrow_mut()
    }

    pub fn shrink_to_fit(&mut self) {
        let mut data = self.data_mut();
        data.event_callbacks.shrink_to_fit();
        data.futures.shrink_to_fit();
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.data_mut()
            .futures
            .push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        let mut data = self.data_mut();
        let dom_element = data.dom_element.clone();
        data.event_callbacks
            .push(EventCallback::new(dom_element.into(), name, f));
    }

    pub fn store_child(&mut self, mut child: Self) {
        let mut data = self.data_mut();
        let mut child = child.data_mut();
        data.event_callbacks
            .extend(mem::take(&mut child.event_callbacks));
        data.futures.extend(mem::take(&mut child.futures));
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.data().dom_element.clone()
    }

    pub fn append_child_now(&mut self, child: &mut impl StrictNodeRef) {
        self.dom_element()
            .append_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn insert_child_before(
        &mut self,
        mut child: StrictNode,
        mut next_child: Option<StrictNode>,
    ) {
        let mut parent = self.clone();

        queue_update(move || {
            parent.insert_child_before_now(&mut child, next_child.as_mut());
        });
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl StrictNodeRef,
        next_child: Option<&mut impl StrictNodeRef>,
    ) {
        if let Some(next_child) = next_child {
            self.dom_element()
                .insert_before(&child.dom_node(), Some(&next_child.dom_node()))
                .unwrap_throw();
        } else {
            self.append_child_now(child);
        }
    }

    pub fn replace_child(&mut self, new_child: StrictNode, old_child: StrictNode) {
        let parent = self.dom_element().clone();
        let new_child = new_child.dom_node().clone();
        let old_child = old_child.dom_node().clone();

        queue_update(move || {
            parent.replace_child(&new_child, &old_child).unwrap_throw();
        });
    }

    pub fn remove_child_now(&mut self, child: &mut impl StrictNodeRef) {
        self.dom_element()
            .remove_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn remove_child(&mut self, child: StrictNode) {
        let parent = self.dom_element().clone();

        queue_update(move || {
            parent.remove_child(&child.dom_node()).unwrap_throw();
        });
    }

    pub fn clear_children(&mut self) {
        let parent = self.dom_element().clone();

        queue_update(move || {
            // This is specified to remove all nodes, if I'm reading it correctly:
            // <https://dom.spec.whatwg.org/#dom-node-textcontent>
            parent.set_text_content(None);
        })
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        value.set_attribute(name, &self.dom_element());
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.dom_element().clone();
        after_render(move || f(&dom_element));
    }

    fn dom_element(&self) -> Ref<web_sys::Element> {
        Ref::map(self.data(), |data| &data.dom_element)
    }
}

#[derive(Clone)]
pub struct StrictText(RefCell<web_sys::Text>);

impl StrictText {
    pub fn new(text: &str) -> Self {
        Self(RefCell::new(document::create_text_node(text)))
    }

    pub fn set_text(&mut self, text: String) {
        let node = self.0.borrow().clone();

        queue_update(move || node.set_text_content(Some(&text)));
    }
}

#[derive(Clone)]
pub struct StrictNode(StrictNodeEnum);

impl StrictNode {
    fn dom_node(&self) -> Ref<web_sys::Node> {
        match &self.0 {
            StrictNodeEnum::Element(elem) => Ref::map(elem.dom_element(), AsRef::as_ref),
            StrictNodeEnum::Text(text) => Ref::map(text.0.borrow(), AsRef::as_ref),
        }
    }
}

#[derive(Clone)]
enum StrictNodeEnum {
    Element(StrictElement),
    Text(StrictText),
}

impl From<StrictElement> for StrictNode {
    fn from(elem: StrictElement) -> Self {
        Self(StrictNodeEnum::Element(elem))
    }
}

impl From<StrictText> for StrictNode {
    fn from(text: StrictText) -> Self {
        Self(StrictNodeEnum::Text(text))
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait StrictNodeRef: Clone + Into<StrictNode> {
    fn dom_node(&self) -> Ref<web_sys::Node>;
}

impl StrictNodeRef for StrictNode {
    fn dom_node(&self) -> Ref<web_sys::Node> {
        self.dom_node()
    }
}

impl StrictNodeRef for StrictElement {
    fn dom_node(&self) -> Ref<web_sys::Node> {
        Ref::map(self.dom_element(), AsRef::as_ref)
    }
}

impl StrictNodeRef for StrictText {
    fn dom_node(&self) -> Ref<web_sys::Node> {
        Ref::map(self.0.borrow(), AsRef::as_ref)
    }
}
