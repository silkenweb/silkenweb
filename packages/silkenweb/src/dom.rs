use wasm_bindgen::JsValue;

use crate::{attribute::Attribute, hydration::node::Namespace};

pub mod wet;

pub trait Dom {
    type Element: DomElement;
    type Text;
    type Node;
}

pub trait DomElement: Clone {
    fn new(ns: Namespace, tag: &str) -> Self;

    fn append_child(&mut self, child: Self);

    fn insert_child_before(&mut self, child: Self, next_child: Option<Self>);

    fn replace_child(&mut self, new_child: Self, old_child: Self);

    fn remove_child(&mut self, child: Self);

    fn clear_children(&mut self);

    fn add_class(&mut self, name: &str);

    fn remove_class(&mut self, name: &str);

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: Attribute;

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static);
}

pub type DefaultDom = wet::Wet;
