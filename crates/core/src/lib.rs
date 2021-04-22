#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
pub mod accumulators;
pub mod hooks;

use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

use hooks::{queue_update, state::ReadSignal};
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

fn set_attribute(dom_element: &dom::Element, name: impl AsRef<str>, value: impl AsRef<str>) {
    let dom_element = dom_element.clone();
    let name = name.as_ref().to_string();
    let value = value.as_ref().to_string();

    queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
}

pub trait StaticAttribute {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element);
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        let dom_element = dom_element.clone();
        let name = name.as_ref().to_string();

        if *self {
            queue_update(move || dom_element.set_attribute(&name, "").unwrap());
        } else {
            queue_update(move || dom_element.remove_attribute(&name).unwrap());
        }
    }
}

impl StaticAttribute for String {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        set_attribute(dom_element, name, self);
    }
}

impl StaticAttribute for str {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        set_attribute(dom_element, name, self);
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
        self.set_attribute(name, &builder.element.dom_element);
    }
}

impl<'a> AttributeValue<String> for &'a str {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        set_attribute(&builder.element.dom_element, name, self);
    }
}

impl<'a> AttributeValue<String> for &'a String {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        set_attribute(&builder.element.dom_element, name, self);
    }
}

impl<T> AttributeValue<T> for ReadSignal<T>
where
    T: 'static + StaticAttribute,
{
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        let name = name.as_ref().to_string();
        let dom_element = builder.element.dom_element.clone();
        self.current().set_attribute(&name, &dom_element);

        let updater = self.map({
            let name = name.clone();
            move |new_value| {
                new_value.set_attribute(&name, &dom_element);
            }
        });

        builder.element.reactive_attrs.insert(name, updater);
    }
}

impl<'a, T> AttributeValue<T> for &'a ReadSignal<T>
where
    ReadSignal<T>: AttributeValue<T>,
    T: 'static,
{
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        (*self).set_attribute(name, builder);
    }
}

pub trait Text {
    fn set_text(&self, builder: &mut ElementBuilder);
}

impl<T> Text for T
where
    T: AsRef<str>,
{
    fn set_text(&self, builder: &mut ElementBuilder) {
        if let Some(text_node) = builder.text_node.as_ref() {
            text_node.set_node_value(Some(self.as_ref()));
        } else {
            let text_node = document().create_text_node(self.as_ref());
            builder.append_child(&text_node);
            builder.text_node = Some(text_node);
        }
    }
}

impl<T> Text for ReadSignal<T>
where
    T: 'static + AsRef<str>,
{
    fn set_text(&self, builder: &mut ElementBuilder) {
        self.current().set_text(builder);
        // TODO: Is there a better way, avoiding the unwrap?
        let text_node = builder.text_node.as_ref().unwrap().clone();

        let updater = self.map({
            move |new_value| {
                text_node.set_node_value(Some(new_value.as_ref()));
            }
        });

        builder.element.reactive_text = Some(updater);
    }
}

// TODO: Code to test this
impl<'a, T> Text for &'a ReadSignal<T>
where
    T: 'static,
    ReadSignal<T>: Text,
{
    fn set_text(&self, builder: &mut ElementBuilder) {
        (*self).set_text(builder);
    }
}

impl AttributeValue<String> for ReadSignal<&'static str> {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        self.map(|&value| value.to_string())
            .set_attribute(name, builder);
    }
}

pub struct ElementBuilder {
    element: ElementData,
    text_node: Option<dom::Text>,
}

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder {
            element: ElementData {
                dom_element: document().create_element(tag.as_ref()).unwrap(),
                children: Vec::new(),
                event_callbacks: Vec::new(),
                reactive_attrs: HashMap::new(),
                reactive_text: None,
            },
            text_node: None,
        }
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
        self.element.children.push(child);
        self
    }

    pub fn text(mut self, child: impl Text) -> Self {
        child.set_text(&mut self);
        mem::drop(child);
        self
    }

    pub fn on(mut self, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        {
            let dom_element = self.element.dom_element.clone();
            self.element
                .event_callbacks
                .push(EventCallback::new(dom_element, name, f));
        }

        self
    }

    fn insert_child_before(&mut self, new_node: &dom::Node, reference_node: &dom::Node) {
        let dom_element = self.element.dom_element.clone();
        let new_node = new_node.clone();
        let reference_node = reference_node.clone();

        queue_update(move || {
            dom_element
                .insert_before(&new_node, Some(&reference_node))
                .unwrap();
        });
    }

    fn append_child(&mut self, element: &dom::Node) {
        let dom_element = self.element.dom_element.clone();
        let element = element.clone();

        queue_update(move || {
            dom_element.append_child(&element).unwrap();
        });
    }

    fn remove_child(&mut self, element: &dom::Node) {
        let dom_element = self.element.dom_element.clone();
        let element = element.clone();

        queue_update(move || {
            dom_element.remove_child(&element).unwrap();
        });
    }
}

impl Builder for ElementBuilder {
    type Target = Element;

    fn build(self) -> Self::Target {
        Element(Rc::new(ElementKind::Static(self.element)))
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

enum ElementKind {
    Static(ElementData),
    Reactive(ReadSignal<dom::Element>),
}

#[derive(Clone)]
pub struct Element(Rc<ElementKind>);

// TODO: Find a better way to add all child types to dom
pub trait DomElement {
    type Target: Into<dom::Element> + AsRef<dom::Element> + Clone;

    fn dom_element(&self) -> Self::Target;
}

impl<E> From<ReadSignal<E>> for Element
where
    E: 'static + DomElement,
{
    fn from(elem: ReadSignal<E>) -> Self {
        let dom_element = Rc::new(RefCell::new(elem.current().dom_element().into()));

        let updater = elem.map({
            move |element| {
                let new_dom_element: dom::Element = element.dom_element().into();

                queue_update({
                    let dom_element = dom_element.borrow().clone();
                    let new_dom_element = new_dom_element.clone();

                    move || {
                        dom_element
                            .replace_with_with_node_1(&new_dom_element)
                            .unwrap();
                    }
                });

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
    // TODO: What happens if you set 2 reactive attrs with same name or 2 text attrs?
    reactive_attrs: HashMap<String, ReadSignal<()>>,
    reactive_text: Option<ReadSignal<()>>,
}

impl DomElement for Element {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        match self.0.as_ref() {
            ElementKind::Static(elem) => elem.dom_element.clone(),
            ElementKind::Reactive(elem) => elem.current().clone(),
        }
    }
}

impl DomElement for ElementBuilder {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.element.dom_element.clone()
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

fn window() -> dom::Window {
    dom::window().expect("Window must be available")
}

fn document() -> dom::Document {
    window().document().expect("Window must contain a document")
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
