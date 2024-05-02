use std::{collections::HashMap, convert::identity, fmt};

use caseless::default_caseless_match_str;
use html_escape::encode_double_quoted_attribute;
use indexmap::IndexMap;
use itertools::Itertools;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{DryChild, LazyElementAction};
use crate::{
    clone,
    dom::{
        hydro::HydroNode,
        private::{DomElement, EventStore, InstantiableDomElement},
        wet::{WetElement, WetNode},
    },
    hydration::HydrationStats,
    node::element::Namespace,
    HEAD_ID_ATTRIBUTE,
};

pub struct SharedDryElement<Node> {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    styles: IndexMap<String, String>,
    children: Vec<Node>,
    shadow_children: Vec<Node>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<Node>,
}

impl<Node> SharedDryElement<Node> {
    fn style_prop_text(&self) -> Option<String> {
        if self.styles.is_empty() {
            return None;
        }

        debug_assert!(!self.attributes.contains_key(STYLE_ATTR));

        Some(
            self.styles
                .iter()
                .map(|(name, value)| format!("{name}: {value};"))
                .join(" "),
        )
    }
}

impl<Node: DryChild> SharedDryElement<Node> {
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        Self {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            styles: IndexMap::new(),
            children: Vec::new(),
            shadow_children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }

    pub fn first_child(&self) -> Option<&Node> {
        self.children.first()
    }

    pub fn next_sibling(&self) -> Option<&Node> {
        self.next_sibling.as_ref()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Node>) {
        self.next_sibling = next_sibling;
    }

    pub fn append_child(&mut self, child: &Node) {
        if let Some(last) = self.children.last_mut() {
            last.set_next_sibling(Some(child));
        }

        self.children.push(child.clone());
    }

    pub fn insert_child_before(&mut self, index: usize, child: &Node, next_child: Option<&Node>) {
        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(child));
        }

        child.set_next_sibling(next_child);

        self.children.insert(index, child.clone());
    }

    pub fn replace_child(&mut self, index: usize, new_child: &Node, old_child: &Node) {
        old_child.set_next_sibling(None);

        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(new_child));
        }

        new_child.set_next_sibling(self.children.get(index + 1));

        self.children[index] = new_child.clone();
    }

    pub fn remove_child(&mut self, index: usize, child: &Node) {
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

    pub fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Node>) {
        for child in children {
            if let Some(previous_child) = self.shadow_children.last_mut() {
                previous_child.set_next_sibling(Some(&child));
            }

            self.shadow_children.push(child);
        }
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
            self.attributes.insert(name.to_owned(), value.to_string());
        } else {
            self.attributes.shift_remove(name);
        }
    }

    pub fn on(
        &mut self,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
        events: &EventStore,
    ) {
        clone!(mut events);

        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f, &mut events)))
    }

    pub fn try_dom_element(&self) -> Option<web_sys::Element> {
        None
    }

    pub fn style_property(&mut self, name: &str, value: &str) {
        self.styles.insert(name.to_owned(), value.to_owned());
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }

    pub fn clone_node(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            styles: self.styles.clone(),
            children: Self::clone_children(&self.children),
            shadow_children: Self::clone_children(&self.shadow_children),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }

    fn clone_children(children: &[Node]) -> Vec<Node> {
        let children: Vec<Node> = children.iter().map(Node::clone_node).collect();

        for (index, child) in children.iter().enumerate() {
            child.set_next_sibling(children.get(index + 1));
        }

        children
    }
}

