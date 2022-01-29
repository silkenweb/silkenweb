use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use caseless::default_caseless_match_str;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{
    wet::{WetElement, WetText},
    DryNode, HydrationNodeData, WetNode,
};
use crate::{attribute::Attribute, clone, remove_following_siblings};

pub struct DryElement {
    namespace: Option<String>,
    tag: String,
    attributes: HashMap<String, Option<String>>,
    children: Vec<HydrationNodeData>,
    stored_children: Vec<HydrationNodeData>,
    hydrate_actions: Vec<Box<dyn FnOnce(&mut WetElement)>>,
}

impl DryElement {
    pub fn new(tag: &str) -> Self {
        Self::new_element(None, tag)
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(Some(namespace), tag)
    }

    pub fn hydrate_child(self, parent: &web_sys::Node, child: &web_sys::Node) -> WetElement {
        clone!(mut child);

        loop {
            if let Some(elem_child) = child.dyn_ref::<web_sys::Element>() {
                let dom_namespace = elem_child.namespace_uri().unwrap_or_default();
                let dry_namespace = match self.namespace.as_ref().map_or("", String::as_ref) {
                    "" => "http://www.w3.org/1999/xhtml",
                    ns => ns,
                };

                if default_caseless_match_str(dry_namespace, &dom_namespace)
                    && default_caseless_match_str(&elem_child.tag_name(), &self.tag)
                {
                    return self.hydrate_element(elem_child);
                }
            }

            let next = child.next_sibling();
            parent.remove_child(&child).unwrap_throw();

            if let Some(next_child) = next {
                child = next_child;
            } else {
                break;
            }
        }

        let wet_child: WetElement = self.into();
        parent.append_child(wet_child.dom_element()).unwrap_throw();

        wet_child
    }

    fn hydrate_element(self, dom_elem: &web_sys::Element) -> WetElement {
        // TODO: Check namespace, element type and attributes match
        let mut elem = WetElement::new_from_element(dom_elem.clone());
        let mut current_child = dom_elem.first_child();

        let mut children = self.children.into_iter();

        for mut child in children.by_ref() {
            if let Some(node) = &current_child {
                let hydrated_elem = child.hydrate_child(dom_elem, node);
                current_child = hydrated_elem.next_sibling();
            } else {
                dom_elem.append_child(&child.dom_node()).unwrap_throw();
                break;
            }
        }

        for child in children {
            dom_elem.append_child(&child.dom_node()).unwrap_throw();
        }

        if let Some(node) = &current_child {
            remove_following_siblings(dom_elem, node);
            dom_elem.remove_child(node).unwrap_throw();
        }

        for event in self.hydrate_actions {
            event(&mut elem);
        }

        for child in self.stored_children {
            elem.store_child(child);
        }

        elem.shrink_to_fit();

        elem
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    pub fn store_child(&mut self, child: HydrationNodeData) {
        self.stored_children.push(child);
    }

    pub fn append_child(&mut self, child: impl DryNode) {
        self.children.push(child.into_hydro())
    }

    pub fn insert_child_before(&mut self, child: impl DryNode, next_child: Option<impl DryNode>) {
        if let Some(next_child) = next_child {
            let next_child = next_child.into_hydro();
            let index = self
                .children
                .iter()
                .position(|existing| existing.is_same(&next_child))
                .expect("Child not found");
            self.children.insert(index, child.into_hydro());
        } else {
            self.append_child(child);
        }
    }

    pub fn replace_child(&mut self, new_child: impl DryNode, old_child: impl DryNode) {
        let old_child = old_child.into_hydro();

        for child in &mut self.children {
            if child.is_same(&old_child) {
                *child = new_child.clone_into_hydro();
            }
        }
    }

    pub fn remove_child(&mut self, child: impl DryNode) {
        let child = child.into_hydro();

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

    fn new_element(namespace: Option<&str>, tag: &str) -> Self {
        Self {
            namespace: namespace.map(str::to_owned),
            tag: tag.to_owned(),
            attributes: HashMap::new(),
            children: Vec::new(),
            stored_children: Vec::new(),
            hydrate_actions: Vec::new(),
        }
    }

    fn requires_closing_tag(&self) -> bool {
        ![
            "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta",
            "param", "source", "track", "wbr",
        ]
        .contains(&self.tag.as_str())
    }
}

impl Display for DryElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Namespace
        // TODO: tag/attribute name validation
        write!(f, "<{}", self.tag)?;

        for (name, value) in &self.attributes {
            // TODO: escaping of values
            if let Some(value) = value {
                write!(f, " {}=\"{}\"", name, value)?;
            } else {
                write!(f, " {}", name)?;
            }
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

impl From<DryElement> for WetElement {
    fn from(element: DryElement) -> Self {
        let mut elem = if let Some(namespace) = element.namespace {
            WetElement::new_in_namespace(&namespace, &element.tag)
        } else {
            WetElement::new(&element.tag)
        };

        for (name, value) in element.attributes {
            elem.attribute(&name, value);
        }

        for mut child in element.children {
            elem.append_child_now(&mut child);
        }

        for child in element.stored_children {
            elem.store_child(child);
        }

        for event in element.hydrate_actions {
            event(&mut elem);
        }

        elem.shrink_to_fit();

        elem
    }
}

#[derive(Clone)]
pub struct DryText(String);

impl DryText {
    pub fn new(text: &str) -> Self {
        Self(text.to_owned())
    }

    pub fn hydrate_child(&self, parent: &web_sys::Node, child: &web_sys::Node) -> WetText {
        if let Some(dom_text) = child.dyn_ref::<web_sys::Text>() {
            let from_dom = || WetText::new_from_text(dom_text.clone());

            match dom_text.text_content() {
                Some(text) if text == self.0 => return from_dom(),
                None if self.0.is_empty() => return from_dom(),
                _ => (),
            }
        }

        let new_text = WetText::new(&self.0);

        parent
            .insert_before(new_text.dom_text(), Some(child))
            .unwrap_throw();

        new_text
    }

    pub fn set_text(&mut self, text: String) {
        self.0 = text;
    }
}

impl Display for DryText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<DryText> for WetText {
    fn from(text: DryText) -> Self {
        WetText::new(&text.0)
    }
}

impl<'a, T: DryNode> DryNode for &'a T {
    fn into_hydro(self) -> HydrationNodeData {
        DryNode::clone_into_hydro(self)
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        DryNode::clone_into_hydro(*self)
    }
}

impl<'a, T: DryNode> DryNode for &'a mut T {
    fn into_hydro(self) -> HydrationNodeData {
        DryNode::clone_into_hydro(self)
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        DryNode::clone_into_hydro(*self)
    }
}
