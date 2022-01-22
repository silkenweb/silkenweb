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
    clone,
    global::document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

// TODO: Split dom_elem and rest of data, so we can implement asref Node.
// TODO: Make node an opaque type, add a text type and make text + element asref to node.
#[derive(Clone)]
pub struct StrictElement(Rc<RefCell<StrictElementData>>);

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

    pub fn store_child(&self, child: Self) {
        let mut data = self.data_mut();
        let mut child_data = child.data_mut();
        data.event_callbacks
            .extend(mem::take(&mut child_data.event_callbacks));
        data.futures.extend(mem::take(&mut child_data.futures));
    }

    pub fn append_child_now(&self, child: &StrictNode) {
        self.data()
            .dom_element
            .append_child(&child.0)
            .unwrap_throw();
    }

    pub fn insert_child_before(&self, child: &StrictNode, next_child: Option<&StrictNode>) {
        let parent = self.clone();
        let next_child = next_child.cloned();
        clone!(child);

        queue_update(move || {
            parent.insert_child_before_now(&child, next_child.as_ref());
        });
    }

    pub fn insert_child_before_now(&self, child: &StrictNode, next_child: Option<&StrictNode>) {
        if let Some(next_child) = next_child {
            self.data()
                .dom_element
                .insert_before(&child.0, Some(&next_child.0))
                .unwrap_throw();
        } else {
            self.append_child_now(child);
        }
    }

    pub fn replace_child(&self, new_child: &StrictNode, old_child: &StrictNode) {
        let parent = self.data().dom_element.clone();
        let new_child = new_child.0.clone();
        let old_child = old_child.0.clone();

        queue_update(move || {
            parent.replace_child(&new_child, &old_child).unwrap_throw();
        });
    }

    pub fn remove_child_now(&self, child: &StrictNode) {
        self.data()
            .dom_element
            .remove_child(&child.0)
            .unwrap_throw();
    }

    pub fn remove_child(&self, child: &StrictNode) {
        let parent = self.data().dom_element.clone();
        let child = child.0.clone();

        queue_update(move || {
            parent.remove_child(&child).unwrap_throw();
        });
    }

    pub fn clear_children(&self) {
        let parent = self.data().dom_element.clone();

        queue_update(move || {
            // TODO: Is this the same as `set_inner_html`?
            parent.set_text_content(None);
        })
    }

    pub fn attribute<T: Attribute>(&self, name: &str, value: T) {
        value.set_attribute(name, &self.data().dom_element);
    }

    pub fn effect(&self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.data().dom_element.clone();
        after_render(move || f(&dom_element));
    }

    pub fn clone_into_node(&self) -> StrictNode {
        StrictNode(self.data().dom_element.clone().into())
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.data().dom_element.clone()
    }

    fn data(&self) -> Ref<StrictElementData> {
        self.0.as_ref().borrow()
    }

    fn data_mut(&self) -> RefMut<StrictElementData> {
        self.0.as_ref().borrow_mut()
    }
}

#[derive(Clone)]
pub struct StrictNode(web_sys::Node);

impl StrictNode {
    pub fn new_text(text: &str) -> Self {
        Self(document::create_text_node(text).into())
    }

    pub fn set_text(&self, text: String) {
        let node = self.0.clone();

        queue_update(move || node.set_text_content(Some(&text)));
    }
}

struct StrictElementData {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}
