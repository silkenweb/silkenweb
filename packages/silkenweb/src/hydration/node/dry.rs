use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use caseless::default_caseless_match_str;
use html_escape::{encode_double_quoted_attribute, encode_text_minimal};
use indexmap::IndexMap;
use silkenweb_base::clone;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{
    wet::{WetElement, WetText},
    Namespace,
};
use crate::{
    attribute::Attribute,
    hydration::{remove_following_siblings, Dry, HydrationStats},
    node::{
        private::{ElementImpl, TextImpl},
        Node,
    },
};

pub struct DryElement {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<Node<Dry>>,
    stored_children: Vec<Node<Dry>>,
    hydrate_actions: Vec<Box<dyn FnOnce(&mut WetElement)>>,
}

impl DryElement {
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        Self {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            stored_children: Vec::new(),
            hydrate_actions: Vec::new(),
        }
    }

    pub fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        clone!(mut child);

        loop {
            if let Some(elem_child) = child.dyn_ref::<web_sys::Element>() {
                let dom_namespace = elem_child.namespace_uri().unwrap_or_default();
                let dry_namespace = self.namespace.as_str();

                if dry_namespace == dom_namespace
                    && default_caseless_match_str(&elem_child.tag_name(), &self.tag)
                {
                    return self.hydrate_element(elem_child, tracker);
                }
            }

            let next = child.next_sibling();
            tracker.node_removed(&child);
            parent.remove_child(&child).unwrap_throw();

            if let Some(next_child) = next {
                child = next_child;
            } else {
                break;
            }
        }

        let mut wet_child = WetElement::new(self.namespace, &self.tag);

        for (name, value) in self.attributes {
            wet_child.attribute(&name, value);
        }

        for child in &self.children {
            wet_child.append_child_now(child);
        }

        for child in self.stored_children {
            wet_child.store_child(child);
        }

        for event in self.hydrate_actions {
            event(&mut wet_child);
        }

        wet_child.shrink_to_fit();

        let new_element = wet_child.dom_element();
        parent.append_child(new_element).unwrap_throw();
        tracker.node_added(new_element);

        wet_child
    }

    fn hydrate_element(
        self,
        dom_elem: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        self.reconcile_attributes(dom_elem, tracker);
        let mut elem = WetElement::new_from_element(dom_elem.clone());
        let mut current_child = dom_elem.first_child();

        let mut children = self.children.into_iter();

        for mut child in children.by_ref() {
            if let Some(node) = &current_child {
                let hydrated_elem = child.hydrate_child(dom_elem, node, tracker);
                current_child = hydrated_elem.next_sibling();
            } else {
                hydrate_with_new(dom_elem, child, tracker);
                break;
            }
        }

        for child in children {
            hydrate_with_new(dom_elem, child, tracker);
        }

        remove_following_siblings(dom_elem, current_child);

        for event in self.hydrate_actions {
            event(&mut elem);
        }

        for child in self.stored_children {
            elem.store_child(child);
        }

        elem.shrink_to_fit();

        elem
    }

    fn reconcile_attributes(&self, dom_elem: &web_sys::Element, tracker: &mut HydrationStats) {
        let dom_attributes = dom_elem.attributes();
        let mut dom_attr_map = HashMap::new();

        for item_index in 0.. {
            if let Some(attr) = dom_attributes.item(item_index) {
                dom_attr_map.insert(attr.name(), attr.value());
            } else {
                break;
            }
        }

        for (name, value) in &self.attributes {
            let value = value.as_ref();

            let set_attr = if let Some(existing_value) = dom_attr_map.remove(name) {
                value != existing_value
            } else {
                true
            };

            if set_attr {
                dom_elem.set_attribute(name, value).unwrap_throw();
                tracker.attribute_set(dom_elem, name, value);
            }
        }

        for name in dom_attr_map.into_keys() {
            if !name.starts_with("data-silkenweb") {
                tracker.attribute_removed(dom_elem, &name);
                dom_elem.remove_attribute(&name).unwrap_throw();
            }
        }
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        assert_ne!(
            name, "xmlns",
            "\"xmlns\" must be set via a namespace at tag creation time"
        );

        if let Some(value) = value.text() {
            self.attributes.insert(name.to_owned(), value.into_owned());
        } else {
            self.attributes.remove(name);
        }
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }

    fn requires_closing_tag(&self) -> bool {
        ![
            "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta",
            "param", "source", "track", "wbr",
        ]
        .contains(&self.tag.as_str())
    }
}

