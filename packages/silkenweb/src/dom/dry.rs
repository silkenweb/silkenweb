use std::{cell::RefCell, fmt, rc::Rc};

use html_escape::encode_text_minimal;
use wasm_bindgen::JsValue;

use super::{
    private::{self, EventStore, InstantiableDomElement},
    wet::WetElement,
    Dry,
};
use crate::node::element::Namespace;

mod shared_element;

pub use shared_element::SharedDryElement;

#[derive(Clone)]

pub struct DryElement(Rc<RefCell<SharedDryElement<DryNode>>>);

impl DryElement {
    fn from_shared(shared: SharedDryElement<DryNode>) -> Self {
        Self(Rc::new(RefCell::new(shared)))
    }
}

impl private::DomElement for DryElement {
    type Node = DryNode;

    fn new(ns: &Namespace, tag: &str) -> Self {
        Self::from_shared(SharedDryElement::new(ns.clone(), tag))
    }

    fn append_child(&mut self, child: &Self::Node) {
        self.0.borrow_mut().append_child(child)
    }

    fn insert_child_before(
        &mut self,
        index: usize,
        child: &Self::Node,
        next_child: Option<&Self::Node>,
    ) {
        self.0
            .borrow_mut()
            .insert_child_before(index, child, next_child)
    }

    fn replace_child(&mut self, index: usize, new_child: &Self::Node, old_child: &Self::Node) {
        self.0
            .borrow_mut()
            .replace_child(index, new_child, old_child)
    }

    fn remove_child(&mut self, index: usize, child: &Self::Node) {
        self.0.borrow_mut().remove_child(index, child)
    }

    fn clear_children(&mut self) {
        self.0.borrow_mut().clear_children()
    }

    fn add_class(&mut self, name: &str) {
        self.0.borrow_mut().add_class(name)
    }

    fn remove_class(&mut self, name: &str) {
        self.0.borrow_mut().remove_class(name)
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        self.0.borrow_mut().attribute(name, value)
    }

    fn on(
        &mut self,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
        events: &mut EventStore,
    ) {
        self.0.borrow_mut().on(name, f, events)
    }

    fn dom_element(&self) -> web_sys::Element {
        self.try_dom_element()
            .expect("Can't get raw dom element from `Dry` dom element")
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        self.0.borrow().try_dom_element()
    }

    fn style_property(&mut self, name: &str, value: &str) {
        self.0.borrow_mut().style_property(name, value)
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.0.borrow_mut().effect(f)
    }

    fn observe_attributes(
        &mut self,
        f: impl FnMut(js_sys::Array, web_sys::MutationObserver) + 'static,
        events: &mut EventStore,
    ) {
        self.0.borrow_mut().observe_attributes(f, events)
    }
}

impl private::InstantiableDomElement for DryElement {
    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>) {
        self.0.borrow_mut().attach_shadow_children(children)
    }

    fn clone_node(&self) -> Self {
        Self::from_shared(self.0.borrow().clone_node())
    }
}

impl fmt::Display for DryElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

#[derive(Clone)]
pub struct DryText(Rc<RefCell<SharedDryText<DryNode>>>);

impl DryText {
    pub fn clone_node(&self) -> Self {
        Self(Rc::new(RefCell::new(self.0.borrow().clone_node())))
    }
}

impl fmt::Display for DryText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl private::DomText for DryText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedDryText::new(text.to_string()))))
    }

    fn set_text(&mut self, text: &str) {
        self.0.borrow_mut().set_text(text.to_string())
    }
}

#[derive(Clone)]
pub enum DryNode {
    Element(DryElement),
    Text(DryText),
}

impl private::InstantiableDomNode for DryNode {
    type DomType = Dry;

    fn into_element(self) -> <Self::DomType as private::Dom>::Element {
        match self {
            DryNode::Element(element) => element,
            DryNode::Text(_) => panic!("Text node when expecting element"),
        }
    }

