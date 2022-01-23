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

fn call2<XValue, XThunk: Into<XValue>, YValue, YThunk: Into<YValue>>(
    x: LazyEnum<XValue, XThunk>,
    y: LazyEnum<YValue, YThunk>,
    f_value: impl FnOnce(XValue, YValue),
    f_thunk: impl FnOnce(XThunk, YThunk),
) {
    match (x, y) {
        (LazyEnum::Value(x), LazyEnum::Value(y)) => f_value(x, y),
        (LazyEnum::Thunk(x), LazyEnum::Thunk(y)) => f_thunk(x.unwrap(), y.unwrap()),
        (mut x, mut y) => {
            x.eval();
            y.eval();
            match (x, y) {
                (LazyEnum::Value(x), LazyEnum::Value(y)) => f_value(x, y),
                _ => panic!("Evaluation of lazy thunk failed"),
            }
        }
    }
}

#[derive(Clone)]
enum LazyEnum<Value, Thunk> {
    Value(Value),
    Thunk(Option<Thunk>),
}

impl<Value, Thunk> LazyEnum<Value, Thunk> {
    fn thunk(thunk: Thunk) -> Self {
        Self::Thunk(Some(thunk))
    }

    fn as_mut(&mut self) -> LazyEnum<&mut Value, &mut Thunk> {
        match self {
            LazyEnum::Value(value) => LazyEnum::Value(value),
            LazyEnum::Thunk(thunk) => LazyEnum::Thunk(thunk.as_mut()),
        }
    }
}

impl<Value, Thunk: Into<Value>> LazyEnum<Value, Thunk> {
    fn eval(&mut self) {
        let thunk = match self {
            LazyEnum::Value(_) => return,
            LazyEnum::Thunk(node) => node.take().unwrap(),
        };

        *self = LazyEnum::Value(thunk.into());
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
        call2(
            self.0.as_mut(),
            child.0,
            |parent, child| parent.store_child(child),
            |parent, child| parent.store_child(child),
        );
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        match &self.0 {
            LazyEnum::Value(elem) => elem.eval_dom_element(),
            LazyEnum::Thunk(elem) => elem.as_ref().unwrap().eval_dom_element(),
        }
    }
}

#[derive(Clone)]
pub struct LazyNode<T>(LazyEnum<StrictNode<T>, StrictNode<T>>);

impl<T: AsRef<web_sys::Node> + Clone + 'static> LazyNode<T> {
    pub fn append_child_now(&mut self, child: &mut impl LazyNodeRef) {
        call2(
            self.0.as_mut(),
            child.as_node_mut().0,
            |parent, child| parent.append_child_now(child),
            |parent, child| parent.append_child_now(child),
        );
    }

    pub fn insert_child_before(&mut self, child: LazyNodeBase, next_child: Option<LazyNodeBase>) {
        // TODO: How to abstract out call3, but with last as Option<Lazy...>
        match (&mut self.0, child.0, next_child) {
            (LazyEnum::Value(parent), LazyEnum::Value(child), None) => {
                parent.insert_child_before(child, None)
            }
            (
                LazyEnum::Value(parent),
                LazyEnum::Value(child),
                Some(LazyNode(LazyEnum::Value(next_child))),
            ) => parent.insert_child_before(child, Some(next_child)),
            (LazyEnum::Thunk(parent), LazyEnum::Thunk(child), None) => parent
                .as_mut()
                .unwrap()
                .insert_child_before(child.unwrap(), None),
            (
                LazyEnum::Thunk(parent),
                LazyEnum::Thunk(child),
                Some(LazyNode(LazyEnum::Thunk(next_child))),
            ) => parent
                .as_mut()
                .unwrap()
                .insert_child_before(child.unwrap(), Some(next_child.unwrap())),
            _ => todo!(),
        }
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl LazyNodeRef,
        next_child: Option<&mut impl LazyNodeRef>,
    ) {
        match (
            &mut self.0,
            child.as_node_mut().0,
            next_child.map(|c| c.as_node_mut().0),
        ) {
            (LazyEnum::Value(parent), LazyEnum::Value(child), None) => {
                let next_child: Option<&mut StrictElement> = None;
                parent.insert_child_before_now(child, next_child)
            }
            (
                LazyEnum::Value(parent),
                LazyEnum::Value(child),
                Some(LazyEnum::Value(next_child)),
            ) => parent.insert_child_before_now(child, Some(next_child)),
            (LazyEnum::Thunk(parent), LazyEnum::Thunk(child), None) => {
                let next_child: Option<&mut StrictElement> = None;
                parent
                    .as_mut()
                    .unwrap()
                    .insert_child_before_now(child.unwrap(), next_child)
            }
            (
                LazyEnum::Thunk(parent),
                LazyEnum::Thunk(child),
                Some(LazyEnum::Thunk(next_child)),
            ) => parent
                .as_mut()
                .unwrap()
                .insert_child_before_now(child.unwrap(), Some(next_child.unwrap())),
            _ => todo!(),
        }
    }

    pub fn replace_child(&mut self, _new_child: LazyNodeBase, _old_child: LazyNodeBase) {
        todo!()
    }

    pub fn remove_child_now(&mut self, child: &mut impl LazyNodeRef) {
        call2(
            self.0.as_mut(),
            child.as_node_mut().0,
            |parent, child| parent.remove_child_now(child),
            |parent, child| parent.remove_child_now(child),
        );
    }

    pub fn remove_child(&mut self, child: LazyNodeBase) {
        call2(
            self.0.as_mut(),
            child.0,
            |parent, child| parent.remove_child(child),
            |parent, child| parent.remove_child(child),
        );
    }

    pub fn clear_children(&mut self) {
        // TODO: call1
        match &mut self.0 {
            LazyEnum::Value(node) => node.clear_children(),
            LazyEnum::Thunk(node) => node.as_mut().unwrap().clear_children(),
        }
    }
}

impl LazyNode<web_sys::Element> {
    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        match &mut self.0 {
            LazyEnum::Value(elem) => elem.attribute(name, value),
            LazyEnum::Thunk(elem) => elem.as_mut().unwrap().attribute(name, value),
        }
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &mut self.0 {
            LazyEnum::Value(elem) => elem.effect(f),
            LazyEnum::Thunk(elem) => elem.as_mut().unwrap().effect(f),
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

    pub fn set_text(&mut self, text: String) {
        match &mut self.0 {
            LazyEnum::Value(node) => node.set_text(text),
            LazyEnum::Thunk(node) => node.as_mut().unwrap().set_text(text),
        }
    }
}

pub trait LazyNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>>;

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>>;

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

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        Lazy(match &mut self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_mut()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_mut().unwrap().as_node_mut()),
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

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        Lazy(match &mut self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_mut()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_mut().unwrap().as_node_mut()),
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

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        Lazy(match &mut self.0 {
            LazyEnum::Value(node) => LazyEnum::Value(node.as_node_mut()),
            LazyEnum::Thunk(node) => LazyEnum::thunk(node.as_mut().unwrap().as_node_mut()),
        })
    }
}
