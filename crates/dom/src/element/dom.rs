use std::{
    cell::{RefCell, RefMut},
    fmt::{self, Display},
    marker::PhantomData,
    rc::Rc,
};

use wasm_bindgen::JsValue;

use self::{
    real::{RealElement, RealNode, RealText},
    virt::{VElement, VNode, VText},
};
use crate::attribute::Attribute;

mod real;
mod virt;

#[derive(Clone)]
pub struct DomElement(Rc<RefCell<LazyElement>>);

impl DomElement {
    pub fn new(tag: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_thunk(VElement::new(tag)))))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_thunk(
            VElement::new_in_namespace(namespace, tag),
        ))))
    }

    pub fn shrink_to_fit(&mut self) {
        if !all_thunks([self]) {
            self.real().shrink_to_fit();
        }
    }

    pub fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.map(
            (name, f),
            |elem, (name, f)| elem.on(name, f),
            |elem, (name, f)| elem.on(name, f),
        );
    }

    pub fn store_child(&mut self, child: Self) {
        self.map1(child, VElement::store_child, RealElement::store_child);
    }

    pub fn eval_dom_element(&self) -> web_sys::Element {
        self.real().dom_element().clone()
    }

    pub fn hydrate_child(&self, parent: &web_sys::Node, child: &web_sys::Node) -> web_sys::Element {
        self.0
            .borrow_mut()
            .value_with(|virt_elem| virt_elem.hydrate_child(parent, child))
            .dom_element()
            .clone()
    }

    pub fn append_child_now(&mut self, child: &mut impl DomNode) {
        self.map1(child, VElement::append_child, RealElement::append_child_now);
    }

    pub fn insert_child_before(
        &mut self,
        child: impl DomNode + 'static,
        next_child: Option<impl DomNode + 'static>,
    ) {
        self.map2(
            child,
            next_child,
            |parent, mut child, mut next_child| {
                parent.insert_child_before(&mut child, next_child.as_mut())
            },
            RealElement::insert_child_before,
        );
    }

    pub fn insert_child_before_now(
        &mut self,
        child: &mut impl DomNode,
        next_child: Option<&mut impl DomNode>,
    ) {
        self.map2(
            child,
            next_child,
            VElement::insert_child_before,
            RealElement::insert_child_before_now,
        );
    }

    pub fn replace_child(
        &mut self,
        mut new_child: impl DomNode + 'static,
        mut old_child: impl DomNode + 'static,
    ) {
        self.map2(
            &mut new_child,
            &mut old_child,
            VElement::replace_child,
            RealElement::replace_child,
        );
    }

    pub fn remove_child(&mut self, child: &mut (impl DomNode + 'static)) {
        self.map1(child, VElement::remove_child, RealElement::remove_child);
    }

    pub fn remove_child_now(&mut self, child: &mut impl DomNode) {
        self.map1(child, VElement::remove_child, RealElement::remove_child_now);
    }

    pub fn clear_children(&mut self) {
        self.map(
            (),
            |elem, _| elem.clear_children(),
            |elem, _| elem.clear_children(),
        );
    }

    pub fn attribute<A: Attribute>(&mut self, name: &str, value: A) {
        self.map(
            (name, value),
            |elem, (name, value)| elem.attribute(name, value),
            |elem, (name, value)| elem.attribute(name, value),
        );
    }

    pub fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.map(f, VElement::effect, RealElement::effect);
    }

    fn real(&self) -> RefMut<RealElement> {
        RefMut::map(self.0.borrow_mut(), Lazy::value)
    }

    fn virt(&self) -> RefMut<VElement> {
        RefMut::map(self.0.borrow_mut(), Lazy::thunk)
    }

    fn map<T, R>(
        &self,
        arg: T,
        f_virt: impl FnOnce(&mut VElement, T) -> R,
        f_real: impl FnOnce(&mut RealElement, T) -> R,
    ) -> R {
        if all_thunks([self]) {
            f_virt(&mut self.virt(), arg)
        } else {
            f_real(&mut self.real(), arg)
        }
    }

    fn map1<T: Thunk>(
        &self,
        arg: T,
        f_virt: impl FnOnce(&mut VElement, T),
        f_real: impl FnOnce(&mut RealElement, T),
    ) {
        if all_thunks([self, &arg]) {
            f_virt(&mut self.virt(), arg);
        } else {
            f_real(&mut self.real(), arg);
        }
    }

    fn map2<T: Thunk, U: Thunk>(
        &mut self,
        arg0: T,
        arg1: U,
        f_virt: impl FnOnce(&mut VElement, T, U),
        f_real: impl FnOnce(&mut RealElement, T, U),
    ) {
        if all_thunks([self, &arg0, &arg1]) {
            f_virt(&mut self.virt(), arg0, arg1);
        } else {
            f_real(&mut self.real(), arg0, arg1);
        }
    }
}