impl ElementImpl<Dry> for DryElement {
    fn new(namespace: Namespace, name: &str) -> Self {
        todo!()
    }

    fn shrink_to_fit(&mut self) {
        todo!()
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        todo!()
    }

    fn store_child(&mut self, child: crate::macros::Node<Dry>) {
        self.stored_children.push(child);
    }

    fn dom_element(&self) -> web_sys::Element {
        todo!()
    }

    fn append_child_now(&mut self, child: &crate::macros::Node<Dry>) {
        todo!()
    }

    fn append_child(&mut self, child: &crate::macros::Node<Dry>) {
        self.children.push(child.clone())
    }

    fn insert_child_before(
        &mut self,
        child: &crate::macros::Node<Dry>,
        next_child: Option<&crate::macros::Node<Dry>>,
    ) {
        if let Some(next_child) = next_child {
            let index = self
                .children
                .iter()
                .position(|existing| existing.is_same(&next_child))
                .expect("Child not found");
            self.children.insert(index, child.clone());
        } else {
            self.append_child(child);
        }
    }

    fn replace_child(
        &mut self,
        new_child: &crate::macros::Node<Dry>,
        old_child: &crate::macros::Node<Dry>,
    ) {
        for child in &mut self.children {
            if child.is_same(&old_child) {
                *child = new_child.clone();
                break;
            }
        }
    }

    fn remove_child(&mut self, child: &crate::macros::Node<Dry>) {
        self.children.retain(|existing| !existing.is_same(&child));
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

impl fmt::Display for DryElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.tag)?;

        for (name, value) in &self.attributes {
            write!(f, " {}=\"{}\"", name, encode_double_quoted_attribute(value))?;
        }

        f.write_str(">")?;

        for child in &self.children {
            child.fmt(f)?;
        }

        let has_children = !self.children.is_empty();

        if self.requires_closing_tag() || has_children {
            write!(f, "</{}>", self.tag)?;
        }

        Ok(())
    }
}

fn hydrate_with_new(
    parent: &web_sys::Element,
    child: HydrationNodeData,
    tracker: &mut HydrationStats,
) {
    let new_child = child.dom_node();
    parent.append_child(&new_child).unwrap_throw();
    tracker.node_added(&new_child);
}

#[derive(Clone)]
pub struct DryText(String);

impl DryText {
    pub fn new(text: &str) -> Self {
        Self(text.to_owned())
    }

    pub fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetText {
        if let Some(dom_text) = child.dyn_ref::<web_sys::Text>() {
            let from_dom = || WetText::new_from_text(dom_text.clone());

            match dom_text.text_content() {
                Some(text) if text == self.0 => return from_dom(),
                None if self.0.is_empty() => return from_dom(),
                _ => (),
            }
        }

        let new_text = WetText::new(&self.0);

        let dom_text = new_text.dom_text();
        parent.insert_before(dom_text, Some(child)).unwrap_throw();
        tracker.node_added(dom_text);

        new_text
    }

    pub fn set_text(&mut self, text: String) {
        self.0 = text;
    }
}

impl TextImpl for DryText {
    fn new(text: &str) -> Self {
        todo!()
    }

    fn set_text(&mut self, text: String) {
        todo!()
    }
}

impl Display for DryText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_text_minimal(&self.0))
    }
}

impl From<DryText> for WetText {
    fn from(text: DryText) -> Self {
        WetText::new(&text.0)
    }
}
