//! Generic DOM types.

use std::fmt;

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;

use self::element::Element;
use crate::hydration::{
    Dry, HydrationStats, Wet,
};

pub mod element;

/// The implmenetation type of Node.
///
/// For example, wet or dry, depending on hydration status.
pub trait NodeImpl: private::NodeImpl {}

impl<T: private::NodeImpl> NodeImpl for T {}

pub(super) mod private {
    use std::fmt::Display;

    use wasm_bindgen::JsValue;

    use super::Node;
    use crate::{hydration::node::Namespace, macros::Attribute};

    pub trait NodeImpl: 'static + Sized {
        type Element: 'static + ElementImpl<Self> + Display;
        type Text: 'static + TextImpl + Display;
    }

    pub trait ElementImpl<Impl: NodeImpl> {
        fn new(namespace: Namespace, name: &str) -> Self;

        fn shrink_to_fit(&mut self);

        fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static);

        fn store_child(&mut self, child: Node<Impl>);

        fn dom_element(&self) -> web_sys::Element;

        fn append_child_now(&mut self, child: &Node<Impl>);

        fn append_child(&mut self, child: &Node<Impl>);

        fn insert_child_before(&mut self, child: &Node<Impl>, next_child: Option<&Node<Impl>>);

        fn replace_child(&mut self, new_child: &Node<Impl>, old_child: &Node<Impl>);

        fn remove_child(&mut self, child: &Node<Impl>);

        fn clear_children(&mut self);

        fn attribute_now<A: Attribute>(&mut self, name: &str, value: A);

        fn attribute<A: Attribute + 'static>(&mut self, name: &str, value: A);

        fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static);
    }

    pub trait TextImpl {
        fn new(text: &str) -> Self;

        fn set_text(&mut self, text: String);
    }
}

use private::TextImpl;

// A DOM Node
pub struct Node<Impl: NodeImpl = Wet>(NodeEnum<Impl>);

impl Node<Wet> {
    pub(super) fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.dom_element().into(),
            NodeEnum::Text(text) => text.dom_text().clone().into(),
        }
    }
}

impl Node<Dry> {
    pub(super) fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.hydrate_child(parent, child, tracker).into(),
            NodeEnum::Text(text) => text.hydrate_child(parent, child, tracker).into(),
        }
    }

    pub(super) fn is_same(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<Impl: NodeImpl> Node<Impl> {
    fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        match &mut self.0 {
            NodeEnum::Element(elem) => elem.take_futures(),
            NodeEnum::Text(text) => text.take_futures(),
        }
    }
}

impl<Impl: NodeImpl> fmt::Display for Node<Impl> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &mut self.0 {
            NodeEnum::Element(elem) => elem.fmt(f),
            NodeEnum::Text(text) => text.fmt(f),
        }
    }
}

impl Clone for Node<Dry> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<Impl: NodeImpl> From<Element<Impl>> for Node<Impl> {
    fn from(elem: Element<Impl>) -> Self {
        Self(NodeEnum::Element(elem))
    }
}

impl<Impl: NodeImpl> From<Text<Impl>> for Node<Impl> {
    fn from(text: Text<Impl>) -> Self {
        Self(NodeEnum::Text(text))
    }
}

enum NodeEnum<Impl: NodeImpl> {
    Element(Element<Impl>),
    Text(Text<Impl>),
}

/// A text DOM node
pub struct Text<Impl: NodeImpl>(Impl::Text);

impl<Impl: NodeImpl> Text<Impl> {
    fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        Vec::new()
    }
}

impl Text<Dry> {
    pub(crate) fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> web_sys::Text {
        self.0.hydrate_child(parent, child, tracker).into()
    }
}

impl Text<Wet> {
    fn dom_text(&self) -> &web_sys::Text {
        self.0.dom_text()
    }
}

impl<Impl: NodeImpl> fmt::Display for Text<Impl> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Construct a text node
pub fn text<Impl: NodeImpl>(text: &str) -> Text<Impl> {
    Text(Impl::Text::new(text))
}
