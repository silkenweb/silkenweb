use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use wasm_bindgen::JsValue;

use super::{Dom, DomElement, DomText};
use crate::hydration::node::Namespace;

pub struct Template<D: Dom, Param>(PhantomData<(D, Param)>);

impl<D: Dom, Param: 'static> Dom for Template<D, Param> {
    type Element = TemplateElement<D, Param>;
    type Node = TemplateNode<D, Param>;
    type Text = TemplateText<D>;
}

pub struct TemplateElement<D: Dom, Param> {
    element: D::Element,
    instantiation_data: OnInstantiate<Self, Param>,
}

struct OnInstantiate<Element, Param>(Rc<RefCell<OnInstantiateData<Element, Param>>>);

impl<Element, Param> OnInstantiate<Element, Param> {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(OnInstantiateData::new())))
    }

    fn add_fn(&mut self, f: impl 'static + Fn(Element, Param) -> Element) {
        self.0.borrow_mut().instantiate_fns.push(Box::new(f))
    }

    fn append_child(&mut self, child: Self) {
        self.0.borrow_mut().children.push(child);
    }
}

impl<Element, Param> Clone for OnInstantiate<Element, Param> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct OnInstantiateData<Element, Param> {
    instantiate_fns: Vec<Box<dyn Fn(Element, Param) -> Element>>,
    children: Vec<OnInstantiate<Element, Param>>,
}

impl<Element, Param> OnInstantiateData<Element, Param> {
    fn new() -> Self {
        Self {
            instantiate_fns: Vec::new(),
            // TODO: Optimize children. 
            children: Vec::new(),
        }
    }
}

impl<D: Dom, Param> TemplateElement<D, Param> {
    pub fn on_instantiate(&mut self, f: impl 'static + Fn(Self, Param) -> Self) {
        self.instantiation_data.add_fn(f)
    }
}

impl<D, Param> DomElement for TemplateElement<D, Param>
where
    D: Dom,
    Param: 'static,
{
    type Node = TemplateNode<D, Param>;

    fn new(ns: Namespace, tag: &str) -> Self {
        Self {
            element: D::Element::new(ns, tag),
            instantiation_data: OnInstantiate::new(),
        }
    }

    fn append_child(&mut self, child: &Self::Node) {
        self.element.append_child(&child.node);
        self.instantiation_data
            .append_child(child.instantiation_data.clone());
    }

    // TODO: Update `OnInstantiate`
    fn insert_child_before(&mut self, child: &Self::Node, next_child: Option<&Self::Node>) {
        self.element
            .insert_child_before(&child.node, next_child.map(|c| &c.node))
    }

    // TODO: Update `OnInstantiate`
    fn replace_child(&mut self, new_child: &Self::Node, old_child: &Self::Node) {
        self.element.replace_child(&new_child.node, &old_child.node)
    }

    // TODO: Update `OnInstantiate`
    fn remove_child(&mut self, child: &Self::Node) {
        self.element.remove_child(&child.node)
    }

    // TODO: Update `OnInstantiate`
    fn clear_children(&mut self) {
        self.element.clear_children()
    }

    fn add_class(&mut self, name: &str) {
        self.element.add_class(name)
    }

    fn remove_class(&mut self, name: &str) {
        self.element.remove_class(name)
    }

    fn clone_node(&self) -> Self {
        todo!("get rid of `clone_node` completely")
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        self.element.attribute(name, value)
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.element.on(name, f)
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.element.effect(f)
    }

    fn store_child(&mut self, child: Self::Node) {
        self.element.store_child(child.node)
    }
}

impl<D: Dom, Param> Clone for TemplateElement<D, Param> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            instantiation_data: self.instantiation_data.clone(),
        }
    }
}

pub struct TemplateNode<D: Dom, Param> {
    node: D::Node,
    instantiation_data: OnInstantiate<TemplateElement<D, Param>, Param>,
}

impl<D: Dom, Param> From<TemplateElement<D, Param>> for TemplateNode<D, Param> {
    fn from(elem: TemplateElement<D, Param>) -> Self {
        Self {
            node: elem.element.into(),
            instantiation_data: elem.instantiation_data,
        }
    }
}

pub struct TemplateText<D: Dom>(D::Text);

impl<D: Dom> Clone for TemplateText<D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: Dom> DomText for TemplateText<D> {
    fn new(text: &str) -> Self {
        Self(D::Text::new(text))
    }

    fn set_text(&mut self, text: &str) {
        self.0.set_text(text)
    }
}

impl<D: Dom, Param> From<TemplateText<D>> for TemplateNode<D, Param> {
    fn from(elem: TemplateText<D>) -> Self {
        Self {
            node: elem.0.into(),
            instantiation_data: OnInstantiate::new(),
        }
    }
}