impl Display for DomElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map(f, |node, f| node.fmt(f), |node, f| node.fmt(f))
    }
}

#[derive(Clone)]
pub struct DomText(Rc<RefCell<LazyText>>);

impl DomText {
    pub fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(Lazy::new_thunk(VText::new(text)))))
    }

    pub fn set_text(&mut self, text: String) {
        self.map(text, VText::set_text, RealText::set_text);
    }

    pub fn hydrate_child(
        &mut self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
    ) -> web_sys::Text {
        // TODO: Validation
        self.0
            .borrow_mut()
            .value_with(|virt_text| virt_text.hydrate_child(parent, child))
            .dom_text()
            .clone()
    }

    fn real(&self) -> RefMut<RealText> {
        RefMut::map(self.0.borrow_mut(), Lazy::value)
    }

    fn virt(&self) -> RefMut<VText> {
        RefMut::map(self.0.borrow_mut(), Lazy::thunk)
    }

    fn map<T, R>(
        &self,
        arg: T,
        f_virt: impl FnOnce(&mut VText, T) -> R,
        f_real: impl FnOnce(&mut RealText, T) -> R,
    ) -> R {
        if all_thunks([self]) {
            f_virt(&mut self.virt(), arg)
        } else {
            f_real(&mut self.real(), arg)
        }
    }
}

impl Display for DomText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map(f, |node, f| node.fmt(f), |node, f| node.fmt(f))
    }
}

/// This is for storing dom nodes without `Box<dyn DomNode>`
#[derive(Clone)]
pub struct DomNodeData(DomNodeEnum);

impl DomNodeData {
    pub fn is_same(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (DomNodeEnum::Element(elem0), DomNodeEnum::Element(elem1)) => {
                Rc::ptr_eq(&elem0.0, &elem1.0)
            }
            (DomNodeEnum::Text(text0), DomNodeEnum::Text(text1)) => Rc::ptr_eq(&text0.0, &text1.0),
            _ => false,
        }
    }

    pub fn hydrate_child(
        &mut self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
    ) -> web_sys::Node {
        match &mut self.0 {
            DomNodeEnum::Element(elem) => elem.hydrate_child(parent, child).into(),
            DomNodeEnum::Text(text) => text.hydrate_child(parent, child).into(),
        }
    }
}

impl Display for DomNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            DomNodeEnum::Element(elem) => elem.fmt(f),
            DomNodeEnum::Text(text) => text.fmt(f),
        }
    }
}

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
pub trait DomNode: Clone + Into<DomNodeData> + RealNode + VNode + Thunk {}

impl RealNode for DomNodeData {
    fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            DomNodeEnum::Element(elem) => elem.dom_node(),
            DomNodeEnum::Text(text) => text.dom_node(),
        }
    }
}

impl VNode for DomNodeData {
    fn node(&self) -> DomNodeData {
        self.clone()
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

impl VNode for DomElement {
    fn node(&self) -> DomNodeData {
        DomNodeData(DomNodeEnum::Element(self.clone()))
    }
}

impl DomNode for DomElement {}

impl RealNode for DomText {
    fn dom_node(&self) -> web_sys::Node {
        self.real().dom_node()
    }
}

impl VNode for DomText {
    fn node(&self) -> DomNodeData {
        DomNodeData(DomNodeEnum::Text(self.clone()))
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
    #[allow(dead_code)]
    fn new_value(x: Value) -> Self {
        Self::Value(x, PhantomData)
    }

    fn new_thunk(x: Thunk) -> Self {
        Self::Thunk(Some(x))
    }
}

impl<Value, Thunk: Into<Value>> Lazy<Value, Thunk> {
    fn value(&mut self) -> &mut Value {
        self.value_with(Thunk::into)
    }

    fn value_with(&mut self, f: impl FnOnce(Thunk) -> Value) -> &mut Value {
        *self = Self::Value(
            match self {
                Lazy::Value(value, _) => return value,
                Lazy::Thunk(thunk) => f(thunk.take().unwrap()),
            },
            PhantomData,
        );

        match self {
            Lazy::Value(value, _) => value,
            Lazy::Thunk(_) => unreachable!(),
        }
    }

    fn thunk(&mut self) -> &mut Thunk {
        match self {
            Lazy::Value(_, _) => panic!("Expected a thunk"),
            Lazy::Thunk(thunk) => return thunk.as_mut().unwrap(),
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
}

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

impl<'a, T: Thunk> Thunk for &'a T {
    fn is_thunk(&self) -> bool {
        T::is_thunk(self)
    }
}

impl<'a, T: Thunk> Thunk for &'a mut T {
    fn is_thunk(&self) -> bool {
        T::is_thunk(self)
    }
}

impl<T: Thunk> Thunk for Option<T> {
    fn is_thunk(&self) -> bool {
        if let Some(x) = self {
            x.is_thunk()
        } else {
            true
        }
    }
}
