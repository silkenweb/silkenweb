use std::{
    fmt::{self, Display},
    mem,
};

use wasm_bindgen::{JsValue, UnwrapThrowExt};

use super::{HydrationNodeData, WetNode};
use crate::{
    attribute::Attribute,
    event::EventCallback,
    global::document,
    render::{after_render, queue_update},
};

pub struct WetElement {
    dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
}

impl WetElement {
    pub fn new(tag: &str) -> Self {
        Self::new_from_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_from_element(document::create_element_ns(namespace, tag))
    }

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

    pub fn store_child(&mut self, mut child: HydrationNodeData) {
        self.event_callbacks
            .append(&mut child.take_wet_event_callbacks());
    }

    pub fn dom_element(&self) -> &web_sys::Element {
        &self.dom_element
    }

    pub fn append_child_now(&mut self, child: &mut impl WetNode) {
        self.dom_element
            .append_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn insert_child_before(&mut self, child: impl WetNode, next_child: Option<impl WetNode>) {
        let parent = self.dom_node();
        let child = child.dom_node();
        let next_child = next_child.map(|c| c.dom_node());

        queue_update(move || {
            parent
                .insert_before(&child, next_child.as_ref())
                .unwrap_throw();
        });
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl WetNode,
        next_child: Option<&mut impl WetNode>,
    ) {
        self.dom_element
            .insert_before(&child.dom_node(), next_child.map(|c| c.dom_node()).as_ref())
            .unwrap_throw();
    }

    pub fn replace_child(&mut self, new_child: &mut impl WetNode, old_child: &mut impl WetNode) {
        let parent = self.dom_element.clone();
        let new_child = new_child.dom_node();
        let old_child = old_child.dom_node();

        queue_update(move || {
            parent.replace_child(&new_child, &old_child).unwrap_throw();
        });
    }

    pub fn remove_child(&mut self, child: &mut impl WetNode) {
        let parent = self.dom_element.clone();
        let child = child.dom_node();
        queue_update(move || {
            parent.remove_child(&child).unwrap_throw();
        });
    }

    pub fn remove_child_now(&mut self, child: &mut impl WetNode) {
        self.dom_element
            .remove_child(&child.dom_node())
            .unwrap_throw();
    }

    pub fn clear_children(&mut self) {
        let parent = self.dom_element.clone();

        queue_update(move || parent.set_inner_html(""));
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        value.set_attribute(name, &self.dom_element);
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let dom_element = self.dom_element.clone();
        after_render(move || f(&dom_element));
    }

    pub(super) fn take_event_callbacks(&mut self) -> Vec<EventCallback> {
        mem::take(&mut self.event_callbacks)
    }
}

impl Display for WetElement {
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

        queue_update(move || {
            parent.set_text_content(Some(&text));
        });
    }
}

impl Display for WetText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(text) = self.0.text_content() {
            f.write_str(&text)?;
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
