use std::{future::Future, mem};

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::{
    attribute::Attribute, element::event::EventCallback, global::document, render::after_render,
    spawn_cancelable_future,
};

pub struct RealElement {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

impl RealElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document::create_element_ns(namespace, tag))
    }

    pub fn shrink_to_fit(&mut self) {
        self.event_callbacks.shrink_to_fit();
        self.futures.shrink_to_fit();
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.futures.push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        let dom_element = self.dom_element.clone();
        self.event_callbacks
            .push(EventCallback::new(dom_element.into(), name, f));
    }

    pub fn store_child(&mut self, child: &mut Self) {
        self.event_callbacks
            .extend(mem::take(&mut child.event_callbacks));
        self.futures.extend(mem::take(&mut child.futures));
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.dom_element.clone()
    }

    pub fn append_child(&mut self, child: &mut impl RealNode) {
        self.dom_element
            .append_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn insert_child_before(
        &mut self,
        child: &mut impl RealNode,
        next_child: Option<&mut impl RealNode>,
    ) {
        if let Some(next_child) = next_child {
            self.dom_element
                .insert_before(&child.dom_node(), Some(&next_child.dom_node()))
                .unwrap_throw();
        } else {
            self.append_child(child);
        }
    }

    pub fn replace_child(&mut self, new_child: &mut impl RealNode, old_child: &mut impl RealNode) {
        self.dom_element
            .replace_child(&new_child.dom_node(), &old_child.dom_node())
            .unwrap_throw();
    }

    pub fn remove_child(&mut self, child: &mut impl RealNode) {
        self.dom_element
            .remove_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn clear_children(&mut self) {
        self.dom_element.set_inner_html("");
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        value.set_attribute(name, &self.dom_element);
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.dom_element.clone();
        after_render(move || f(&dom_element));
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
        Self {
            dom_element,
            event_callbacks: Vec::new(),
            futures: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct RealText(web_sys::Text);

impl RealText {
    pub fn new(text: &str) -> Self {
        Self(document::create_text_node(text))
    }

    pub fn set_text(&mut self, text: &str) {
        self.0.set_text_content(Some(text));
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait RealNode {
    fn dom_node(&self) -> web_sys::Node;
}

impl RealNode for RealElement {
    fn dom_node(&self) -> web_sys::Node {
        self.dom_element.clone().into()
    }
}

impl RealNode for RealText {
    fn dom_node(&self) -> web_sys::Node {
        self.0.clone().into()
    }
}
