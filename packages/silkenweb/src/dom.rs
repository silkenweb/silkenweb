use wasm_bindgen::JsValue;

use crate::{attribute::Attribute, hydration::node::Namespace};

pub mod dry;
pub mod wet;

pub trait Dom: 'static {
    type Element: DomElement<Self::Node>;
    type Text: DomText<Self::Node>;
    type Node;
}

pub trait DomElement<Node>: Into<Node> + Clone + 'static {
    fn new(ns: Namespace, tag: &str) -> Self;

    fn append_child(&mut self, child: &Node);

    fn insert_child_before(&mut self, child: &Node, next_child: Option<&Node>);

    fn replace_child(&mut self, new_child: &Node, old_child: &Node);

    fn remove_child(&mut self, child: &Node);

    fn clear_children(&mut self);

    fn add_class(&mut self, name: &str);

    fn remove_class(&mut self, name: &str);

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: Attribute;

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static);

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static);

    fn store_child(&mut self, child: Node);
}

pub trait DomText<Node>: Into<Node> + Clone + 'static {
    fn new(text: &str) -> Self;
}

pub type DefaultDom = wet::Wet;