impl SharedDryElement<HydroNode> {
    pub fn hydrate_child(
        self,
        parent: &web_sys::Node,
        skip_filtered: &impl Fn(Option<web_sys::Node>) -> Option<web_sys::Node>,
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

            let next = skip_filtered(child.next_sibling());
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

    pub fn hydrate(self, dom_elem: &web_sys::Element, tracker: &mut HydrationStats) -> WetElement {
        let existing_namespace = dom_elem.namespace_uri().unwrap_or_default();
        let new_namespace = &self.namespace;
        let new_tag = &self.tag;

        if new_namespace.as_str() == existing_namespace
            && default_caseless_match_str(&dom_elem.tag_name(), new_tag)
        {
            self.hydrate_element(dom_elem, tracker)
        } else {
            let new_dom_elem = new_namespace.create_element(new_tag);

            while let Some(child) = dom_elem.first_child() {
                new_dom_elem.append_child(&child).unwrap_throw();
            }

            dom_elem
                .replace_with_with_node_1(&new_dom_elem)
                .unwrap_throw();
            self.hydrate_element(&new_dom_elem, tracker)
        }
    }

    pub fn hydrate_in_head(self, head: &WetElement, id: &str, tracker: &mut HydrationStats) {
        let id = id.to_string();
        let skip_filtered = move |mut node: Option<web_sys::Node>| {
            while let Some(current) = node {
                if current
                    .dyn_ref::<web_sys::Element>()
                    .is_some_and(|elem| elem.get_attribute(HEAD_ID_ATTRIBUTE).as_ref() == Some(&id))
                {
                    return Some(current);
                }

                node = current.next_sibling();
            }

            None
        };

        Self::hydrate_children(&head.dom_element(), &skip_filtered, self.children, tracker);
    }

    fn hydrate_element(
        self,
        dom_elem: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        self.reconcile_attributes(dom_elem, tracker);
        let mut elem = WetElement::from_element(dom_elem.clone());

        Self::hydrate_children(dom_elem, &identity, self.children, tracker);

        if !self.shadow_children.is_empty() {
            let shadow_root = elem.create_shadow_root();
            Self::hydrate_children(&shadow_root, &identity, self.shadow_children, tracker);
        }

        for event in self.hydrate_actions {
            event(&mut elem);
        }

        elem
    }

    fn hydrate_children(
        dom_elem: &web_sys::Node,
        skip_filtered: &impl Fn(Option<web_sys::Node>) -> Option<web_sys::Node>,
        children: impl IntoIterator<Item = HydroNode>,
        tracker: &mut HydrationStats,
    ) {
        let mut children = children.into_iter();
        let mut current_child = skip_filtered(dom_elem.first_child());

        for child in children.by_ref() {
            if let Some(node) = &current_child {
                let hydrated_elem = child.hydrate_child(dom_elem, skip_filtered, node, tracker);
                current_child = skip_filtered(hydrated_elem.dom_node().next_sibling());
            } else {
                Self::hydrate_with_new(dom_elem, child, tracker);
                break;
            }
        }

        for child in children {
            Self::hydrate_with_new(dom_elem, child, tracker);
        }

        Self::remove_children_from(dom_elem, skip_filtered, current_child, tracker);
    }

    /// Remove `child` and all siblings after `child`
    fn remove_children_from(
        parent: &web_sys::Node,
        skip_filtered: &impl Fn(Option<web_sys::Node>) -> Option<web_sys::Node>,
        mut child: Option<web_sys::Node>,
        tracker: &mut HydrationStats,
    ) {
        while let Some(node) = child {
            let next_child = skip_filtered(node.next_sibling());
            tracker.node_removed(&node);
            parent.remove_child(&node).unwrap_throw();
            child = next_child;
        }
    }

    fn hydrate_with_new(parent: &web_sys::Node, child: HydroNode, tracker: &mut HydrationStats) {
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
            Self::set_attribute(&mut dom_attr_map, name, value, dom_elem, tracker);
        }

        if let Some(style) = self.style_prop_text() {
            Self::set_attribute(&mut dom_attr_map, STYLE_ATTR, &style, dom_elem, tracker)
        }

        for name in dom_attr_map.into_keys() {
            if !name.starts_with("data-silkenweb") {
                tracker.attribute_removed(dom_elem, &name);
                dom_elem.remove_attribute(&name).unwrap_throw();
            }
        }
    }

    fn set_attribute(
        dom_attr_map: &mut HashMap<String, String>,
        name: &str,
        value: &str,
        dom_elem: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) {
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
}

impl<Node: fmt::Display> SharedDryElement<Node> {
    #[cfg(feature = "declarative-shadow-dom")]
    fn write_shadow_dom(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shadow_children.is_empty() {
            return Ok(());
        }

        f.write_str(r#"<template shadowroot="open">"#)?;

        for child in &self.shadow_children {
            child.fmt(f)?;
        }

        f.write_str("</template>")?;

        Ok(())
    }

    #[cfg(not(feature = "declarative-shadow-dom"))]
    fn write_shadow_dom(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl<Node: fmt::Display> fmt::Display for SharedDryElement<Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.tag)?;

        for (name, value) in &self.attributes {
            fmt_attr(f, name, value)?;
        }

        if let Some(style) = self.style_prop_text() {
            fmt_attr(f, STYLE_ATTR, &style)?;
        }

        f.write_str(">")?;

        self.write_shadow_dom(f)?;

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

fn fmt_attr(f: &mut fmt::Formatter, name: &str, value: &str) -> Result<(), fmt::Error> {
    write!(f, " {}=\"{}\"", name, encode_double_quoted_attribute(value))?;
    Ok(())
}

impl<Node: Into<WetNode>> From<SharedDryElement<Node>> for WetElement {
    fn from(dry: SharedDryElement<Node>) -> Self {
        let mut wet = WetElement::new(&dry.namespace, &dry.tag);

        if let Some(style) = dry.style_prop_text() {
            wet.attribute(STYLE_ATTR, &style);
        }

        for (name, value) in dry.attributes {
            wet.attribute(&name, value);
        }

        for child in dry.children {
            wet.append_child(&child.into());
        }

        if !dry.shadow_children.is_empty() {
            wet.attach_shadow_children(dry.shadow_children.into_iter().map(|child| child.into()));
        }

        for action in dry.hydrate_actions {
            action(&mut wet);
        }

        wet
    }
}

const STYLE_ATTR: &str = "style";

const NO_CLOSING_TAG: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta", "param",
    "source", "track", "wbr",
];
