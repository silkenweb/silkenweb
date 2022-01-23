use std::future::Future;

use wasm_bindgen::JsValue;

use super::{
    lazy::{call2, call3, map1, Lazy},
    strict::{StrictElement, StrictNode, StrictNodeRef, StrictText},
};
use crate::attribute::Attribute;

// TODO: Use `StrictElement` as the thunk type for now, just to get us going.
pub struct HydrationElement(Lazy<StrictElement, StrictElement>);

impl HydrationElement {
    pub fn new(tag: &str) -> Self {
        Self(Lazy::thunk(StrictElement::new(tag)))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self(Lazy::thunk(StrictElement::new_in_namespace(namespace, tag)))
    }

    pub fn shrink_to_fit(&mut self) {
        map1(
            self.0.as_mut(),
            (),
            |elem, _| elem.shrink_to_fit(),
            |elem, _| elem.shrink_to_fit(),
        );
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        map1(
            self.0.as_mut(),
            future,
            |elem, future| elem.spawn_future(future),
            |elem, future| elem.spawn_future(future),
        );
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        map1(
            self.0.as_mut(),
            f,
            |elem, f| elem.on(name, f),
            |elem, f| elem.on(name, f),
        );
    }

    pub fn store_child(&mut self, child: Self) {
        call2(
            self.0.as_mut(),
            child.0,
            StrictElement::store_child,
            StrictElement::store_child,
        );
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.0.as_ref().eval().eval_dom_element()
    }
}

#[derive(Clone)]
pub struct HydrationNode<T>(Lazy<StrictNode<T>, StrictNode<T>>);

impl<T: AsRef<web_sys::Node> + Clone + 'static> HydrationNode<T> {
    pub fn append_child_now(&mut self, child: &mut impl HydrationNodeRef) {
        call2(
            self.0.as_mut(),
            child.as_node_mut(),
            StrictNode::append_child_now,
            StrictNode::append_child_now,
        );
    }

    pub fn insert_child_before(
        &mut self,
        child: HydrationNodeBase,
        next_child: Option<HydrationNodeBase>,
    ) {
        if let Some(next_child) = next_child {
            call3(
                self.0.as_mut(),
                child.0,
                next_child.0,
                |parent, child, next_child| parent.insert_child_before(child, Some(next_child)),
                |parent, child, next_child| parent.insert_child_before(child, Some(next_child)),
            );
        } else {
            call2(
                self.0.as_mut(),
                child.0,
                |parent, child| parent.insert_child_before(child, None),
                |parent, child| parent.insert_child_before(child, None),
            );
        }
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl HydrationNodeRef,
        next_child: Option<&mut impl HydrationNodeRef>,
    ) {
        if let Some(next_child) = next_child {
            call3(
                self.0.as_mut(),
                child.as_node_mut(),
                next_child.as_node_mut(),
                |parent, child, next_child| parent.insert_child_before_now(child, Some(next_child)),
                |parent, child, next_child| parent.insert_child_before_now(child, Some(next_child)),
            );
        } else {
            type NextChild<'a> = Option<&'a mut StrictElement>;

            call2(
                self.0.as_mut(),
                child.as_node_mut(),
                |parent, child| parent.insert_child_before_now(child, None as NextChild),
                |parent, child| parent.insert_child_before_now(child, None as NextChild),
            );
        }
    }

    pub fn replace_child(&mut self, new_child: HydrationNodeBase, old_child: HydrationNodeBase) {
        call3(
            self.0.as_mut(),
            new_child.0,
            old_child.0,
            |parent, new_child, old_child| parent.replace_child(new_child, old_child),
            |parent, new_child, old_child| parent.replace_child(new_child, old_child),
        );
    }

    pub fn remove_child_now(&mut self, child: &mut impl HydrationNodeRef) {
        call2(
            self.0.as_mut(),
            child.as_node_mut(),
            StrictNode::remove_child_now,
            StrictNode::remove_child_now,
        );
    }

    pub fn remove_child(&mut self, child: HydrationNodeBase) {
        call2(
            self.0.as_mut(),
            child.0,
            StrictNode::remove_child,
            StrictNode::remove_child,
        );
    }

    pub fn clear_children(&mut self) {
        map1(
            self.0.as_mut(),
            (),
            |node, _| node.clear_children(),
            |node, _| node.clear_children(),
        );
    }
}

impl HydrationNode<web_sys::Element> {
    pub fn to_mut(
        &mut self,
    ) -> Lazy<&mut StrictNode<web_sys::Element>, &mut StrictNode<web_sys::Element>> {
        self.0.as_mut()
    }
}

impl Lazy<&mut StrictNode<web_sys::Element>, &mut StrictNode<web_sys::Element>> {
    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        map1(
            self.as_mut(),
            value,
            |elem, value| elem.attribute(name, value),
            |elem, value| elem.attribute(name, value),
        );
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        map1(
            self.as_mut(),
            f,
            |elem, f| elem.effect(f),
            |elem, f| elem.effect(f),
        );
    }
}

pub type HydrationNodeBase = HydrationNode<web_sys::Node>;

impl<T: Into<web_sys::Node>> HydrationNode<T> {
    pub fn into_base(self) -> HydrationNodeBase {
        HydrationNode(map1(self.0, (), |x, _| x.into_base(), |x, _| x.into_base()))
    }
}

#[derive(Clone)]
pub struct HydrationText(Lazy<StrictText, StrictText>);

impl HydrationText {
    pub fn new(text: &str) -> Self {
        Self(Lazy::thunk(StrictText::new(text)))
    }

    pub fn set_text(&mut self, text: String) {
        map1(
            self.0.as_mut(),
            text,
            |node, text| node.set_text(text),
            |node, text| node.set_text(text),
        );
    }
}

pub trait HydrationNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>>;

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>>;

    fn clone_into_node(&self) -> HydrationNode<Self::Node> {
        HydrationNode(map1(
            self.as_node_ref(),
            (),
            |x, _| x.clone(),
            |x, _| x.clone(),
        ))
    }
}

impl<T> HydrationNodeRef for HydrationNode<T>
where
    T: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static,
{
    type Node = T;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        as_node_ref(self.0.as_ref())
    }

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        as_node_mut(self.0.as_mut())
    }
}

impl HydrationNodeRef for HydrationText {
    type Node = web_sys::Text;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        as_node_ref(self.0.as_ref())
    }

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        as_node_mut(self.0.as_mut())
    }
}

impl HydrationNodeRef for HydrationElement {
    type Node = web_sys::Element;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        as_node_ref(self.0.as_ref())
    }

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        as_node_mut(self.0.as_mut())
    }
}

fn as_node_ref<'a, Value: StrictNodeRef, Thunk: StrictNodeRef>(
    node: Lazy<&'a Value, &'a Thunk>,
) -> Lazy<&'a StrictNode<Value::Node>, &'a StrictNode<Thunk::Node>> {
    map1(
        node,
        (),
        |node, _| node.as_node_ref(),
        |node, _| node.as_node_ref(),
    )
}

fn as_node_mut<'a, Value: StrictNodeRef, Thunk: StrictNodeRef>(
    node: Lazy<&'a mut Value, &'a mut Thunk>,
) -> Lazy<&'a mut StrictNode<Value::Node>, &'a mut StrictNode<Thunk::Node>> {
    map1(
        node,
        (),
        |node, _| node.as_node_mut(),
        |node, _| node.as_node_mut(),
    )
}
