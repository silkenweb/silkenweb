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

fn map1<XValue, XThunk, Args, ValueResult, ThunkResult>(
    x: LazyEnum<XValue, XThunk>,
    args: Args,
    f_value: impl FnOnce(XValue, Args) -> ValueResult,
    f_thunk: impl FnOnce(XThunk, Args) -> ThunkResult,
) -> LazyEnum<ValueResult, ThunkResult> {
    match x {
        LazyEnum::Value(x) => LazyEnum::Value(f_value(x, args)),
        LazyEnum::Thunk(x) => LazyEnum::thunk(f_thunk(x.unwrap(), args)),
    }
}

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

    fn as_ref(&self) -> LazyEnum<&Value, &Thunk> {
        match self {
            LazyEnum::Value(value) => LazyEnum::Value(value),
            LazyEnum::Thunk(thunk) => LazyEnum::Thunk(thunk.as_ref()),
        }
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
            StrictNode::append_child_now,
            StrictNode::append_child_now,
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
            child.as_node_ref().0,
            StrictNode::remove_child_now,
            StrictNode::remove_child_now,
        );
    }

    pub fn remove_child(&mut self, child: LazyNodeBase) {
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

impl LazyNode<web_sys::Element> {
    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        map1(
            self.0.as_mut(),
            value,
            |elem, value| elem.attribute(name, value),
            |elem, value| elem.attribute(name, value),
        );
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        map1(
            self.0.as_mut(),
            f,
            |elem, f| elem.effect(f),
            |elem, f| elem.effect(f),
        );
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
        map1(
            self.0.as_mut(),
            text,
            |node, text| node.set_text(text),
            |node, text| node.set_text(text),
        );
    }
}

pub trait LazyNodeRef {
    type Node: AsRef<web_sys::Node> + Into<web_sys::Node> + Clone + 'static;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>>;

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>>;

    fn clone_into_node(&self) -> Lazy<StrictNodeBase, StrictNodeBase> {
        Lazy(map1(
            self.as_node_ref().0,
            (),
            |node, _| node.clone_into_node(),
            |node, _| node.clone_into_node(),
        ))
    }
}

impl<T> LazyNodeRef for LazyNode<T>
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

impl LazyNodeRef for LazyText {
    type Node = web_sys::Text;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        as_node_ref(self.0.as_ref())
    }

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        as_node_mut(self.0.as_mut())
    }
}

impl LazyNodeRef for LazyElement {
    type Node = web_sys::Element;

    fn as_node_ref(&self) -> Lazy<&StrictNode<Self::Node>, &StrictNode<Self::Node>> {
        as_node_ref(self.0.as_ref())
    }

    fn as_node_mut(&mut self) -> Lazy<&mut StrictNode<Self::Node>, &mut StrictNode<Self::Node>> {
        as_node_mut(self.0.as_mut())
    }
}

fn as_node_ref<'a, Value: StrictNodeRef, Thunk: StrictNodeRef>(
    node: LazyEnum<&'a Value, &'a Thunk>,
) -> Lazy<&'a StrictNode<Value::Node>, &'a StrictNode<Thunk::Node>> {
    Lazy(map1(
        node,
        (),
        |node, _| node.as_node_ref(),
        |node, _| node.as_node_ref(),
    ))
}

fn as_node_mut<'a, Value: StrictNodeRef, Thunk: StrictNodeRef>(
    node: LazyEnum<&'a mut Value, &'a mut Thunk>,
) -> Lazy<&'a mut StrictNode<Value::Node>, &'a mut StrictNode<Thunk::Node>> {
    Lazy(map1(
        node,
        (),
        |node, _| node.as_node_mut(),
        |node, _| node.as_node_mut(),
    ))
}
