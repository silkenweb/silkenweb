// TODO: Enable warnings
#![allow(dead_code, unused_variables)]
use std::{cell::RefCell, future::Future, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::JsValue;

use super::strict::StrictElement;
use crate::{attribute::Attribute, spawn_cancelable_future};

pub struct DeferredElement {
    namespace: Option<String>,
    tag: String,
    children: Vec<Self>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    events: Vec<Box<dyn FnOnce(&mut StrictElement)>>,
}

// TODO: Use an Rc for data and derive this
impl Clone for DeferredElement {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl DeferredElement {
    pub fn new(tag: &str) -> Self {
        Self {
            namespace: None,
            tag: tag.to_owned(),
            children: Vec::new(),
            futures: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self {
            namespace: Some(namespace.to_owned()),
            tag: tag.to_owned(),
            children: Vec::new(),
            futures: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.futures.push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.events.push(Box::new(move |elem| elem.on(name, f)))
    }

    pub fn store_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn append_child(&mut self, child: impl Into<DeferredNode>) {
        todo!()
    }

    pub fn insert_child_before(
        &mut self,
        child: impl Into<DeferredNode>,
        next_child: Option<impl Into<DeferredNode>>,
    ) {
        todo!()
    }

    pub fn replace_child(
        &mut self,
        new_child: impl Into<DeferredNode>,
        old_child: impl Into<DeferredNode>,
    ) {
        todo!()
    }

    pub fn remove_child(&mut self, child: impl Into<DeferredNode>) {
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

impl From<DeferredElement> for StrictElement {
    fn from(_: DeferredElement) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct DeferredText(Rc<RefCell<String>>);

impl DeferredText {
    pub fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(text.to_owned())))
    }

    pub fn set_text(&mut self, text: String) {
        *self.0.borrow_mut() = text;
    }
}

// TODO: Rename to DeferredBaseNode
#[derive(Clone)]
pub struct DeferredNode(
    // TODO: This will contain an enum of Text|Element
);
