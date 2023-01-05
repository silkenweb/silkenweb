use std::{cell::RefCell, collections::BTreeMap, fmt, rc::Rc};

use wasm_bindgen::JsValue;

use super::private::EventStore;
use crate::{
    dom::{
        private::{DomElement, DomText, InstantiableDomElement, InstantiableDomNode},
        InstantiableDom,
    },
    node::element::{GenericElement, Namespace},
};

pub struct TemplateElement<Param, D: InstantiableDom> {
    element: D::InstantiableElement,
    initialization_fns: InitializationFns<Param, D>,
}

impl<Param, D> TemplateElement<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    pub fn instantiate(&self, param: &Param) -> GenericElement<D> {
        self.initialization_fns
            .initialize(self.element.clone_node(), param)
    }

    pub fn on_instantiate(
        &mut self,
        f: impl 'static + Fn(GenericElement<D>, &Param) -> GenericElement<D>,
    ) {
        self.initialization_fns.add_fn(f)
    }
}

impl<Param, D> DomElement for TemplateElement<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    type Node = TemplateNode<Param, D>;

    fn new(ns: Namespace, tag: &str) -> Self {
        Self {
            element: D::InstantiableElement::new(ns, tag),
            initialization_fns: InitializationFns::new(),
        }
    }

    fn append_child(&mut self, child: &Self::Node) {
        self.element.append_child(&child.node);
        self.initialization_fns.append_child(child.clone());
    }

    fn insert_child_before(
        &mut self,
        index: usize,
        child: &Self::Node,
        next_child: Option<&Self::Node>,
    ) {
        self.element
            .insert_child_before(index, &child.node, next_child.map(|c| &c.node));
        self.initialization_fns.insert_child(index, child.clone());
    }

    fn replace_child(&mut self, index: usize, new_child: &Self::Node, old_child: &Self::Node) {
        self.element
            .replace_child(index, &new_child.node, &old_child.node);
        self.initialization_fns
            .replace_child(index, new_child.clone());
    }

    fn remove_child(&mut self, index: usize, child: &Self::Node) {
        self.element.remove_child(index, &child.node);
        self.initialization_fns.remove_child(index);
    }

    fn clear_children(&mut self) {
        self.element.clear_children();
        self.initialization_fns.clear_children();
    }

    fn add_class(&mut self, name: &str) {
        self.element.add_class(name)
    }

    fn remove_class(&mut self, name: &str) {
        self.element.remove_class(name)
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        self.element.attribute(name, value)
    }

    fn on(
        &mut self,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
        events: &mut EventStore,
    ) {
        self.element.on(name, f, events)
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        self.element.try_dom_element()
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.element.effect(f)
    }
}

impl<Param, D> fmt::Display for TemplateElement<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

impl<Param, D: InstantiableDom> Clone for TemplateElement<Param, D> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

pub struct TemplateNode<Param, D: InstantiableDom> {
    node: D::Node,
    initialization_fns: InitializationFns<Param, D>,
}

impl<Param, D: InstantiableDom> fmt::Display for TemplateNode<Param, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node.fmt(f)
    }
}

impl<Param, D: InstantiableDom> Clone for TemplateNode<Param, D> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

impl<Param, D: InstantiableDom> From<TemplateElement<Param, D>> for TemplateNode<Param, D> {
    fn from(elem: TemplateElement<Param, D>) -> Self {
        Self {
            node: elem.element.into(),
            initialization_fns: elem.initialization_fns,
        }
    }
}

pub struct TemplateText<D: InstantiableDom>(D::Text);

impl<D: InstantiableDom> Clone for TemplateText<D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: InstantiableDom> DomText for TemplateText<D> {
    fn new(text: &str) -> Self {
        Self(D::Text::new(text))
    }

    fn set_text(&mut self, text: &str) {
        self.0.set_text(text)
    }
}

impl<Param, D> From<TemplateText<D>> for TemplateNode<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    fn from(elem: TemplateText<D>) -> Self {
        Self {
            node: elem.0.into(),
            initialization_fns: InitializationFns::new(),
        }
    }
}

struct InitializationFns<Param, D: InstantiableDom>(Rc<RefCell<SharedInitializationFns<Param, D>>>);

impl<Param, D> InitializationFns<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    fn new() -> Self {
        Self(Rc::new(RefCell::new(SharedInitializationFns::new())))
    }

    fn add_fn(&mut self, f: impl 'static + Fn(GenericElement<D>, &Param) -> GenericElement<D>) {
        self.0.borrow_mut().initialization_fns.push(Box::new(f))
    }

    fn append_child(&mut self, child: TemplateNode<Param, D>) {
        let mut data = self.0.borrow_mut();

        if !child.initialization_fns.is_empty() {
            let child_count = data.child_count;

            data.children.insert(child_count, child);
        }

        data.child_count += 1;
    }

    fn insert_child(&mut self, index: usize, child: TemplateNode<Param, D>) {
        let mut data = self.0.borrow_mut();
        let later_children = data.children.split_off(&index);

        for (i, existing_child) in later_children.into_iter() {
            data.children.insert(i + 1, existing_child);
        }

        if !child.initialization_fns.is_empty() {
            data.children.insert(index, child);
        }

        data.child_count += 1;
    }

    fn replace_child(&mut self, index: usize, child: TemplateNode<Param, D>) {
        let mut data = self.0.borrow_mut();

        if !child.initialization_fns.is_empty() {
            data.children.insert(index, child);
        }
    }

    fn remove_child(&mut self, index: usize) {
        let mut data = self.0.borrow_mut();
        data.children.remove(&index);

        let later_children = data.children.split_off(&index);

        for (i, child) in later_children.into_iter() {
            data.children.insert(i - 1, child);
        }

        data.child_count -= 1;
    }

    fn clear_children(&mut self) {
        let mut data = self.0.borrow_mut();
        data.children.clear();
        data.child_count = 0;
    }

    fn initialize(&self, element: D::InstantiableElement, param: &Param) -> GenericElement<D> {
        self.0.borrow().initialize(element, param)
    }

    fn is_empty(&self) -> bool {
        let data = self.0.borrow();

        data.initialization_fns.is_empty() && data.children.is_empty()
    }
}

impl<Param, D: InstantiableDom> Clone for InitializationFns<Param, D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct SharedInitializationFns<Param, D: InstantiableDom> {
    initialization_fns: Vec<InitializeElem<Param, D>>,
    children: BTreeMap<usize, TemplateNode<Param, D>>,
    child_count: usize,
}

impl<Param, D> SharedInitializationFns<Param, D>
where
    D: InstantiableDom,
    Param: 'static,
{
    fn new() -> Self {
        Self {
            initialization_fns: Vec::new(),
            children: BTreeMap::new(),
            child_count: 0,
        }
    }

    fn initialize(&self, element: D::InstantiableElement, param: &Param) -> GenericElement<D> {
        let has_children = !self.children.is_empty();
        let first_child = has_children.then(|| element.clone().into().first_child());

        let mut element = GenericElement::from_dom(element, self.child_count);

        for f in &self.initialization_fns {
            element = f(element, param);
        }

        if let Some(mut current_child) = first_child {
            let mut current_index = 0;

            for (&index, child_template) in &self.children {
                while current_index < index {
                    current_child = current_child.next_sibling();
                    current_index += 1;
                }

                let child_elem = child_template
                    .initialization_fns
                    .initialize(current_child.clone().into_element(), param);
                element.store_child(child_elem);
            }
        }

        element
    }
}

type InitializeElem<Param, D> = Box<dyn Fn(GenericElement<D>, &Param) -> GenericElement<D>>;
