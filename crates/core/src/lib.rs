#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
pub mod hooks;

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
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
        .replace_with_with_node_1(&elem.0.borrow().dom_element)
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

pub struct ElementBuilder(Element);

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder(Element(Rc::new(RefCell::new(ElementData {
            dom_element: document().create_element(tag.as_ref()).unwrap(),
            parent: None,
            children: Vec::new(),
            event_callbacks: Vec::new(),
            generate: None,
        }))))
    }

    pub fn attribute(self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.0
            .dom_element()
            .set_attribute(name.as_ref(), value.as_ref())
            .unwrap();
        self
    }

    pub fn child(self, child: impl Into<Element>) -> Self {
        // TODO: Optimize out unneccessary children?
        let child = child.into();

        self.0.append_child(&child.dom_element());
        child.data_mut().parent = Some(Rc::downgrade(&self.0 .0));
        self.0.data_mut().children.push(child);
        self
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        self.0
            .append_child(&document().create_text_node(child.as_ref()));
        self
    }

    pub fn on(self, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        {
            let mut data = self.0.data_mut();
            let dom_element = data.dom_element.clone();
            data.event_callbacks
                .push(EventCallback::new(dom_element, name, f));
        }

        self
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
pub struct Element(Rc<RefCell<ElementData>>);

impl From<GetState<Element>> for Element {
    fn from(_elem: GetState<Element>) -> Self {
        todo!()
    }
}

impl From<GetState<Element>> for ElementBuilder {
    fn from(elem: GetState<Element>) -> Self {
        elem.into()
    }
}

pub struct ElementData {
    dom_element: dom::Element,
    parent: Option<rc::Weak<RefCell<ElementData>>>,
    children: Vec<Element>,
    event_callbacks: Vec<EventCallback>,
    generate: Option<Box<dyn MkElem>>,
}

impl ElementData {
    fn dom_depth(&self) -> usize {
        self.parent
            .as_ref()
            .map_or(0, |p| 1 + p.upgrade().unwrap().borrow().dom_depth())
    }
}

trait MkElem {
    fn mk_elem(&self) -> Element;
}

impl Element {
    fn append_child(&self, node: &dom::Node) {
        self.dom_element().append_child(node).unwrap();
    }

    fn remove_child(&self, node: &dom::Node) {
        self.dom_element().remove_child(node).unwrap();
    }

    fn dom_element(&self) -> Ref<dom::Element> {
        Ref::map(self.data(), |e| &e.dom_element)
    }

    fn data(&self) -> Ref<ElementData> {
        self.0.borrow()
    }

    fn data_mut(&self) -> RefMut<ElementData> {
        self.0.as_ref().borrow_mut()
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
