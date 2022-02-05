use std::{
    cell::{RefCell, RefMut},
    fmt::{self, Display},
    rc::Rc,
};

use wasm_bindgen::JsValue;

use self::{
    dry::{DryElement, DryText},
    wet::{WetElement, WetText},
};
use super::lazy::{Lazy, IsDry};
use crate::{attribute::Attribute, event::EventCallback, HydrationTracker};

mod dry;
mod wet;

#[derive(Clone)]
pub struct HydrationElement(Rc<RefCell<Lazy<WetElement, DryElement>>>);

impl HydrationElement {
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new(
            || WetElement::new(namespace, tag),
            || DryElement::new(namespace, tag),
        ))))
    }

    pub fn shrink_to_fit(&mut self) {
        self.borrow_mut()
            .map((), |_, _| (), |elem, _| elem.shrink_to_fit());
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.borrow_mut().map(
            (name, f),
            |elem, (name, f)| elem.on(name, f),
            |elem, (name, f)| elem.on(name, f),
        );
    }

    pub fn store_child(&mut self, child: HydrationNodeData) {
        self.borrow_mut()
            .map1(child, DryElement::store_child, WetElement::store_child);
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.wet().dom_element().clone()
    }

    pub fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut impl HydrationTracker,
    ) -> web_sys::Element {
        self.borrow_mut()
            .wet_with(|dry_elem| dry_elem.hydrate_child(parent, child, tracker))
            .dom_element()
            .clone()
    }

    pub fn append_child_now(&mut self, child: impl HydrationNode) {
        self.borrow_mut().map1(
            child,
            DryElement::append_child,
            WetElement::append_child_now,
        );
    }

    pub fn append_child(&mut self, child: impl HydrationNode) {
        self.borrow_mut()
            .map1(child, DryElement::append_child, WetElement::append_child);
    }

    pub fn insert_child_before(
        &mut self,
        child: impl HydrationNode,
        next_child: Option<impl HydrationNode>,
    ) {
        self.borrow_mut().map2(
            child,
            next_child,
            DryElement::insert_child_before,
            WetElement::insert_child_before,
        );
    }

    pub fn replace_child(&mut self, new_child: impl HydrationNode, old_child: impl HydrationNode) {
        self.borrow_mut().map2(
            new_child,
            old_child,
            DryElement::replace_child,
            WetElement::replace_child,
        );
    }

    pub fn remove_child(&mut self, child: impl HydrationNode) {
        self.borrow_mut()
            .map1(child, DryElement::remove_child, WetElement::remove_child);
    }

    pub fn clear_children(&mut self) {
        self.borrow_mut().map(
            (),
            |elem, _| elem.clear_children(),
            |elem, _| elem.clear_children(),
        );
    }

    pub fn attribute_now<A: Attribute>(&mut self, name: &str, value: A) {
        self.borrow_mut().map(
            (name, value),
            |elem, (name, value)| elem.attribute(name, value),
            |elem, (name, value)| elem.attribute_now(name, value),
        );
    }

    pub fn attribute<A: Attribute + 'static>(&mut self, name: &str, value: A) {
        self.borrow_mut().map(
            (name, value),
            |elem, (name, value)| elem.attribute(name, value),
            |elem, (name, value)| elem.attribute(name, value),
        );
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.borrow_mut()
            .map(f, DryElement::effect, WetElement::effect);
    }

    fn borrow_mut(&self) -> RefMut<Lazy<WetElement, DryElement>> {
        self.0.borrow_mut()
    }

    fn wet(&self) -> RefMut<WetElement> {
        RefMut::map(self.0.borrow_mut(), Lazy::wet)
    }
}

impl fmt::Display for HydrationElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.borrow_mut()
            .map(f, |node, f| node.fmt(f), |node, f| node.fmt(f))
    }
}
#[derive(Clone)]
pub struct HydrationText(Rc<RefCell<Lazy<WetText, DryText>>>);

impl HydrationText {
    pub fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new(
            || WetText::new(text),
            || DryText::new(text),
        ))))
    }

    pub fn set_text(&mut self, text: String) {
        self.borrow_mut()
            .map(text, DryText::set_text, WetText::set_text);
    }

    pub fn hydrate_child(
        &mut self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut impl HydrationTracker,
    ) -> web_sys::Text {
        self.borrow_mut()
            .wet_with(|dry_text| dry_text.hydrate_child(parent, child, tracker))
            .dom_text()
            .clone()
    }

    pub fn eval_dom_text(&self) -> web_sys::Text {
        self.wet().dom_text().clone()
    }

    fn borrow_mut(&self) -> RefMut<Lazy<WetText, DryText>> {
        self.0.borrow_mut()
    }

    fn wet(&self) -> RefMut<WetText> {
        RefMut::map(self.0.borrow_mut(), Lazy::wet)
    }
}

impl Display for HydrationText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.borrow_mut()
            .map(f, |node, f| node.fmt(f), |node, f| node.fmt(f))
    }
}

