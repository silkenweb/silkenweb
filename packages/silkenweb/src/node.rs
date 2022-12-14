//! Generic DOM types.

use std::{
    fmt::{self, Display},
};

use silkenweb_signals_ext::value::Value;

use self::element::Resource;
use crate::dom::{wet::Wet, DefaultDom, Dom, DomText};

pub mod element;

/// A DOM Node
pub struct Node<D: Dom = DefaultDom> {
    node: D::Node,
    resources: Vec<Resource<D>>,
}

impl<D: Dom> Node<D> {
    fn as_node(&self) -> &D::Node {
        &self.node
    }

    fn into_node(self) -> D::Node {
        self.node
    }
}

impl Node<Wet> {
    pub fn dom_node(&self) -> &web_sys::Node {
        self.node.dom_node()
    }
}

impl<D: Dom> Value for Node<D> {}

impl<D: Dom> From<Text<D>> for Node<D> {
    fn from(text: Text<D>) -> Self {
        Self {
            node: text.0.into(),
            resources: Vec::new(),
        }
    }
}

impl<D: Dom> Display for Node<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node.fmt(f)
    }
}

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
