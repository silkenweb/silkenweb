// TODO: Enable warnings
#![allow(dead_code, unused_variables)]
use std::future::Future;

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
}

#[derive(Clone)]
pub struct DeferredNode<T>(T);

impl DeferredNode<web_sys::Element> {
    pub fn append_child(&mut self, child: &mut impl DeferredNodeRef) {
        todo!()
    }

    pub fn insert_child_before(
        &mut self,
        child: &mut impl DeferredNodeRef,
        next_child: Option<&mut impl DeferredNodeRef>,
    ) {
        todo!()
    }

    pub fn replace_child(
        &mut self,
        new_child: &mut impl DeferredNodeRef,
        old_child: &mut impl DeferredNodeRef,
    ) {
        todo!()
    }

    pub fn remove_child(&mut self, child: &mut impl DeferredNodeRef) {
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

pub type DeferredNodeBase = DeferredNode<web_sys::Node>;

impl<T: Into<web_sys::Node>> DeferredNode<T> {
    pub fn into_base(self) -> DeferredNodeBase {
        todo!()
    }
}

#[derive(Clone)]
pub struct DeferredText(DeferredNode<web_sys::Text>);

impl DeferredText {
    pub fn new(text: &str) -> Self {
        todo!()
    }

    pub fn set_text(&mut self, text: String) {
        todo!()
    }
}

pub trait DeferredNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> &DeferredNode<Self::Node>;

    fn as_node_mut(&mut self) -> &mut DeferredNode<Self::Node>;

    fn clone_into_node(&self) -> DeferredNode<Self::Node> {
        self.as_node_ref().clone()
    }
}

impl<T> DeferredNodeRef for DeferredNode<T>
where
    T: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static,
{
    type Node = T;

    fn as_node_ref(&self) -> &DeferredNode<Self::Node> {
        self
    }

    fn as_node_mut(&mut self) -> &mut DeferredNode<Self::Node> {
        self
    }
}

impl DeferredNodeRef for DeferredText {
    type Node = web_sys::Text;

    fn as_node_ref(&self) -> &DeferredNode<Self::Node> {
        todo!()
    }

    fn as_node_mut(&mut self) -> &mut DeferredNode<Self::Node> {
        todo!()
    }
}

impl DeferredNodeRef for DeferredElement {
    type Node = web_sys::Element;

    fn as_node_ref(&self) -> &DeferredNode<Self::Node> {
        todo!()
    }

    fn as_node_mut(&mut self) -> &mut DeferredNode<Self::Node> {
        todo!()
    }
}
