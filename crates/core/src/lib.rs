#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
pub mod hooks;

use std::{
    cell::RefCell,
    collections::HashMap,
    mem,
    rc::{self, Rc},
};

use hooks::state::GetState;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

pub fn mount(id: &str, elem: impl Into<Element>) {
    let elem = elem.into();

    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
        .replace_with_with_node_1(&elem.dom_element())
        .unwrap();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));
}

pub fn unmount(id: &str) {
    // TODO: Restore dom to before app was mounted
    APPS.with(|apps| apps.borrow_mut().remove(id));
}

pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

pub trait StaticAttribute {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element);
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        if *self {
            dom_element.set_attribute(name.as_ref(), "true").unwrap();
        } else {
            dom_element.remove_attribute(name.as_ref()).unwrap()
        }
    }
}

impl StaticAttribute for String {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        dom_element.set_attribute(name.as_ref(), &self).unwrap();
    }
}

pub trait AttributeValue<T> {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder);
}

impl<T> AttributeValue<T> for T
where
    T: StaticAttribute,
{
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        self.set_attribute(name, &builder.0.dom_element);
    }
}

impl<'a> AttributeValue<String> for &'a str {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        builder
            .0
            .dom_element
            .set_attribute(name.as_ref(), self)
            .unwrap();
    }
}

impl<'a> AttributeValue<String> for &'a String {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        builder
            .0
            .dom_element
            .set_attribute(name.as_ref(), self)
            .unwrap();
    }
}

impl<T> AttributeValue<T> for GetState<T>
where
    T: 'static + StaticAttribute,
{
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        let owned_name = name.as_ref().to_string();
        let name_key = owned_name.clone();
        let dom_element = builder.0.dom_element.clone();
        self.current().set_attribute(name, &dom_element);

        let updater = self.with(move |new_value| {
            new_value.set_attribute(&owned_name, &dom_element);
        });

        builder.0.reactive_attrs.insert(name_key, updater);
    }
}

pub struct ElementBuilder(ElementData);

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder(ElementData {
            dom_element: document().create_element(tag.as_ref()).unwrap(),
            children: Vec::new(),
            event_callbacks: Vec::new(),
            reactive_attrs: HashMap::new(),
        })
    }

    pub fn attribute<T>(mut self, name: impl AsRef<str>, value: impl AttributeValue<T>) -> Self {
        value.set_attribute(name, &mut self);
        mem::drop(value);
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        // TODO: Optimize out unneccessary children?
        let child = child.into();

        self.append_child(&child.dom_element());
        self.0.children.push(child);
        self
    }

    pub fn text(mut self, child: impl AsRef<str>) -> Self {
        self.append_child(&document().create_text_node(child.as_ref()));
        self
    }

    pub fn on(mut self, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        {
            let dom_element = self.0.dom_element.clone();
            self.0
                .event_callbacks
                .push(EventCallback::new(dom_element, name, f));
        }

        self
    }

    fn append_child(&mut self, element: &dom::Node) {
        self.0.dom_element.append_child(element).unwrap();
    }

    fn remove_child(&mut self, element: &dom::Node) {
        self.0.dom_element.remove_child(element).unwrap();
    }
}

impl Builder for ElementBuilder {
    type Target = Element;

    fn build(self) -> Self::Target {
        Element(Rc::new(ElementKind::Static(self.0)))
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

enum ElementKind {
    Static(ElementData),
    Reactive(GetState<dom::Element>),
}

#[derive(Clone)]
pub struct Element(Rc<ElementKind>);

// TODO: Find a better way to add all child types to dom
pub trait DomElement {
    fn dom_element(&self) -> dom::Element;
}

impl<E> From<GetState<E>> for Element
where
    E: 'static + DomElement,
{
    fn from(elem: GetState<E>) -> Self {
        let dom_element = Rc::new(RefCell::new(elem.current().dom_element()));

        let updater = elem.with({
            move |element| {
                let new_dom_element = element.dom_element();

                dom_element
                    .borrow()
                    .replace_with_with_node_1(&new_dom_element)
                    .unwrap();
                dom_element.replace(new_dom_element.clone());
                new_dom_element
            }
        });

        Self(Rc::new(ElementKind::Reactive(updater)))
    }
}

pub struct ElementData {
    dom_element: dom::Element,
    children: Vec<Element>,
    event_callbacks: Vec<EventCallback>,
    reactive_attrs: HashMap<String, GetState<()>>,
}

impl DomElement for Element {
    fn dom_element(&self) -> dom::Element {
        match self.0.as_ref() {
            ElementKind::Static(elem) => elem.dom_element.clone(),
            ElementKind::Reactive(elem) => elem.current().clone(),
        }
    }
}

impl DomElement for ElementBuilder {
    fn dom_element(&self) -> dom::Element {
        self.0.dom_element.clone()
    }
}

impl Builder for Element {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }
}

pub trait Builder {
    type Target;

    fn build(self) -> Self::Target;
}

struct EventCallback {
    target: dom::Element,
    name: &'static str,
    callback: Closure<dyn FnMut(JsValue)>,
}

impl EventCallback {
    fn new(target: dom::Element, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        let callback = Closure::wrap(Box::new(f) as Box<dyn FnMut(JsValue)>);
        target
            .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref())
            .unwrap();

        Self {
            target,
            name,
            callback,
        }
    }
}

impl Drop for EventCallback {
    fn drop(&mut self) {
        self.target
            .remove_event_listener_with_callback(
                self.name,
                self.callback.as_ref().as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}

pub trait Dependent {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<ElementData>>);
}

fn window() -> dom::Window {
    dom::window().expect("Window must be available")
}

fn document() -> dom::Document {
    window().document().expect("Window must contain a document")
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
