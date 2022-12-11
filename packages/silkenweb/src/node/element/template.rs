use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

use wasm_bindgen::JsValue;

use super::{Dom, DomElement, DomText, GenericElement};
use crate::{dom::DomNode, hydration::node::Namespace};

pub struct Template<D: Dom, Param>(PhantomData<(D, Param)>);

impl<D: Dom, Param: 'static> Dom for Template<D, Param> {
    type Element = TemplateElement<D, Param>;
    type Node = TemplateNode<D, Param>;
    type Text = TemplateText<D>;
}

pub struct TemplateElement<D: Dom, Param> {
    element: D::Element,
    initialization_fns: InitializationFns<D, Param>,
}

impl<D: Dom, Param> TemplateElement<D, Param> {
    pub fn instantiate(&self, param: &Param) -> GenericElement<D> {
        self.initialization_fns.initialize(&self.element, param)
    }

    pub fn on_instantiate(
        &mut self,
        f: impl 'static + Fn(GenericElement<D>, &Param) -> GenericElement<D>,
    ) {
        self.initialization_fns.add_fn(f)
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
            initialization_fns: InitializationFns::new(),
        }
    }

    fn append_child(&mut self, child: &Self::Node) {
        self.element.append_child(&child.node);
        self.initialization_fns.append_child(child.clone());
    }

    // TODO: Update `InitializationFns`
    fn insert_child_before(&mut self, child: &Self::Node, next_child: Option<&Self::Node>) {
        self.element
            .insert_child_before(&child.node, next_child.map(|c| &c.node))
    }

    // TODO: Update `InitializationFns`
    fn replace_child(&mut self, new_child: &Self::Node, old_child: &Self::Node) {
        self.element.replace_child(&new_child.node, &old_child.node)
    }

    // TODO: Update `InitializationFns`
    fn remove_child(&mut self, child: &Self::Node) {
        self.element.remove_child(&child.node)
    }

    // TODO: Update `InitializationFns`
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
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

pub struct TemplateNode<D: Dom, Param> {
    node: D::Node,
    initialization_fns: InitializationFns<D, Param>,
}

impl<D, Param> DomNode for TemplateNode<D, Param>
where
    D: Dom,
    Param: 'static,
{
    type DomType = Template<D, Param>;

    fn try_to_element(self) -> Option<TemplateElement<D, Param>> {
        todo!()
    }

    fn first_child(&self) -> Option<Self> {
        todo!()
    }

    fn next_sibling(&self) -> Option<Self> {
        todo!()
    }
}

impl<D: Dom, Param> Clone for TemplateNode<D, Param> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

impl<D: Dom, Param> From<TemplateElement<D, Param>> for TemplateNode<D, Param> {
    fn from(elem: TemplateElement<D, Param>) -> Self {
        Self {
            node: elem.element.into(),
            initialization_fns: elem.initialization_fns,
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
            initialization_fns: InitializationFns::new(),
        }
    }
}

struct InitializationFns<D: Dom, Param>(Rc<RefCell<SharedInitializationFns<D, Param>>>);

impl<D: Dom, Param> InitializationFns<D, Param> {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(SharedInitializationFns::new())))
    }

    fn add_fn(&mut self, f: impl 'static + Fn(GenericElement<D>, &Param) -> GenericElement<D>) {
        self.0.borrow_mut().initialization_fns.push(Box::new(f))
    }

    fn append_child(&mut self, child: TemplateNode<D, Param>) {
        let mut data = self.0.borrow_mut();

        if !child.initialization_fns.is_empty() {
            let child_count = data.child_count;

            data.children.insert(child_count, child);
        }

        data.child_count += 1;
    }

    fn initialize(&self, element: &D::Element, param: &Param) -> GenericElement<D> {
        let data = self.0.borrow();
        let mut element = GenericElement {
            element: element.clone_node(),
            has_preceding_children: data.child_count > 0,
            child_vec: None,
            child_builder: None,
            resources: Vec::new(),
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        };

        for f in &data.initialization_fns {
            element = f(element, param);
        }

        // TODO: Initialize children

        element
    }

    fn is_empty(&self) -> bool {
        let data = self.0.borrow();

        data.initialization_fns.is_empty() && data.children.is_empty()
    }
}

impl<D: Dom, Param> Clone for InitializationFns<D, Param> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct SharedInitializationFns<D: Dom, Param> {
    initialization_fns: Vec<InitializeElem<D, Param>>,
    children: BTreeMap<usize, TemplateNode<D, Param>>,
    child_count: usize,
}

impl<D: Dom, Param> SharedInitializationFns<D, Param> {
    fn new() -> Self {
        Self {
            initialization_fns: Vec::new(),
            children: BTreeMap::new(),
            child_count: 0,
        }
    }
}

type InitializeElem<D, Param> = Box<dyn Fn(GenericElement<D>, &Param) -> GenericElement<D>>;
