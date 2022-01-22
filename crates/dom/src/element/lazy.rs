// TODO: Enable dead code warnings
#![allow(dead_code)]
use std::future::Future;

use wasm_bindgen::JsValue;

use super::strict::{StrictElement, StrictNode, StrictNodeBase, StrictNodeRef, StrictText};
use crate::attribute::Attribute;

// TODO: Use `StrictElement` as the thunk type for now, jsut to get us going.
pub struct LazyElement(LazyEnum<StrictElement, StrictElement>);

#[derive(Clone)]
pub struct Lazy<Value, Thunk>(LazyEnum<Value, Thunk>);

#[derive(Clone)]
enum LazyEnum<Value, Thunk> {
    Value(Value),
    Thunk(Option<Thunk>),
}

impl<Value, Thunk> LazyEnum<Value, Thunk> {
    fn thunk(thunk: Thunk) -> Self {
        Self::Thunk(Some(thunk))
    }
}

impl LazyElement {
    pub fn new(tag: &str) -> Self {
        Self(LazyEnum::thunk(StrictElement::new(tag)))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self(LazyEnum::thunk(StrictElement::new_in_namespace(
            namespace, tag,
        )))
    }

    pub fn shrink_to_fit(&mut self) {
        match &mut self.0 {
            LazyEnum::Value(elem) => elem.shrink_to_fit(),
            LazyEnum::Thunk(elem) => elem.as_mut().unwrap().shrink_to_fit(),
        }
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        match &mut self.0 {
            LazyEnum::Value(elem) => elem.spawn_future(future),
            LazyEnum::Thunk(elem) => elem.as_mut().unwrap().spawn_future(future),
        }
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        match &mut self.0 {
            LazyEnum::Value(elem) => elem.on(name, f),
            LazyEnum::Thunk(elem) => elem.as_mut().unwrap().on(name, f),
        }
    }

    pub fn store_child(&mut self, child: Self) {
        match (&mut self.0, child) {
            (LazyEnum::Value(elem), Self(LazyEnum::Value(child))) => elem.store_child(child),
            (LazyEnum::Thunk(elem), Self(LazyEnum::Thunk(child))) => {
                elem.as_mut().unwrap().store_child(child.unwrap())
            }
            (_, mut child) => {
                self.eval();
                child.eval();
                self.store_child(child)
            }
        }
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        match &self.0 {
            LazyEnum::Value(elem) => elem.eval_dom_element(),
            LazyEnum::Thunk(elem) => elem.as_ref().unwrap().eval_dom_element(),
        }
    }

    fn eval(&mut self) {
        let thunk = match &mut self.0 {
            LazyEnum::Value(_) => return,
            LazyEnum::Thunk(elem) => elem.take().unwrap(),
        };

        self.0 = LazyEnum::Value(thunk);
    }
}

#[derive(Clone)]
pub struct LazyNode<T>(LazyEnum<StrictNode<T>, StrictNode<T>>);

