//! Generic DOM types.

use std::fmt;

use silkenweb_signals_ext::value::Value;

use crate::dom::{
    private::{DomText, EventStore},
    DefaultDom, Dom, InDom,
};

mod component;

pub mod element;

pub use component::Component;

/// A DOM Node
pub struct Node<D: Dom = DefaultDom> {
    node: D::Node,
    resources: ResourceVec,
    events: EventStore,
}

impl<D: Dom> Value for Node<D> {}

impl<D: Dom> InDom for Node<D> {
    type Dom = D;
}

impl<D: Dom> From<Text<D>> for Node<D> {
    fn from(text: Text<D>) -> Self {
        Self {
            node: text.0.into(),
            resources: Vec::new(),
            events: EventStore::default(),
        }
    }
}

impl<D: Dom> fmt::Display for Node<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node.fmt(f)
    }
}

/// Trait alias for nodes that can be used as a child
pub trait ChildNode<D: Dom = DefaultDom>: Into<Node<D>> + Value + 'static {}

impl<D: Dom, T: Into<Node<D>> + Value + 'static> ChildNode<D> for T {}

/// A text DOM node
pub struct Text<D: Dom>(D::Text);

impl<D: Dom> Text<D> {
    pub fn new(text: &str) -> Self {
        Self(D::Text::new(text))
    }
}

impl<D: Dom> Value for Text<D> {}

/// Construct a text node
pub fn text<D: Dom>(text: &str) -> Text<D> {
    Text(D::Text::new(text))
}

trait Resource {}

impl<T> Resource for T {}

type ResourceVec = Vec<Box<dyn Resource>>;
