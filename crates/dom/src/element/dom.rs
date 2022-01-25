use std::{
    cell::{RefCell, RefMut},
    future::Future,
    marker::PhantomData,
    rc::Rc,
};

use wasm_bindgen::JsValue;

use self::{
    real::{RealElement, RealNode, RealText},
    virt::{VElement, VText},
};
use crate::{attribute::Attribute, render::queue_update};

mod real;
mod virt;

#[derive(Clone)]
pub struct DomElement(Rc<RefCell<LazyElement>>);

impl DomElement {
    pub fn new(tag: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_value(RealElement::new(
            tag,
        )))))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_value(
            RealElement::new_in_namespace(namespace, tag),
        ))))
    }

    fn real(&self) -> RefMut<RealElement> {
        RefMut::map(self.0.borrow_mut(), Lazy::value)
    }

    pub fn shrink_to_fit(&mut self) {
        self.real().shrink_to_fit();
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
        self.real().spawn_future(future);
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.real().on(name, f);
    }

    pub fn store_child(&mut self, child: Self) {
        self.real().store_child(&mut child.real());
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.real().eval_dom_element()
    }

    pub fn append_child_now(&mut self, child: &mut impl DomNode) {
        if all_thunks([self, child]) {
            todo!()
        } else {
            self.real().append_child(child)
        }
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
        self.real().insert_child_before(child, next_child);
    }

    pub fn replace_child(
        &mut self,
        mut new_child: impl DomNode + 'static,
        mut old_child: impl DomNode + 'static,
    ) {
        let parent = self.clone();

        queue_update(move || parent.real().replace_child(&mut new_child, &mut old_child));
    }

    pub fn remove_child_now(&mut self, child: &mut impl DomNode) {
        self.real().remove_child(child);
    }

    pub fn remove_child(&mut self, mut child: impl DomNode + 'static) {
        let mut parent = self.clone();

        queue_update(move || {
            parent.remove_child_now(&mut child);
        });
    }

    pub fn clear_children(&mut self) {
        let parent = self.clone();

        queue_update(move || parent.real().clear_children())
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        self.real().attribute(name, value);
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.real().effect(f);
    }
}

#[derive(Clone)]
pub struct DomText(Rc<RefCell<LazyText>>);

impl DomText {
    pub fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_value(RealText::new(text)))))
    }

    pub fn set_text(&mut self, text: String) {
        let parent = self.clone();

        queue_update(move || parent.real().set_text(&text));
    }

    fn real(&self) -> RefMut<RealText> {
        RefMut::map(self.0.borrow_mut(), Lazy::value)
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
pub trait DomNode: Clone + Into<DomNodeData> + RealNode + Thunk {}

impl RealNode for DomNodeData {
    fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            DomNodeEnum::Element(elem) => elem.dom_node(),
            DomNodeEnum::Text(text) => text.dom_node(),
        }
    }
}

impl Thunk for DomNodeData {
    fn is_thunk(&self) -> bool {
        match &self.0 {
            DomNodeEnum::Element(elem) => elem.is_thunk(),
            DomNodeEnum::Text(text) => text.is_thunk(),
        }
    }
}

impl DomNode for DomNodeData {
    // TODO: When we get GAT's maybe we can do something like this to avoid multiple
    // borrows:
    //
    // ```rust
    // type BorrowedMut<'a> = DomNodeEnum<RefMut<'a, DomElement>, RefMut<'a, DomText>>;
    //
    // fn borrow_mut(&'a mut self) -> Self::BorrowedMut<'a>;
    // ```
}

impl RealNode for DomElement {
    fn dom_node(&self) -> web_sys::Node {
        self.real().dom_node()
    }
}

impl DomNode for DomElement {}

impl RealNode for DomText {
    fn dom_node(&self) -> web_sys::Node {
        self.real().dom_node()
    }
}

impl DomNode for DomText {}

enum Lazy<Value, Thunk> {
    Value(Value, PhantomData<Thunk>),
    // TODO: feature to disable this at compile time
    #[allow(dead_code)]
    Thunk(Option<Thunk>),
}

impl<Value, Thunk> Lazy<Value, Thunk> {
    fn new_value(x: Value) -> Self {
        Self::Value(x, PhantomData)
    }
}

impl<Value, Thunk: Into<Value>> Lazy<Value, Thunk> {
    fn value(&mut self) -> &mut Value {
        *self = Self::Value(
            match self {
                Lazy::Value(value, _) => return value,
                Lazy::Thunk(thunk) => thunk.take().unwrap().into(),
            },
            PhantomData,
        );

        match self {
            Lazy::Value(value, _) => value,
            Lazy::Thunk(_) => unreachable!(),
        }
    }
}

impl<V, T> Thunk for Rc<RefCell<Lazy<V, T>>> {
    fn is_thunk(&self) -> bool {
        match *self.borrow() {
            Lazy::Value(_, _) => false,
            Lazy::Thunk(_) => true,
        }
    }
}

type LazyElement = Lazy<RealElement, VElement>;
type LazyText = Lazy<RealText, VText>;

// TODO: Typically, we'd check if `is_thunk`, `evaluate` if needed and pass the
// arg on to a function. Each of these will borrow for Rc types. Can we find a
// way around this? Maybe a `Borrowed` type on the `DomNode` trait?
pub trait Thunk {
    fn is_thunk(&self) -> bool;

    // TODO:
    // fn evaluate(&mut self);
}

// TODO: If args are not all thunks, evaluate then all.
fn all_thunks<const COUNT: usize>(args: [&dyn Thunk; COUNT]) -> bool {
    args.into_iter().all(Thunk::is_thunk)
}

impl Thunk for DomElement {
    fn is_thunk(&self) -> bool {
        self.0.is_thunk()
    }
}

impl Thunk for DomText {
    fn is_thunk(&self) -> bool {
        self.0.is_thunk()
    }
}
