use std::{
    cell::{Ref, RefCell, RefMut},
    future::Future,
    rc::Rc,
};

use wasm_bindgen::JsValue;

use self::real::{RealElement, RealNode, RealText};
use crate::{attribute::Attribute, render::queue_update};

mod real;
mod virt;

// TODO: This will become an Rc<RefCell<Lazy<RealElement, VElement>>>
#[derive(Clone)]
pub struct DomElement(Rc<RefCell<RealElement>>);

impl DomElement {
    pub fn new(tag: &str) -> Self {
        Self(Rc::new(RefCell::new(RealElement::new(tag))))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self(Rc::new(RefCell::new(RealElement::new_in_namespace(
            namespace, tag,
        ))))
    }

    fn data(&self) -> Ref<RealElement> {
        self.0.borrow()
    }

    fn data_mut(&mut self) -> RefMut<RealElement> {
        self.0.borrow_mut()
    }

    pub fn shrink_to_fit(&mut self) {
        self.data_mut().shrink_to_fit();
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.data_mut().spawn_future(future);
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.data_mut().on(name, f);
    }

    pub fn store_child(&mut self, mut child: Self) {
        self.data_mut().store_child(&mut child.data_mut());
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.data().eval_dom_element()
    }

    pub fn append_child_now(&mut self, child: &mut impl DomNode) {
        self.data_mut().append_child(child)
    }

    pub fn insert_child_before(
        &mut self,
        mut child: impl DomNode + 'static,
        mut next_child: Option<impl DomNode + 'static>,
    ) {
        let mut parent = self.clone();

        queue_update(move || {
            parent.insert_child_before_now(&mut child, next_child.as_mut());
        });
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl DomNode,
        next_child: Option<&mut impl DomNode>,
    ) {
        self.data_mut().insert_child_before(child, next_child);
    }

    pub fn replace_child(
        &mut self,
        mut new_child: impl DomNode + 'static,
        mut old_child: impl DomNode + 'static,
    ) {
        let mut parent = self.clone();

        queue_update(move || {
            parent
                .data_mut()
                .replace_child(&mut new_child, &mut old_child)
        });
    }

    pub fn remove_child_now(&mut self, child: &mut impl DomNode) {
        self.data_mut().remove_child(child);
    }

    pub fn remove_child(&mut self, mut child: impl DomNode + 'static) {
        let mut parent = self.clone();

        queue_update(move || {
            parent.remove_child_now(&mut child);
        });
    }

    pub fn clear_children(&mut self) {
        let mut parent = self.clone();

        queue_update(move || parent.data_mut().clear_children())
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        self.data_mut().attribute(name, value);
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.data_mut().effect(f);
    }
}

#[derive(Clone)]
pub struct DomText(Rc<RefCell<RealText>>);

impl DomText {
    pub fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(RealText::new(text))))
    }

    pub fn set_text(&mut self, text: String) {
        let parent = self.clone();

        queue_update(move || parent.0.borrow_mut().set_text(&text));
    }
}

/// This is for storing dom nodes without `Box<dyn DomNode>`
#[derive(Clone)]
pub struct DomNodeData(DomNodeEnum);

impl DomNodeData {}

#[derive(Clone)]
enum DomNodeEnum {
    Element(DomElement),
    Text(DomText),
}

impl From<DomElement> for DomNodeData {
    fn from(elem: DomElement) -> Self {
        Self(DomNodeEnum::Element(elem))
    }
}

impl From<DomText> for DomNodeData {
    fn from(text: DomText) -> Self {
        Self(DomNodeEnum::Text(text))
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait DomNode: Clone + Into<DomNodeData> + RealNode {}

impl RealNode for DomNodeData {
    fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            DomNodeEnum::Element(elem) => elem.dom_node(),
            DomNodeEnum::Text(text) => text.dom_node(),
        }
    }
}

impl DomNode for DomNodeData {}

impl RealNode for DomElement {
    fn dom_node(&self) -> web_sys::Node {
        self.data().dom_node()
    }
}

impl DomNode for DomElement {}

impl RealNode for DomText {
    fn dom_node(&self) -> web_sys::Node {
        self.0.borrow().dom_node()
    }
}

impl DomNode for DomText {}
