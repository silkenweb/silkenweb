use std::fmt::Display;

use wasm_bindgen::JsValue;

use crate::{attribute::Attribute, node::element::Namespace};

pub trait Dom: 'static {
    type Element: DomElement<Node = Self::Node>;
    type Text: DomText + Into<Self::Node>;
    type Node: Clone + Display + 'static;
}

pub trait InstantiableDom:
    Dom<Element = Self::InstantiableElement, Node = Self::InstantiableNode>
{
    type InstantiableElement: InstantiableDomElement<Node = Self::InstantiableNode>;
    type InstantiableNode: InstantiableDomNode<DomType = Self>;
}

pub trait DomElement: Into<Self::Node> + Clone + 'static {
    type Node;

    fn new(ns: Namespace, tag: &str) -> Self;

    fn append_child(&mut self, child: &Self::Node);

    fn insert_child_before(
        &mut self,
        index: usize,
        child: &Self::Node,
        next_child: Option<&Self::Node>,
    );

    fn replace_child(&mut self, index: usize, new_child: &Self::Node, old_child: &Self::Node);

    fn remove_child(&mut self, index: usize, child: &Self::Node);

    fn clear_children(&mut self);

    fn attach_shadow_children(&self, children: impl IntoIterator<Item = Self::Node>);

    fn add_class(&mut self, name: &str);

    fn remove_class(&mut self, name: &str);

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: Attribute;

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static);

    fn dom_element(&self) -> web_sys::Element {
        self.try_dom_element().unwrap()
    }

    fn try_dom_element(&self) -> Option<web_sys::Element>;

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static);
}

pub trait DomText: Clone + 'static {
    fn new(text: &str) -> Self;

    fn set_text(&mut self, text: &str);
}

pub trait InstantiableDomElement: DomElement {
    fn clone_node(&self) -> Self;
}

pub trait InstantiableDomNode: Display + Clone {
    type DomType: Dom;

    fn into_element(self) -> <Self::DomType as Dom>::Element;

    fn first_child(&self) -> Self;

    fn next_sibling(&self) -> Self;
}
