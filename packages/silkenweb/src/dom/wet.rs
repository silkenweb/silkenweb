use std::fmt;

use silkenweb_base::document;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{ShadowRootInit, ShadowRootMode};

use super::{
    private::{DomElement, DomText, EventStore, InstantiableDomElement, InstantiableDomNode},
    Wet,
};
use crate::{node::element::Namespace, task::on_animation_frame};

#[derive(Clone)]
pub struct WetElement {
    element: web_sys::Element,
}

impl WetElement {
    pub fn from_element(element: web_sys::Element) -> Self {
        Self { element }
    }

    pub fn create_shadow_root(&self) -> web_sys::ShadowRoot {
        self.element.shadow_root().unwrap_or_else(|| {
            self.element
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap_throw()
        })
    }
}

impl fmt::Display for WetElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.element.outer_html())
    }
}

impl DomElement for WetElement {
    type Node = WetNode;

    fn new(ns: Namespace, tag: &str) -> Self {
        Self {
            element: ns.create_element(tag),
        }
    }

    fn append_child(&mut self, child: &WetNode) {
        self.element.append_child(child.dom_node()).unwrap_throw();
    }

    fn insert_child_before(
        &mut self,
        _index: usize,
        child: &WetNode,
        next_child: Option<&WetNode>,
    ) {
        self.element
            .insert_before(child.dom_node(), next_child.map(|c| c.dom_node()))
            .unwrap_throw();
    }

    fn replace_child(&mut self, _index: usize, new_child: &WetNode, old_child: &WetNode) {
        self.element
            .replace_child(new_child.dom_node(), old_child.dom_node())
            .unwrap_throw();
    }

    fn remove_child(&mut self, _index: usize, child: &WetNode) {
        self.element.remove_child(child.dom_node()).unwrap_throw();
    }

    fn clear_children(&mut self) {
        self.element.set_text_content(Some(""))
    }

    fn add_class(&mut self, name: &str) {
        self.element.class_list().add_1(name).unwrap_throw()
    }

    fn remove_class(&mut self, name: &str) {
        self.element.class_list().remove_1(name).unwrap_throw()
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        if let Some(attr) = value.text() {
            self.element.set_attribute(name, &attr)
        } else {
            self.element.remove_attribute(name)
        }
        .unwrap_throw()
    }

    fn on(
        &mut self,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
        events: &mut EventStore,
    ) {
        events.add_listener(&self.element, name, f);
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        Some(self.element.clone())
    }

    fn style_property(&mut self, name: &str, value: &str) {
        let style_props = if let Some(elem) = self.element.dyn_ref::<web_sys::HtmlElement>() {
            elem.style()
        } else if let Some(elem) = self.element.dyn_ref::<web_sys::SvgElement>() {
            elem.style()
        } else {
            panic!("Unknown element type");
        };

        style_props.set_property(name, value).unwrap_throw();
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let element = self.element.clone();
        on_animation_frame(move || f(&element));
    }
}

impl InstantiableDomElement for WetElement {
    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>) {
        let shadow_root = self.create_shadow_root();

        for child in children {
            shadow_root.append_child(child.dom_node()).unwrap_throw();
        }
    }

    fn clone_node(&self) -> Self {
        Self {
            element: self
                .element
                .clone_node_with_deep(true)
                .unwrap()
                .unchecked_into(),
        }
    }
}

#[derive(Clone)]
pub struct WetText(web_sys::Text);

impl WetText {
    pub fn from_dom(text: web_sys::Text) -> Self {
        Self(text)
    }

    pub fn dom_text(&self) -> &web_sys::Text {
        &self.0
    }

    pub fn text(&self) -> String {
        self.0.text_content().expect("No text content found")
    }

    pub fn clone_node(&self) -> Self {
        Self(self.0.clone_node().unwrap_throw().unchecked_into())
    }
}

impl DomText for WetText {
    fn new(text: &str) -> Self {
        Self(document::create_text_node(text))
    }

    fn set_text(&mut self, text: &str) {
        self.0.set_text_content(Some(text));
    }
}

#[derive(Clone)]
pub struct WetNode(web_sys::Node);

impl WetNode {
    pub fn dom_node(&self) -> &web_sys::Node {
        &self.0
    }

    pub fn clone_node(&self) -> Self {
        Self(self.0.clone_node_with_deep(true).unwrap_throw())
    }
}

impl fmt::Display for WetNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(elem) = self.0.dyn_ref::<web_sys::Element>() {
            f.write_str(&elem.outer_html())
        } else {
            f.write_str(&self.0.text_content().expect("No text content found"))
        }
    }
}

impl InstantiableDomNode for WetNode {
    type DomType = Wet;

    fn into_element(self) -> WetElement {
        WetElement {
            element: self.0.unchecked_into(),
        }
    }

    fn first_child(&self) -> Self {
        Self(self.0.first_child().unwrap_throw())
    }

    fn next_sibling(&self) -> Self {
        Self(self.0.next_sibling().unwrap_throw())
    }
}

impl From<WetElement> for WetNode {
    fn from(element: WetElement) -> Self {
        Self(element.element.into())
    }
}

impl From<WetText> for WetNode {
    fn from(text: WetText) -> Self {
        Self(text.0.into())
    }
}
