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

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

pub fn mount(id: &str, elem: impl Into<Element>) {
    let elem = elem.into();
    let dom_element = elem.dom_element.clone();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));

    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
        .append_child(&dom_element)
        .unwrap();
}

pub fn unmount(id: &str) {
    APPS.with(|apps| apps.borrow_mut().remove(id));
}

pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

pub struct ElementBuilder(Element);

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder(Element {
            dom_element: document().create_element(tag.as_ref()).unwrap(),
            states: Vec::new(),
            event_callbacks: Vec::new(),
        })
    }

    pub fn attribute(self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.0
            .dom_element
            .set_attribute(name.as_ref(), value.as_ref())
            .unwrap();
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();

        self.0.append_child(&child.dom_element);
        self.0.states.extend(child.states);
        self.0.event_callbacks.extend(child.event_callbacks);
        self
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        self.0
            .append_child(&document().create_text_node(child.as_ref()));
        self
    }

    pub fn on(mut self, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        self.0.event_callbacks.push(Rc::new(EventCallback::new(
            self.0.dom_element.clone(),
            name,
            f,
        )));
        self
    }

    fn clone_build(&self) -> Element {
        self.0.clone()
    }
}

impl Builder for ElementBuilder {
    type Target = Element;

    fn build(self) -> Self::Target {
        self.0
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

#[derive(Clone)]
pub struct Element {
    dom_element: dom::Element,
    states: Vec<Rc<RefCell<dyn OwnedChild>>>,
    event_callbacks: Vec<Rc<EventCallback>>,
}

impl Element {
    fn append_child(&self, node: &dom::Node) {
        self.dom_element.append_child(node).unwrap();
    }

    fn set_parents(&self, parent: Rc<RefCell<dyn OwnedChild>>) {
        for child in &self.states {
            let parent = Rc::downgrade(&parent);
            child.borrow_mut().set_parent(parent);
        }

        mem::drop(parent)
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

fn dom_depth(parent: &Option<rc::Weak<RefCell<dyn OwnedChild>>>) -> usize {
    parent
        .as_ref()
        .and_then(rc::Weak::upgrade)
        .map_or(0, |p| p.borrow().dom_depth() + 1)
}

pub trait OwnedChild {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>);

    fn dom_depth(&self) -> usize;
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
