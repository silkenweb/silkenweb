use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

use wasm_bindgen::JsValue;

use super::{Dom, DomElement, DomText, GenericElement};
use crate::{
    dom::{InstantiableDom, InstantiableDomElement, InstantiableDomNode},
    hydration::node::Namespace,
};

pub struct Template<D: InstantiableDom, Param>(PhantomData<(D, Param)>);

impl<D: InstantiableDom, Param: 'static> Dom for Template<D, Param> {
    type Element = TemplateElement<D, Param>;
    type Node = TemplateNode<D, Param>;
    type Text = TemplateText<D>;
}

pub struct TemplateElement<D: InstantiableDom, Param> {
    element: D::InstantiableElement,
    initialization_fns: InitializationFns<D, Param>,
}

impl<D, Param> TemplateElement<D, Param>
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

impl<D, Param> DomElement for TemplateElement<D, Param>
where
    D: InstantiableDom,
    Param: 'static,
{
    type Node = TemplateNode<D, Param>;

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

    // TODO: Test
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

    // TODO: Test
    fn replace_child(&mut self, index: usize, new_child: &Self::Node, old_child: &Self::Node) {
        self.element
            .replace_child(index, &new_child.node, &old_child.node);
        self.initialization_fns
            .replace_child(index, new_child.clone());
    }

    // TODO: Test
    fn remove_child(&mut self, index: usize, child: &Self::Node) {
        self.element.remove_child(index, &child.node);
        self.initialization_fns.remove_child(index);
    }

    // TODO: Test
    fn clear_children(&mut self) {
        self.element.clear_children();
        self.initialization_fns.clear_children();
    }

    fn attach_shadow_children(&self, children: impl IntoIterator<Item = Self::Node>) {
        // TODO: We need support shadow children in templates
        todo!()
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

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.element.on(name, f)
    }

    fn dom_element(&self) -> Option<web_sys::Element> {
        self.element.dom_element()
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.element.effect(f)
    }
}

impl<D: InstantiableDom, Param> Clone for TemplateElement<D, Param> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

pub struct TemplateNode<D: InstantiableDom, Param> {
    node: D::Node,
    initialization_fns: InitializationFns<D, Param>,
}

impl<D: InstantiableDom, Param> Clone for TemplateNode<D, Param> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            initialization_fns: self.initialization_fns.clone(),
        }
    }
}

impl<D: InstantiableDom, Param> From<TemplateElement<D, Param>> for TemplateNode<D, Param> {
    fn from(elem: TemplateElement<D, Param>) -> Self {
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

impl<D, Param> From<TemplateText<D>> for TemplateNode<D, Param>
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

struct InitializationFns<D: InstantiableDom, Param>(Rc<RefCell<SharedInitializationFns<D, Param>>>);

impl<D, Param> InitializationFns<D, Param>
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

    fn append_child(&mut self, child: TemplateNode<D, Param>) {
        let mut data = self.0.borrow_mut();

        if !child.initialization_fns.is_empty() {
            let child_count = data.child_count;

            data.children.insert(child_count, child);
        }

        data.child_count += 1;
    }

    fn insert_child(&mut self, index: usize, child: TemplateNode<D, Param>) {
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

    fn replace_child(&mut self, index: usize, child: TemplateNode<D, Param>) {
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

impl<D: InstantiableDom, Param> Clone for InitializationFns<D, Param> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct SharedInitializationFns<D: InstantiableDom, Param> {
    initialization_fns: Vec<InitializeElem<D, Param>>,
    children: BTreeMap<usize, TemplateNode<D, Param>>,
    child_count: usize,
}

impl<D, Param> SharedInitializationFns<D, Param>
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
        let mut element = GenericElement {
            element,
            static_child_count: self.child_count,
            child_vec: None,
            child_builder: None,
            resources: Vec::new(),
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        };

        for f in &self.initialization_fns {
            element = f(element, param);
        }

        if !self.children.is_empty() {
            let mut current_index = 0;
            let mut current_child: D::Node = element.element.clone().into().first_child();

            for (&index, child_template) in &self.children {
                while current_index < index {
                    current_child = current_child.next_sibling();
                    current_index += 1;
                }

                let mut child_elem = child_template
                    .initialization_fns
                    .initialize(current_child.clone().into_element(), param)
                    .build();
                element.resources.append(&mut child_elem.resources);
            }
        }

        element
    }
}

type InitializeElem<D, Param> = Box<dyn Fn(GenericElement<D>, &Param) -> GenericElement<D>>;
