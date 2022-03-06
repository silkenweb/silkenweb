use std::{
    fmt::{self, Display},
    mem,
};

use html_escape::encode_text_minimal;
use silkenweb_base::{document, intern_str};
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use super::{event::EventCallback, Namespace};
use crate::{
    attribute::Attribute,
    hydration::Wet,
    node::{private::{ElementImpl, TextImpl}, Node},
    task::{after_animation_frame, queue_update},
};

pub struct WetElement {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
}

impl WetElement {
    pub fn new_from_element(dom_element: web_sys::Element) -> Self {
        Self {
            dom_element,
            event_callbacks: Vec::new(),
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

    pub fn dom_element(&self) -> &web_sys::Element {
        &self.dom_element
    }

    pub fn clear_children(&mut self) {
        let parent = self.dom_element.clone();

        queue_update(move || parent.set_inner_html(""));
    }

    pub fn attribute_now<A: Attribute>(&mut self, name: &str, value: A) {
        Self::set_attribute(&self.dom_element, name, value);
    }

    pub fn attribute<A: Attribute + 'static>(&mut self, name: &str, value: A) {
        let name = name.to_owned();
        let dom_element = self.dom_element.clone();
        queue_update(move || Self::set_attribute(&dom_element, &name, value));
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.dom_element.clone();
        after_animation_frame(move || f(&dom_element));
    }

    pub(super) fn take_event_callbacks(&mut self) -> Vec<EventCallback> {
        mem::take(&mut self.event_callbacks)
    }

    fn set_attribute<A: Attribute>(dom_element: &web_sys::Element, name: &str, value: A) {
        if let Some(attr) = value.text() {
            dom_element.set_attribute(name, &attr)
        } else {
            dom_element.remove_attribute(name)
        }
        .unwrap_throw()
    }
}

impl ElementImpl<Wet> for WetElement {
    fn new(namespace: Namespace, tag: &str) -> Self {
        let dom_element = match namespace {
            Namespace::Html => document::create_element(tag),
            Namespace::Other(ns) => document::create_element_ns(ns.map(intern_str), tag),
        };
        Self::new_from_element(dom_element)
    }

    fn shrink_to_fit(&mut self) {
        todo!()
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        todo!()
    }

    fn store_child(&mut self, child: Node<Wet>) {
        self.event_callbacks
        .append(&mut child.take_wet_event_callbacks());
}

    fn dom_element(&self) -> web_sys::Element {
        todo!()
    }

    fn append_child_now(&mut self, child: &Node<Wet>) {
        self.dom_element
            .append_child(&child.dom_node())
            .unwrap_throw();
    }

    fn append_child(&mut self, child: &Node<Wet>) {
        let parent = self.dom_element().clone();
        let child = child.dom_node();
        queue_update(move || {
            parent.append_child(&child).unwrap_throw();
        });
    }

    fn insert_child_before(&mut self, child: &Node<Wet>, next_child: Option<&Node<Wet>>) {
        let parent = self.dom_element().clone();
        let child = child.dom_node();
        let next_child = next_child.map(|c| c.dom_node());

        queue_update(move || {
            parent
                .insert_before(&child, next_child.as_ref())
                .unwrap_throw();
        });
    }

    fn replace_child(&mut self, new_child: &Node<Wet>, old_child: &Node<Wet>) {
        let parent = self.dom_element.clone();
        let new_child = new_child.dom_node();
        let old_child = old_child.dom_node();

        queue_update(move || {
            parent.replace_child(&new_child, &old_child).unwrap_throw();
        });
    }

    fn remove_child(&mut self, child: &Node<Wet>) {
        let parent = self.dom_element.clone();
        let child = child.dom_node();
        queue_update(move || {
            parent.remove_child(&child).unwrap_throw();
        });
    }

    fn clear_children(&mut self) {
        todo!()
    }

    fn attribute_now<A: Attribute>(&mut self, name: &str, value: A) {
        todo!()
    }

    fn attribute<A: Attribute + 'static>(&mut self, name: &str, value: A) {
        todo!()
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        todo!()
    }
}

impl fmt::Display for WetElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.dom_element.outer_html())
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
        let parent = self.0.clone();

        queue_update(move || parent.set_text_content(Some(&text)));
    }
}

impl TextImpl for WetText {
    fn new(text: &str) -> Self {
        todo!()
    }

    fn set_text(&mut self, text: String) {
        todo!()
    }
}

impl From<WetText> for web_sys::Text {
    fn from(text: WetText) -> Self {
        text.0
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
