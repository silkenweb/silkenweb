// TODO: Enable this warning
#![allow(dead_code, unused_variables)]
use std::future::Future;

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::JsValue;

use super::real::{RealElement, RealText};
use crate::{attribute::Attribute, global::document, spawn_cancelable_future};

pub struct VElement {
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

impl VElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document::create_element_ns(namespace, tag))
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
        Self {
            futures: Vec::new(),
        }
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.futures.push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        todo!()
    }

    pub fn store_child(&mut self, child: &mut Self) {
        todo!()
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        todo!()
    }

    pub fn append_child(&mut self, child: &mut impl VNode) {
        todo!()
    }

    pub fn insert_child_before(
        &mut self,
        child: &mut impl VNode,
        next_child: Option<&mut impl VNode>,
    ) {
        todo!()
    }

    pub fn replace_child(&mut self, new_child: &mut impl VNode, old_child: &mut impl VNode) {
        todo!()
    }

    pub fn remove_child(&mut self, child: &mut impl VNode) {
        todo!()
    }

    pub fn clear_children(&mut self) {
        todo!()
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        todo!()
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        todo!()
    }
}

impl From<VElement> for RealElement {
    fn from(_: VElement) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct VText(String);

impl VText {
    pub fn new(text: &str) -> Self {
        Self(text.to_owned())
    }

    pub fn set_text(&mut self, text: String) {
        self.0 = text;
    }
}

impl From<VText> for RealText {
    fn from(_: VText) -> Self {
        todo!()
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait VNode {}

impl VNode for VElement {}

impl VNode for VText {}
