use silkenweb_base::{document, intern_str};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

use super::{
    Dom, DomElement, DomText, InstantiableDom, InstantiableDomElement, InstantiableDomNode,
};
use crate::{hydration::node::Namespace, task::on_animation_frame};

pub struct Wet;

impl Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

impl InstantiableDom for Wet {
    type InstantiableElement = WetElement;
    type InstantiableNode = WetNode;
}

#[derive(Clone)]
pub struct WetElement {
    element: web_sys::Element,
    // TODO: Store event callbacks, unless wasm-bindgen weak-refs is enabled.
}

impl DomElement for WetElement {
    type Node = WetNode;

    fn new(ns: Namespace, tag: &str) -> Self {
        let element = match ns {
            Namespace::Html => document::create_element(tag),
            Namespace::Other(ns) => document::create_element_ns(ns.map(intern_str), tag),
        };

        Self { element }
    }

    fn append_child(&mut self, child: &WetNode) {
        self.element.append_child(child.dom_node()).unwrap_throw();
    }

    fn insert_child_before(&mut self, _index: usize, child: &WetNode, next_child: Option<&WetNode>) {
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

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.element
            .add_event_listener_with_callback(name, Closure::new(f).into_js_value().unchecked_ref())
            .unwrap_throw();
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let element = self.element.clone();
        on_animation_frame(move || f(&element));
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
        }
    }
}

#[derive(Clone)]
pub struct WetText(web_sys::Text);

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
    pub(crate) fn dom_node(&self) -> &web_sys::Node {
        &self.0
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
        Self(self.0.first_child().unwrap())
    }

    fn next_sibling(&self) -> Self {
        Self(self.0.next_sibling().unwrap())
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