/// This is for storing dom nodes without `Box<dyn HydrationNode>`
#[derive(Clone)]
pub struct HydrationNodeData(HydrationNodeEnum);

impl HydrationNodeData {
    pub fn is_same(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (HydrationNodeEnum::Element(elem0), HydrationNodeEnum::Element(elem1)) => {
                Rc::ptr_eq(&elem0.0, &elem1.0)
            }
            (HydrationNodeEnum::Text(text0), HydrationNodeEnum::Text(text1)) => {
                Rc::ptr_eq(&text0.0, &text1.0)
            }
            _ => false,
        }
    }

    pub fn hydrate_child(
        &mut self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut impl HydrationTracker,
    ) -> web_sys::Node {
        match &mut self.0 {
            HydrationNodeEnum::Element(elem) => elem.hydrate_child(parent, child, tracker).into(),
            HydrationNodeEnum::Text(text) => text.hydrate_child(parent, child, tracker).into(),
        }
    }

    fn take_wet_event_callbacks(&mut self) -> Vec<EventCallback> {
        match &mut self.0 {
            HydrationNodeEnum::Element(elem) => elem.wet().take_event_callbacks(),
            HydrationNodeEnum::Text(_) => Vec::new(),
        }
    }
}

impl fmt::Display for HydrationNodeData {
    // We don't output any namespaces, as they're not required for HTML that will be
    // parsed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            HydrationNodeEnum::Element(elem) => elem.fmt(f),
            HydrationNodeEnum::Text(text) => text.fmt(f),
        }
    }
}

#[derive(Clone)]
enum HydrationNodeEnum {
    Element(HydrationElement),
    Text(HydrationText),
}

impl From<HydrationElement> for HydrationNodeData {
    fn from(elem: HydrationElement) -> Self {
        Self(HydrationNodeEnum::Element(elem))
    }
}

impl From<HydrationText> for HydrationNodeData {
    fn from(text: HydrationText) -> Self {
        Self(HydrationNodeEnum::Text(text))
    }
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait HydrationNode: WetNode + DryNode + IsDry {}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait DryNode {
    fn into_hydro(self) -> HydrationNodeData;

    fn clone_into_hydro(&self) -> HydrationNodeData;
}

/// A node in the DOM
///
/// This lets us pass a reference to an element or text as a node, without
/// actually constructing a node
pub trait WetNode {
    fn dom_node(&self) -> web_sys::Node;
}

impl<'a, T: HydrationNode> HydrationNode for &'a T {}

impl<'a, T: HydrationNode> HydrationNode for &'a mut T {}

impl WetNode for HydrationNodeData {
    fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            HydrationNodeEnum::Element(elem) => elem.dom_node(),
            HydrationNodeEnum::Text(text) => text.dom_node(),
        }
    }
}

impl DryNode for HydrationNodeData {
    fn into_hydro(self) -> HydrationNodeData {
        self
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        self.clone()
    }
}

impl IsDry for HydrationNodeData {
    fn is_dry(&self) -> bool {
        match &self.0 {
            HydrationNodeEnum::Element(elem) => elem.is_dry(),
            HydrationNodeEnum::Text(text) => text.is_dry(),
        }
    }
}

impl HydrationNode for HydrationNodeData {
    // TODO: When we get GAT's maybe we can do something like this to avoid multiple
    // borrows:
    //
    // ```rust
    // type BorrowedMut<'a> =
    //     HydrationNodeEnum<RefMut<'a, HydrationElement>, RefMut<'a, HydrationText>>;
    //
    // fn borrow_mut(&'a mut self) -> Self::BorrowedMut<'a>;
    // ```
}

impl WetNode for HydrationElement {
    fn dom_node(&self) -> web_sys::Node {
        self.wet().dom_node()
    }
}

impl DryNode for HydrationElement {
    fn into_hydro(self) -> HydrationNodeData {
        HydrationNodeData(HydrationNodeEnum::Element(self))
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        self.clone().into_hydro()
    }
}

impl HydrationNode for HydrationElement {}

impl WetNode for HydrationText {
    fn dom_node(&self) -> web_sys::Node {
        self.wet().dom_node()
    }
}

impl DryNode for HydrationText {
    fn into_hydro(self) -> HydrationNodeData {
        HydrationNodeData(HydrationNodeEnum::Text(self))
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        self.clone().into_hydro()
    }
}

impl HydrationNode for HydrationText {}

impl IsDry for HydrationElement {
    fn is_dry(&self) -> bool {
        self.0.borrow().is_dry()
    }
}

impl IsDry for HydrationText {
    fn is_dry(&self) -> bool {
        self.0.borrow().is_dry()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Namespace {
    /// New elements in the `Html` namespace are created with `create_element`,
    /// thus avoiding converting the namespace to a javascript string.
    Html,
    Other(Option<&'static str>),
}

impl Namespace {
    fn as_str(&self) -> &str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::Other(None) => "",
            Namespace::Other(Some(ns)) => ns,
        }
    }
}
