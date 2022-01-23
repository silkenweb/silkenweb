// TODO: Enable dead code warnings
#![allow(dead_code)]
use std::future::Future;

use wasm_bindgen::JsValue;

use super::strict::{StrictElement, StrictNode, StrictNodeRef, StrictText};
use crate::attribute::Attribute;

// TODO: Use `StrictElement` as the thunk type for now, just to get us going.
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
        child: &mut impl LazyNodeRef,
        next_child: Option<&mut impl LazyNodeRef>,
    ) {
        if let Some(next_child) = next_child {
            call3(
                self.0.as_mut(),
                child.as_node_mut().0,
                next_child.as_node_mut().0,
                |parent, child, next_child| parent.insert_child_before_now(child, Some(next_child)),
                |parent, child, next_child| parent.insert_child_before_now(child, Some(next_child)),
            );
        } else {
            type NextChild<'a> = Option<&'a mut StrictElement>;

            call2(
                self.0.as_mut(),
                child.as_node_mut().0,
                |parent, child| parent.insert_child_before_now(child, None as NextChild),
                |parent, child| parent.insert_child_before_now(child, None as NextChild),
            );
        }
    }

    pub fn replace_child(&mut self, new_child: LazyNodeBase, old_child: LazyNodeBase) {
        call3(
            self.0.as_mut(),
            new_child.0,
            old_child.0,
            |parent, new_child, old_child| parent.replace_child(new_child, old_child),
            |parent, new_child, old_child| parent.replace_child(new_child, old_child),
        );
    }

    pub fn remove_child_now(&mut self, child: &mut impl LazyNodeRef) {
        call2(
            self.0.as_mut(),
            child.as_node_mut().0,
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
    pub fn to_mut(
        &mut self,
    ) -> Lazy<&mut StrictNode<web_sys::Element>, &mut StrictNode<web_sys::Element>> {
        Lazy(self.0.as_mut())
    }
}

impl Lazy<&mut StrictNode<web_sys::Element>, &mut StrictNode<web_sys::Element>> {
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

impl<T: Into<web_sys::Node>> LazyNode<T> {
    pub fn into_base(self) -> LazyNodeBase {
        LazyNode(map1(self.0, (), |x, _| x.into_base(), |x, _| x.into_base()))
    }
}

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

    fn clone_into_node(&self) -> LazyNode<Self::Node> {
        LazyNode(map1(
            self.as_node_ref().0,
            (),
            |x, _| x.clone(),
            |x, _| x.clone(),
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

fn call3<
    XValue,
    XThunk: Into<XValue>,
    YValue,
    YThunk: Into<YValue>,
    ZValue,
    ZThunk: Into<ZValue>,
>(
    x: LazyEnum<XValue, XThunk>,
    y: LazyEnum<YValue, YThunk>,
    z: LazyEnum<ZValue, ZThunk>,
    f_value: impl FnOnce(XValue, YValue, ZValue),
    f_thunk: impl FnOnce(XThunk, YThunk, ZThunk),
) {
    match (x, y, z) {
        (LazyEnum::Value(x), LazyEnum::Value(y), LazyEnum::Value(z)) => f_value(x, y, z),
        (LazyEnum::Thunk(x), LazyEnum::Thunk(y), LazyEnum::Thunk(z)) => {
            f_thunk(x.unwrap(), y.unwrap(), z.unwrap())
        }
        (mut x, mut y, mut z) => {
            x.eval();
            y.eval();
            z.eval();
            match (x, y, z) {
                (LazyEnum::Value(x), LazyEnum::Value(y), LazyEnum::Value(z)) => f_value(x, y, z),
                _ => panic!("Evaluation of lazy thunk failed"),
            }
        }
    }
}