    fn first_child(&self) -> Self {
        match self {
            DryNode::Element(element) => element
                .0
                .borrow()
                .first_child()
                .expect("No children")
                .clone(),
            DryNode::Text(_) => panic!("Text nodes can't have children"),
        }
    }

    fn next_sibling(&self) -> Self {
        match self {
            DryNode::Element(element) => element
                .0
                .borrow()
                .next_sibling()
                .expect("This is the last child")
                .clone(),
            DryNode::Text(text) => text
                .0
                .borrow()
                .next_sibling()
                .expect("This is the last child")
                .clone(),
        }
    }
}

impl From<DryElement> for DryNode {
    fn from(value: DryElement) -> Self {
        Self::Element(value)
    }
}

impl From<DryText> for DryNode {
    fn from(value: DryText) -> Self {
        Self::Text(value)
    }
}

impl fmt::Display for DryNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DryNode::Element(element) => element.fmt(f),
            DryNode::Text(text) => text.fmt(f),
        }
    }
}

impl DryChild for DryNode {
    fn clone_node(&self) -> Self {
        match self {
            DryNode::Element(element) => DryNode::Element(element.clone_node()),
            DryNode::Text(text) => DryNode::Text(text.clone_node()),
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<&Self>) {
        let next_sibling = next_sibling.cloned();

        match self {
            DryNode::Element(element) => element.0.borrow_mut().set_next_sibling(next_sibling),
            DryNode::Text(text) => text.0.borrow_mut().set_next_sibling(next_sibling),
        }
    }
}

pub trait DryChild: Clone {
    fn clone_node(&self) -> Self;

    fn set_next_sibling(&self, next_sibling: Option<&Self>);
}

pub struct SharedDryText<Node> {
    text: String,
    next_sibling: Option<Node>,
}

impl<Node> SharedDryText<Node> {
    pub fn new(text: String) -> Self {
        Self {
            text,
            next_sibling: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn next_sibling(&self) -> Option<&Node> {
        self.next_sibling.as_ref()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Node>) {
        self.next_sibling = next_sibling;
    }

    pub fn clone_node(&self) -> Self {
        Self {
            text: self.text.clone(),
            next_sibling: None,
        }
    }
}

impl<Node> fmt::Display for SharedDryText<Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        encode_text_minimal(&self.text).fmt(f)
    }
}

impl<Node> From<SharedDryText<Node>> for String {
    fn from(value: SharedDryText<Node>) -> Self {
        value.text
    }
}

type LazyElementAction = Box<dyn FnOnce(&mut WetElement)>;

#[cfg(test)]
mod tests {
    use silkenweb_macros::cfg_browser;

    #[cfg_browser(false)]
    use crate::task::{render_now, scope};
    use crate::{dom::Dry, elements::html::*, node::element::TextParentElement, prelude::*};

    #[cfg(feature = "declarative-shadow-dom")]
    #[test]
    fn declarative_shadow_dom() {
        assert_eq!(
            shadow_host().freeze().to_string(),
            r#"<div><template shadowroot="open"><slot></slot></template><h2>Light content</h2></div>"#
        );
    }

    #[cfg(not(feature = "declarative-shadow-dom"))]
    #[test]
    fn declarative_shadow_dom() {
        assert_eq!(
            shadow_host().freeze().to_string(),
            r#"<div><h2>Light content</h2></div>"#
        );
    }

    fn shadow_host() -> Div<Dry> {
        div()
            .attach_shadow_children([slot()])
            .child(h2().text("Light content"))
    }

    #[cfg_browser(false)]
    #[tokio::test]
    async fn style_property() {
        scope(async {
            let app: Div<Dry> = div()
                .style_property("--test0", "value0")
                .style_property("--test1", "value1");

            render_now().await;

            assert_eq!(
                app.freeze().to_string(),
                r#"<div style="--test0: value0; --test1: value1;"></div>"#
            );
        })
        .await
    }
}
