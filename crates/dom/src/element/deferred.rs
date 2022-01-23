// TODO: Enable warnings
#![allow(dead_code, unused_variables)]
use std::future::Future;

use wasm_bindgen::JsValue;

use super::strict::StrictNodeBase;
use crate::attribute::Attribute;

pub struct DeferredElement {}

impl DeferredElement {
    pub fn new(tag: &str) -> Self {
        todo!()
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        todo!()
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
        todo!()
    }

    pub fn shrink_to_fit(&mut self) {
        todo!()
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        todo!()
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        todo!()
    }

    pub fn store_child(&mut self, child: Self) {
        todo!()
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        todo!()
    }

    fn dom_element(&self) -> &web_sys::Element {
        todo!()
    }
}

#[derive(Clone)]
pub struct DeferredNode<T>(T);

impl<T: AsRef<web_sys::Node> + Clone + 'static> DeferredNode<T> {
    pub fn append_child_now(&mut self, child: &mut impl DeferredNodeRef) {
        todo!()
    }

    pub fn insert_child_before(
        &mut self,
        child: DeferredNodeBase,
        next_child: Option<DeferredNodeBase>,
    ) {
        todo!()
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl DeferredNodeRef,
        next_child: Option<&mut impl DeferredNodeRef>,
    ) {
        todo!()
    }

    pub fn replace_child(&mut self, new_child: DeferredNodeBase, old_child: DeferredNodeBase) {
        todo!()
    }

    pub fn remove_child_now(&mut self, child: &mut impl DeferredNodeRef) {
        todo!()
    }

    pub fn remove_child(&mut self, child: DeferredNodeBase) {
        todo!()
    }

    pub fn clear_children(&mut self) {
        todo!()
    }

    fn dom_node(&self) -> &web_sys::Node {
        todo!()
    }
}

impl DeferredNode<web_sys::Element> {
    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        todo!()
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        todo!()
    }
}

pub type DeferredNodeBase = DeferredNode<web_sys::Node>;

impl<T: Into<web_sys::Node>> DeferredNode<T> {
    pub fn into_base(self) -> StrictNodeBase {
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
