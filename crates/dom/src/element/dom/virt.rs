use std::{collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::JsValue;

use super::{
    real::{RealElement, RealText},
    DomElement, DomNodeData,
};
use crate::{attribute::Attribute, spawn_cancelable_future};

pub struct VElement {
    namespace: Option<String>,
    tag: String,
    attributes: HashMap<String, Option<String>>,
    children: Vec<DomNodeData>,
    stored_children: Vec<DomElement>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    hydrate_actions: Vec<Box<dyn FnOnce(&mut RealElement)>>,
}

impl VElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(None, tag)
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(Some(namespace), tag)
    }

    pub fn hydrate(&self, _node: &web_sys::Node) -> web_sys::Node {
        todo!()
    }

    fn new_element(namespace: Option<&str>, tag: &str) -> Self {
        Self {
            namespace: namespace.map(str::to_owned),
            tag: tag.to_owned(),
            attributes: HashMap::new(),
            children: Vec::new(),
            stored_children: Vec::new(),
            futures: Vec::new(),
            hydrate_actions: Vec::new(),
        }
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.futures.push(spawn_cancelable_future(future));
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    pub fn store_child(&mut self, child: DomElement) {
        self.stored_children.push(child);
    }

    pub fn append_child(&mut self, child: &mut impl VNode) {
        self.children.push(child.node())
    }

    pub fn insert_child_before(
        &mut self,
        child: &mut impl VNode,
        next_child: Option<&mut impl VNode>,
    ) {
        if let Some(next_child) = next_child {
            let next_child = next_child.node();
            let index = self
                .children
                .iter()
                .position(|existing| existing.is_same(&next_child))
                .expect("Child not found");
            self.children.insert(index, child.node());
        } else {
            self.append_child(child);
        }
    }

    pub fn replace_child(&mut self, new_child: &mut impl VNode, old_child: &mut impl VNode) {
        for child in &mut self.children {
            if child.node().is_same(&old_child.node()) {
                *child = new_child.node();
            }
        }
    }

    pub fn remove_child(&mut self, child: &mut impl VNode) {
        let child = child.node();

        self.children.retain(|existing| !existing.is_same(&child));
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        self.attributes.insert(name.to_owned(), value.text());
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }
}

impl From<VElement> for RealElement {
    fn from(element: VElement) -> Self {
        let mut elem = if let Some(namespace) = element.namespace {
            RealElement::new_in_namespace(&namespace, &element.tag)
        } else {
            RealElement::new(&element.tag)
        };

        for (name, value) in element.attributes {
            elem.attribute(&name, value);
        }

        for mut child in element.children {
            elem.append_child(&mut child);
        }

        for future in element.futures {
            elem.store_future(future);
        }

        for child in element.stored_children {
            elem.store_child(child);
        }

        for event in element.hydrate_actions {
            event(&mut elem);
        }

        elem
    }
}

#[derive(Clone)]
pub struct VText(String);

impl VText {
    pub fn new(text: &str) -> Self {
        Self(text.to_owned())
    }

    pub fn set_text(&mut self, text: String) {
        self.0 = text;
    }
}

impl From<VText> for RealText {
    fn from(text: VText) -> Self {
        RealText::new(&text.0)
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait VNode {
    fn node(&self) -> DomNodeData;
}
