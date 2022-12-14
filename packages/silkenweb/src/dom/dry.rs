use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use indexmap::IndexMap;
use itertools::Itertools;
use wasm_bindgen::JsValue;

use super::{
    wet::WetElement, Dom, DomElement, DomText, InstantiableDom, InstantiableDomElement,
    InstantiableDomNode,
};
use crate::hydration::node::Namespace;

pub struct Dry;

impl Dom for Dry {
    type Element = DryElement;
    type Node = DryNode;
    type Text = DryText;
}

impl InstantiableDom for Dry {
    type InstantiableElement = DryElement;
    type InstantiableNode = DryNode;
}

#[derive(Clone)]
pub struct DryElement(Rc<RefCell<SharedDryElement>>);

impl DryElement {
    fn borrow_mut(&self) -> RefMut<SharedDryElement> {
        self.0.as_ref().borrow_mut()
    }
}

struct SharedDryElement {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<DryNode>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<DryNode>,
}

type LazyElementAction = Box<dyn FnOnce(&mut WetElement)>;

impl DomElement for DryElement {
    type Node = DryNode;

    fn new(namespace: Namespace, tag: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedDryElement {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        })))
    }

    fn append_child(&mut self, child: &DryNode) {
        let mut shared = self.borrow_mut();

        if let Some(last) = shared.children.last_mut() {
            last.set_next_sibling(Some(child));
        }

        shared.children.push(child.clone());
    }

    fn insert_child_before(&mut self, index: usize, child: &DryNode, next_child: Option<&DryNode>) {
        let mut shared = self.borrow_mut();

        if index > 0 {
            shared.children[index - 1].set_next_sibling(Some(child));
        }

        child.set_next_sibling(next_child);

        shared.children.insert(index, child.clone());
    }

    fn replace_child(&mut self, index: usize, new_child: &DryNode, old_child: &DryNode) {
        old_child.set_next_sibling(None);
        let mut shared = self.borrow_mut();

        if index > 0 {
            shared.children[index - 1].set_next_sibling(Some(new_child));
        }

        new_child.set_next_sibling(shared.children.get(index + 1));

        shared.children[index] = new_child.clone();
    }

    fn remove_child(&mut self, index: usize, child: &DryNode) {
        child.set_next_sibling(None);
        let mut shared = self.borrow_mut();

        if index > 0 {
            shared.children[index - 1].set_next_sibling(shared.children.get(index + 1));
        }

        shared.children.remove(index);
    }

    fn clear_children(&mut self) {
        let mut shared = self.borrow_mut();

        for child in &shared.children {
            child.set_next_sibling(None);
        }

        shared.children.clear();
    }

    fn add_class(&mut self, name: &str) {
        self.borrow_mut()
            .attributes
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

    fn remove_class(&mut self, name: &str) {
        if let Some(class) = self.borrow_mut().attributes.get_mut("class") {
            *class = class
                .split_ascii_whitespace()
                .filter(|&c| c != name)
                .join(" ");
        }
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        assert_ne!(
            name, "xmlns",
            "\"xmlns\" must be set via a namespace at tag creation time"
        );

        let mut shared = self.borrow_mut();

        if let Some(value) = value.text() {
            shared
                .attributes
                .insert(name.to_owned(), value.into_owned());
        } else {
            shared.attributes.remove(name);
        }
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.borrow_mut()
            .hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    fn dom_element(&self) -> Option<web_sys::Element> {
        None
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.borrow_mut()
            .hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }
}

impl InstantiableDomElement for DryElement {
    fn clone_node(&self) -> Self {
        let shared = self.borrow_mut();
        let children = shared.children.clone();

        for (index, child) in children.iter().enumerate() {
            child.set_next_sibling(children.get(index + 1));
        }

        Self(Rc::new(RefCell::new(SharedDryElement {
            namespace: shared.namespace,
            tag: shared.tag.clone(),
            attributes: shared.attributes.clone(),
            children,
            hydrate_actions: Vec::new(),
            next_sibling: None,
        })))
    }
}

#[derive(Clone)]
pub struct DryText(Rc<RefCell<SharedDryText>>);

impl DryText {
    fn borrow_mut(&self) -> RefMut<SharedDryText> {
        self.0.as_ref().borrow_mut()
    }
}

struct SharedDryText {
    text: String,
    next_sibling: Option<DryNode>,
}

impl DomText for DryText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedDryText {
            text: text.to_owned(),
            next_sibling: None,
        })))
    }

    fn set_text(&mut self, text: &str) {
        self.0.as_ref().borrow_mut().text = text.to_string();
    }
}

#[derive(Clone)]
pub enum DryNode {
    Text(DryText),
    Element(DryElement),
}

impl DryNode {
    fn set_next_sibling(&self, next_sibling: Option<&DryNode>) {
        let next_sibling = next_sibling.map(DryNode::clone);

        match self {
            DryNode::Text(text) => text.borrow_mut().next_sibling = next_sibling,
            DryNode::Element(element) => element.borrow_mut().next_sibling = next_sibling,
        }
    }
}

impl InstantiableDomNode for DryNode {
    type DomType = Dry;

    fn into_element(self) -> DryElement {
        match self {
            Self::Element(element) => element,
            Self::Text(_) => panic!("Type is `Text`, not `Element`"),
        }
    }

    fn first_child(&self) -> Self {
        match self {
            Self::Text(_) => panic!("Text elements don't have children"),
            Self::Element(element) => element.0.borrow().children.first().unwrap().clone(),
        }
    }

    fn next_sibling(&self) -> Self {
        match self {
            DryNode::Text(text) => text
                .0
                .borrow()
                .next_sibling
                .as_ref()
                .expect("No more siblings")
                .clone(),
            DryNode::Element(element) => element
                .0
                .borrow()
                .next_sibling
                .as_ref()
                .expect("No more siblings")
                .clone(),
        }
    }
}

impl From<DryElement> for DryNode {
    fn from(element: DryElement) -> Self {
        Self::Element(element)
    }
}

impl From<DryText> for DryNode {
    fn from(text: DryText) -> Self {
        Self::Text(text)
    }
}
