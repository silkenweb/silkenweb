use std::{
    fmt::{self, Display},
    mem,
};

use html_escape::encode_text_minimal;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::XmlSerializer;

use super::{HydrationNodeData, Namespace, WetNode};
use crate::{
    attribute::Attribute,
    event::EventCallback,
    global::document,
    intern_str,
    render::{after_render, queue_update, RenderUpdate},
};

pub struct WetElement {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
}

impl WetElement {
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        let dom_element = match namespace {
            Namespace::Html => document::create_element(tag),
            Namespace::Other(ns) => document::create_element_ns(ns.map(intern_str), tag),
        };
        Self::new_from_element(dom_element)
    }

    pub fn new_from_element(dom_element: web_sys::Element) -> Self {
        Self {
            dom_element,
            event_callbacks: Vec::new(),
        }
    }

    pub fn fmt(&self, current_ns: Namespace, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dom_elem = self.dom_element();
        let dom_namespace = dom_elem.namespace_uri().unwrap_or_default();

        // TODO: Test this
        if current_ns.as_str() == dom_namespace {
            f.write_str(&self.dom_element.outer_html())
        } else {
            f.write_str(
                &XmlSerializer::new()
                    .unwrap_throw()
                    .serialize_to_string(dom_elem)
                    .unwrap_throw(),
            )
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.event_callbacks.shrink_to_fit();
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        let dom_element = self.dom_element.clone();
        self.event_callbacks
            .push(EventCallback::new(dom_element.into(), name, f));
    }

    pub fn store_child(&mut self, mut child: HydrationNodeData) {
        self.event_callbacks
            .append(&mut child.take_wet_event_callbacks());
    }

    pub fn dom_element(&self) -> &web_sys::Element {
        &self.dom_element
    }

    pub fn append_child_now(&mut self, child: impl WetNode) {
        self.dom_element
            .append_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn append_child(&mut self, child: impl WetNode) {
        queue_update(RenderUpdate::AppendChild {
            parent: self.dom_element.clone(),
            child: child.dom_node(),
        });
    }

    pub fn insert_child_before(&mut self, child: impl WetNode, next_child: Option<impl WetNode>) {
        queue_update(RenderUpdate::InsertBefore {
            parent: self.dom_element.clone(),
            child: child.dom_node(),
            next_child: next_child.map(|c| c.dom_node()),
        });
    }

    pub fn replace_child(&mut self, new_child: impl WetNode, old_child: impl WetNode) {
        queue_update(RenderUpdate::ReplaceChild {
            parent: self.dom_element.clone(),
            new_child: new_child.dom_node(),
            old_child: old_child.dom_node(),
        });
    }

    pub fn remove_child(&mut self, child: impl WetNode) {
        queue_update(RenderUpdate::RemoveChild {
            parent: self.dom_element.clone(),
            child: child.dom_node(),
        });
    }

    pub fn clear_children(&mut self) {
        queue_update(RenderUpdate::ClearChildren {
            parent: self.dom_element.clone(),
        });
    }

    pub fn attribute_now<A: Attribute>(&mut self, name: &str, value: A) {
        value.set_attribute(name, &self.dom_element);
    }

    pub fn attribute<A: Attribute + 'static>(&mut self, name: &str, value: A) {
        let name = name.to_owned();
        let dom_element = self.dom_element.clone();
        queue_update(RenderUpdate::Function(Box::new(move || {
            value.set_attribute(&name, &dom_element)
        })));
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.dom_element.clone();
        after_render(move || f(&dom_element));
    }

    pub(super) fn take_event_callbacks(&mut self) -> Vec<EventCallback> {
        mem::take(&mut self.event_callbacks)
    }
}

#[derive(Clone)]
pub struct WetText(web_sys::Text);

impl WetText {
    pub fn new(text: &str) -> Self {
        Self(document::create_text_node(text))
    }

    pub fn new_from_text(node: web_sys::Text) -> Self {
        Self(node)
    }

    pub fn dom_text(&self) -> &web_sys::Text {
        &self.0
    }

    pub fn set_text(&mut self, text: String) {
        queue_update(RenderUpdate::SetTextContent{parent: self.0.clone(), text});
    }
}

impl Display for WetText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(text) = self.0.text_content() {
            f.write_str(&encode_text_minimal(&text))?;
        }

        Ok(())
    }
}

impl WetNode for WetElement {
    fn dom_node(&self) -> web_sys::Node {
        self.dom_element.clone().into()
    }
}

impl WetNode for WetText {
    fn dom_node(&self) -> web_sys::Node {
        self.0.clone().into()
    }
}

impl<'a, T: WetNode> WetNode for &'a T {
    fn dom_node(&self) -> web_sys::Node {
        WetNode::dom_node(*self)
    }
}

impl<'a, T: WetNode> WetNode for &'a mut T {
    fn dom_node(&self) -> web_sys::Node {
        WetNode::dom_node(*self)
    }
}