impl<T: AsRef<web_sys::Node> + Clone + 'static> LazyNode<T> {
    pub fn append_child_now(&self, child: &impl LazyNodeRef) {
        match (&self.0, child.as_node_ref().0) {
            (LazyEnum::Value(parent), LazyEnum::Value(child)) => parent.append_child_now(child),
            (LazyEnum::Thunk(parent), LazyEnum::Thunk(child)) => {
                parent.as_ref().unwrap().append_child_now(child.unwrap())
            }
            _ => todo!(),
        }
    }

    pub fn insert_child_before(&self, child: LazyNodeBase, next_child: Option<LazyNodeBase>) {
        match (&self.0, child.0, next_child) {
            (LazyEnum::Value(parent), LazyEnum::Value(child), None) => {
                parent.insert_child_before(child, None)
            }
            (
                LazyEnum::Value(parent),
                LazyEnum::Value(child),
                Some(LazyNode(LazyEnum::Value(next_child))),
            ) => parent.insert_child_before(child, Some(next_child)),
            (LazyEnum::Thunk(parent), LazyEnum::Thunk(child), None) => parent
                .as_ref()
                .unwrap()
                .insert_child_before(child.unwrap(), None),
            (
                LazyEnum::Thunk(parent),
                LazyEnum::Thunk(child),
                Some(LazyNode(LazyEnum::Thunk(next_child))),
            ) => parent
                .as_ref()
                .unwrap()
                .insert_child_before(child.unwrap(), Some(next_child.unwrap())),
            _ => todo!(),
        }
    }

    pub fn insert_child_before_now(
        &self,
        child: &impl LazyNodeRef,
        next_child: Option<&impl LazyNodeRef>,
    ) {
        match (
            &self.0,
            child.as_node_ref().0,
            next_child.map(|c| c.as_node_ref().0),
        ) {
            (LazyEnum::Value(parent), LazyEnum::Value(child), None) => {
                let next_child: Option<&StrictElement> = None;
                parent.insert_child_before_now(child, next_child)
            }
            (
                LazyEnum::Value(parent),
                LazyEnum::Value(child),
                Some(LazyEnum::Value(next_child)),
            ) => parent.insert_child_before_now(child, Some(next_child)),
            (LazyEnum::Thunk(parent), LazyEnum::Thunk(child), None) => {
                let next_child: Option<&StrictElement> = None;
                parent
                    .as_ref()
                    .unwrap()
                    .insert_child_before_now(child.unwrap(), next_child)
            }
            (
                LazyEnum::Thunk(parent),
                LazyEnum::Thunk(child),
                Some(LazyEnum::Thunk(next_child)),
            ) => parent
                .as_ref()
                .unwrap()
                .insert_child_before_now(child.unwrap(), Some(next_child.unwrap())),
            _ => todo!(),
        }
    }

    pub fn replace_child(&self, _new_child: LazyNodeBase, _old_child: LazyNodeBase) {
        todo!()
    }

    pub fn remove_child_now(&self, _child: &impl LazyNodeRef) {
        todo!()
    }

    pub fn remove_child(&self, _child: LazyNodeBase) {
        todo!()
    }

    pub fn clear_children(&self) {
        match &self.0 {
            LazyEnum::Value(node) => node.clear_children(),
            LazyEnum::Thunk(node) => node.as_ref().unwrap().clear_children(),
        }
    }
}

impl LazyNode<web_sys::Element> {
    pub fn attribute<A: Attribute>(&self, name: &str, value: A) {
        match &self.0 {
            LazyEnum::Value(elem) => elem.attribute(name, value),
            LazyEnum::Thunk(elem) => elem.as_ref().unwrap().attribute(name, value),
        }
    }

    pub fn effect(&self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &self.0 {
            LazyEnum::Value(elem) => elem.effect(f),
            LazyEnum::Thunk(elem) => elem.as_ref().unwrap().effect(f),
        }
    }
}

pub type LazyNodeBase = LazyNode<web_sys::Node>;

#[derive(Clone)]
pub struct LazyText(LazyEnum<StrictText, StrictText>);

impl LazyText {
    pub fn new(text: &str) -> Self {
        Self(LazyEnum::thunk(StrictText::new(text)))
    }

    pub fn set_text(&self, text: String) {
        match &self.0 {
            LazyEnum::Value(node) => node.set_text(text),
            LazyEnum::Thunk(node) => node.as_ref().unwrap().set_text(text),
        }
    }
}

pub trait LazyNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>>;

    fn clone_into_node(&self) -> Lazy<StrictNodeBase, StrictNodeBase> {
        Lazy(match self.as_node_ref().0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.clone_into_node()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.unwrap().clone_into_node()),
        })
    }
}

impl<T> LazyNodeRef for LazyNode<T>
where
    T: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static,
{
    type Node = T;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        Lazy(match &self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_ref()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_ref().unwrap().as_node_ref()),
        })
    }
}

impl LazyNodeRef for LazyText {
    type Node = web_sys::Text;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        Lazy(match &self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_ref()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_ref().unwrap().as_node_ref()),
        })
    }
}

impl LazyNodeRef for LazyElement {
    type Node = web_sys::Element;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        Lazy(match &self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_ref()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_ref().unwrap().as_node_ref()),
        })
    }
}
