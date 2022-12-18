use std::fmt;

use silkenweb_base::{document, intern_str};
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
    events: EventStore,
}

impl WetElement {
    pub fn from_element(element: web_sys::Element) -> Self {
        Self {
            element,
            events: EventStore::default(),
        }
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
        let element = match ns {
            Namespace::Html => document::create_element(tag),
            _ => document::create_element_ns(intern_str(ns.as_str()), tag),
        };

        Self {
            element,
            events: EventStore::default(),
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

    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>) {
        let elem = &self.element;
        let shadow_root = elem.shadow_root().unwrap_or_else(|| {
            elem.attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .unwrap_throw()
        });

        for child in children {
            shadow_root.append_child(child.dom_node()).unwrap_throw();
        }
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

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.events.add_listener(&self.element, name, f)
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        Some(self.element.clone())
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let element = self.element.clone();
        on_animation_frame(move || f(&element));
    }

    fn events(&mut self) -> EventStore {
        self.events.clone()
    }
}

impl InstantiableDomElement for WetElement {
    fn clone_node(&self) -> Self {
        Self {
            element: self
                .element
                .clone_node_with_deep(true)
                .unwrap()
                .unchecked_into(),
            events: EventStore::default(),
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
pub struct WetNode {
    node: web_sys::Node,
    events: EventStore,
}

impl WetNode {
    pub fn dom_node(&self) -> &web_sys::Node {
        &self.node
    }
}

impl fmt::Display for WetNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(elem) = self.node.dyn_ref::<web_sys::Element>() {
            f.write_str(&elem.outer_html())
        } else {
            f.write_str(&self.node.text_content().expect("No text content found"))
        }
    }
}

impl InstantiableDomNode for WetNode {
    type DomType = Wet;

    fn into_element(self) -> WetElement {
        WetElement {
            element: self.node.unchecked_into(),
            events: self.events,
        }
    }

    fn first_child(&self) -> Self {
        Self {
            node: self.node.first_child().unwrap_throw(),
            events: EventStore::default(),
        }
    }

    fn next_sibling(&self) -> Self {
        Self {
            node: self.node.next_sibling().unwrap_throw(),
            events: EventStore::default(),
        }
    }
}

impl From<WetElement> for WetNode {
    fn from(element: WetElement) -> Self {
        Self {
            node: element.element.into(),
            events: element.events,
        }
    }
}

impl From<WetText> for WetNode {
    fn from(text: WetText) -> Self {
        Self {
            node: text.0.into(),
            events: EventStore::default(),
        }
    }
}
