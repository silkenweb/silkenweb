use crate::hydration::node::Namespace;

pub mod wet;

pub trait Dom {
    type Element: DomElement;
    type Text;
    type Node;
}

// TODO: Is `Clone` required?
pub trait DomElement: Clone {
    fn new(ns: Namespace, tag: &str) -> Self;
}

pub type DefaultDom = wet::Wet;
