use std::{collections::HashMap, fmt};

use caseless::default_caseless_match_str;
use html_escape::encode_double_quoted_attribute;
use indexmap::IndexMap;
use itertools::Itertools;
use silkenweb_base::clone;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{
    hydro::HydroNode,
    private::DomElement,
    wet::{WetElement, WetNode},
    TrackSibling,
};
use crate::{
    hydration::{remove_following_siblings, HydrationStats},
    node::element::Namespace,
};

pub(super) struct SharedDryElement<Child> {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<Child>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<Child>,
}

impl<Child: TrackSibling> SharedDryElement<Child> {
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        Self {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }

    pub fn first_child(&self) -> Option<&Child> {
        self.children.first()
    }

    pub fn next_sibling(&self) -> Option<&Child> {
        self.next_sibling.as_ref()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Child>) {
        self.next_sibling = next_sibling;
    }

    pub fn append_child(&mut self, child: &Child) {
        if let Some(last) = self.children.last_mut() {
            last.set_next_sibling(Some(child));
        }

        self.children.push(child.clone());
    }

    pub fn insert_child_before(&mut self, index: usize, child: &Child, next_child: Option<&Child>) {
        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(child));
        }

        child.set_next_sibling(next_child);

        self.children.insert(index, child.clone());
    }

    pub fn replace_child(&mut self, index: usize, new_child: &Child, old_child: &Child) {
        old_child.set_next_sibling(None);

        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(new_child));
        }

        new_child.set_next_sibling(self.children.get(index + 1));

        self.children[index] = new_child.clone();
    }

    pub fn remove_child(&mut self, index: usize, child: &Child) {
        child.set_next_sibling(None);
        if index > 0 {
            self.children[index - 1].set_next_sibling(self.children.get(index + 1));
        }

        self.children.remove(index);
    }

    pub fn clear_children(&mut self) {
        for child in &self.children {
            child.set_next_sibling(None);
        }

        self.children.clear();
    }

    pub fn attach_shadow_children(&self, _children: impl IntoIterator<Item = Child>) {
        // TODO: We need to support shadow root in dry nodes, we just don't print it yet
        // (as there's no way to).
        todo!()
    }

    pub fn add_class(&mut self, name: &str) {
        self.attributes
            .entry("class".to_owned())
            .and_modify(|class| {
                if !class.split_ascii_whitespace().any(|c| c == name) {
                    if !class.is_empty() {
                        class.push(' ');
                    }

                    class.push_str(name);
                }
            })
            .or_insert_with(|| name.to_owned());
    }

    pub fn remove_class(&mut self, name: &str) {
        if let Some(class) = self.attributes.get_mut("class") {
            *class = class
                .split_ascii_whitespace()
                .filter(|&c| c != name)
                .join(" ");
        }
    }

    pub fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
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

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    pub fn try_dom_element(&self) -> Option<web_sys::Element> {
        None
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }

    pub fn clone_node(&self) -> Self {
        let children = self.children.clone();

        for (index, child) in children.iter().enumerate() {
            child.set_next_sibling(children.get(index + 1));
        }

        Self {
            namespace: self.namespace,
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            children,
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }
}

impl SharedDryElement<HydroNode> {
    pub fn hydrate_child(
        self,
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

        let wet_child: WetElement = self.into();
        let new_element = wet_child.dom_element();
        parent.append_child(&new_element).unwrap_throw();
        tracker.node_added(&new_element);

        wet_child
    }

    fn hydrate_element(
        self,
        dom_elem: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        self.reconcile_attributes(dom_elem, tracker);
        let mut elem = WetElement::from_element(dom_elem.clone());
        let mut current_child = dom_elem.first_child();

        let mut children = self.children.into_iter();

        for child in children.by_ref() {
            if let Some(node) = &current_child {
                let hydrated_elem = child.hydrate_child(dom_elem, node, tracker);
                current_child = hydrated_elem.dom_node().next_sibling();
            } else {
                Self::hydrate_with_new(dom_elem, child, tracker);
                break;
            }
        }

        for child in children {
            Self::hydrate_with_new(dom_elem, child, tracker);
        }

        remove_following_siblings(dom_elem, current_child);

        for event in self.hydrate_actions {
            event(&mut elem);
        }

        elem
    }

    fn hydrate_with_new(parent: &web_sys::Element, child: HydroNode, tracker: &mut HydrationStats) {
        let child = WetNode::from(child);
        let new_child = child.dom_node();
        parent.append_child(new_child).unwrap_throw();
        tracker.node_added(new_child);
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
}

impl<Child: fmt::Display> fmt::Display for SharedDryElement<Child> {
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
        let requires_closing_tag = !NO_CLOSING_TAG.contains(&self.tag.as_str());

        if requires_closing_tag || has_children {
            write!(f, "</{}>", self.tag)?;
        }

        Ok(())
    }
}

impl<Child: Into<WetNode>> From<SharedDryElement<Child>> for WetElement {
    fn from(dry: SharedDryElement<Child>) -> Self {
        let mut wet = WetElement::new(dry.namespace, &dry.tag);

        for (name, value) in dry.attributes {
            wet.attribute(&name, value);
        }

        for child in dry.children {
            wet.append_child(&child.into());
        }

        for action in dry.hydrate_actions {
            action(&mut wet);
        }

        wet
    }
}

type LazyElementAction = Box<dyn FnOnce(&mut WetElement)>;

const NO_CLOSING_TAG: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta", "param",
    "source", "track", "wbr",
];
