use std::{
    cell::{Ref, RefCell, RefMut},
    future::Future,
    mem,
    rc::Rc,
};

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::event::EventCallback;
use crate::{
    attribute::Attribute,
    clone,
    global::document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

// TODO: Parameterize on element type (web_sys::{Element | Node})
#[derive(Clone)]
pub struct StrictElement(Rc<RefCell<StrictElementData>>);

impl StrictElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(&document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(&document::create_element_ns(namespace, tag))
    }

    pub fn new_text(text: &str) -> Self {
        Self::new_element(&document::create_text_node(text))
    }

    fn new_element(dom_element: &web_sys::Node) -> Self {
        Self(Rc::new(RefCell::new(StrictElementData {
            dom_element: dom_element.clone(),
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
            .push(EventCallback::new(dom_element, name, f));
    }

    pub fn set_text(&self, text: String) {
        let node = self.data().dom_element.clone();

        queue_update(move || node.set_text_content(Some(&text)));
    }

    // TODO: How to distinguish between `add_child` and `append_child`
    pub fn add_child(&self, child: Self) {
        let mut data = self.data_mut();
        let mut child_data = child.data_mut();
        data.event_callbacks
            .extend(mem::take(&mut child_data.event_callbacks));
        data.futures.extend(mem::take(&mut child_data.futures));
    }

    pub fn append_child_now(&self, child: &Self) {
        self.data()
            .dom_element
            .append_child(&child.data().dom_element)
            .unwrap_throw();
    }

    pub fn insert_child_before(&self, child: &Self, next_child: Option<&Self>) {
        let parent = self.clone();
        let next_child = next_child.cloned();
        clone!(child);

        queue_update(move || {
            parent.insert_child_before_now(&child, next_child.as_ref());
        });
    }

    pub fn insert_child_before_now(&self, child: &Self, next_child: Option<&Self>) {
        if let Some(next_child) = next_child {
            self.data()
                .dom_element
                .insert_before(
                    &child.data().dom_element,
                    Some(&next_child.data().dom_element),
                )
                .unwrap_throw();
        } else {
            self.append_child_now(child);
        }
    }

    pub fn replace_child(&self, new_child: &Self, old_child: &Self) {
        let parent = self.data().dom_element.clone();
        let new_child = new_child.data().dom_element.clone();
        let old_child = old_child.data().dom_element.clone();

        queue_update(move || {
            parent.replace_child(&new_child, &old_child).unwrap_throw();
        });
    }

    pub fn remove_child_now(&self, child: &Self) {
        self.data()
            .dom_element
            .remove_child(&child.data().dom_element)
            .unwrap_throw();
    }

    pub fn remove_child(&self, child: &Self) {
        let parent = self.data().dom_element.clone();
        let child = child.data().dom_element.clone();

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
        value.set_attribute(name, self.data().dom_element.dyn_ref().unwrap_throw());
    }

    pub fn effect(&self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self
            .data()
            .dom_element
            .dyn_ref::<web_sys::Element>()
            .unwrap_throw()
            .clone();
        after_render(move || f(&dom_element));
    }

    pub fn eval_dom_element(&self) -> web_sys::Node {
        self.data().dom_element.clone()
    }

    fn data(&self) -> Ref<StrictElementData> {
        self.0.as_ref().borrow()
    }

    fn data_mut(&self) -> RefMut<StrictElementData> {
        self.0.as_ref().borrow_mut()
    }
}

struct StrictElementData {
    pub(super) dom_element: web_sys::Node,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}
