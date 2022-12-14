use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use indexmap::IndexMap;
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
    fn borrow_mut(&mut self) -> RefMut<SharedDryElement> {
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

        set_next_sibling(shared.children.last_mut(), child.clone());
        shared.children.push(child.clone());
    }

    fn insert_child_before(&mut self, index: usize, child: &DryNode, next_child: Option<&DryNode>) {
        todo!()
    }

    fn replace_child(&mut self, _index: usize, new_child: &DryNode, old_child: &DryNode) {
        todo!()
    }

    fn remove_child(&mut self, _index: usize, child: &DryNode) {
        todo!()
    }

    fn clear_children(&mut self) {
        todo!()
    }

    fn add_class(&mut self, name: &str) {
        todo!()
    }

    fn remove_class(&mut self, name: &str) {
        todo!()
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        todo!()
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        todo!()
    }

    fn dom_element(&self) -> Option<web_sys::Element> {
        None
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        todo!()
    }
}

impl InstantiableDomElement for DryElement {
    fn clone_node(&self) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct DryText(Rc<RefCell<SharedDryText>>);

impl DryText {
    fn borrow_mut(&mut self) -> RefMut<SharedDryText> {
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

fn set_next_sibling(node: Option<&mut DryNode>, next_sibling: DryNode) {
    match node {
        Some(node) => match node {
            DryNode::Text(text) => text.borrow_mut().next_sibling = Some(next_sibling),
            DryNode::Element(element) => element.borrow_mut().next_sibling = Some(next_sibling),
        },
        None => (),
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
