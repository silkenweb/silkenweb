//! A reactive interface to the DOM.
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
pub mod element_list;
mod render;
pub mod router;
use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

use render::queue_update;
pub use render::{after_render, render_updates};
use silkenweb_reactive::{clone, signal::ReadSignal};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

/// Mount an element on the document.
///
/// `id` is the html element id of the parent element. The element is added as
/// the last child of this element.
///
/// Mounting an `id` that is already mounted will remove that element.
///
/// An [`Element`] can only appear once in the document. Adding an [`Element`]
/// to the document a second time will move it. It will still require
/// unmounting from both places to free up any resources.
pub fn mount(id: &str, elem: impl Into<Element>) {
    unmount(id);
    let elem = elem.into();

    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
        .append_child(&elem.dom_element())
        .unwrap();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));
}

/// Unmount an element.
///
/// This is mostly useful for testing and checking for memory leaks
pub fn unmount(id: &str) {
    if let Some(elem) = APPS.with(|apps| apps.borrow_mut().remove(id)) {
        elem.dom_element().remove();
    }
}

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

/// Build an HTML element.
pub struct ElementBuilder {
    element: ElementData,
    text_nodes: Vec<dom::Text>,
}

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder {
            element: ElementData {
                dom_element: document().create_element(tag.as_ref()).unwrap(),
                children: Vec::new(),
                event_callbacks: Vec::new(),
                reactive_attrs: HashMap::new(),
                reactive_text: Vec::new(),
                reactive_with_dom: Vec::new(),
            },
            text_nodes: Vec::new(),
        }
    }

    /// Set an attribute. Attribute values can be reactive.
    pub fn attribute<T>(mut self, name: impl AsRef<str>, value: impl AttributeValue<T>) -> Self {
        value.set_attribute(name, &mut self);
        mem::drop(value);
        self
    }

    /// Add a child element after existing children. The child element can be
    /// reactive.
    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();

        self.append_child(&child.dom_element());
        self.element.children.push(child);
        self
    }

    /// Add a text node after existing children. The text node can be reactive.
    pub fn text(mut self, child: impl Text) -> Self {
        child.set_text(&mut self);
        mem::drop(child);
        self
    }

    /// Apply an effect after the next render. For example, to set the focus of
    /// an element:
    ///
    /// ```no_run
    /// # use silkenweb_dom::tag;
    /// # use web_sys::HtmlInputElement;
    /// # let element = tag("input");
    /// element.effect(|elem: &HtmlInputElement| elem.focus().unwrap());
    /// ```
    ///
    /// Effects can be reactive. For example, to set the visibibilty of an item
    /// based on a `hidden` boolean signal:
    ///
    /// ```no_run
    /// # use silkenweb_dom::tag;
    /// # use silkenweb_reactive::signal::Signal;
    /// # use web_sys::HtmlInputElement;
    /// # let element = tag("input");
    /// let hidden = Signal::new(false);
    /// let is_hidden = hidden.read();
    ///
    /// element.effect(is_hidden.map(|&hidden| move |elem: &HtmlInputElement| elem.set_hidden(hidden)));
    /// ```
    pub fn effect<T>(mut self, child: impl Effect<T>) -> Self {
        child.set_effect(&mut self);
        self
    }

    /// Register an event handler.
    ///
    /// `name` is the name of the event. See the [MDN Events] page for a list.
    ///
    /// `f` is the callback when the event fires and will be passed the
    /// javascript `Event` object.
    ///
    /// [MDN Events]: https://developer.mozilla.org/en-US/docs/Web/Events
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
        clone!(new_node, reference_node);

        queue_update(move || {
            dom_element
                .insert_before(&new_node, Some(&reference_node))
                .unwrap();
        });
    }

    fn append_child(&mut self, element: &dom::Node) {
        let dom_element = self.element.dom_element.clone();
        clone!(element);

        queue_update(move || {
            dom_element.append_child(&element).unwrap();
        });
    }

    fn remove_child(&mut self, element: &dom::Node) {
        let dom_element = self.element.dom_element.clone();
        clone!(element);

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

    fn into_element(self) -> Element {
        self.build()
    }
}

impl DomElement for ElementBuilder {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.element.dom_element.clone()
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

/// An HTML element.
///
/// Elements can only appear once in the document. If an element is added again,
/// it will be moved.
#[derive(Clone)]
pub struct Element(Rc<ElementKind>);

impl DomElement for Element {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        match self.0.as_ref() {
            ElementKind::Static(elem) => elem.dom_element.clone(),
            ElementKind::Reactive(elem) => elem.current().clone(),
        }
    }
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
                    clone!(new_dom_element);

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

impl Builder for Element {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }

    fn into_element(self) -> Element {
        self
    }
}

/// A non-reactive attribute.
pub trait StaticAttribute {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element);
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.as_ref().to_string();

        if *self {
            queue_update(move || {
                dom_element.set_attribute(&name, "").unwrap();
            });
        } else {
            queue_update(move || {
                dom_element.remove_attribute(&name).unwrap();
            });
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

fn set_attribute(dom_element: &dom::Element, name: impl AsRef<str>, value: impl AsRef<str>) {
    clone!(dom_element);
    let name = name.as_ref().to_string();
    let value = value.as_ref().to_string();

    queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
}

/// A potentially reactive attribute.
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

impl AttributeValue<String> for ReadSignal<&'static str> {
    fn set_attribute(&self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        self.map(|&value| value.to_string())
            .set_attribute(name, builder);
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
            clone!(name);
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

/// An [`Effect`] that can be applied to an [`Element`] after rendering.
pub trait Effect<T> {
    fn set_effect(self, builder: &mut ElementBuilder);
}

impl<F, T> Effect<T> for F
where
    F: 'static + Fn(&T),
    T: 'static + JsCast,
{
    fn set_effect(self, builder: &mut ElementBuilder) {
        let dom_element = builder.dom_element().dyn_into().unwrap();
        after_render(move || self(&dom_element))
    }
}

impl<F, T> Effect<T> for ReadSignal<F>
where
    F: 'static + Clone + Fn(&T),
    T: 'static + Clone + JsCast,
{
    fn set_effect(self, builder: &mut ElementBuilder) {
        let dom_element: T = builder.dom_element().dyn_into().unwrap();
        let current = self.current().clone();

        after_render({
            clone!(dom_element);
            move || current(&dom_element)
        });

        let updater = self.map(move |new_value| {
            after_render({
                clone!(new_value, dom_element);
                move || new_value(&dom_element)
            })
        });

        builder.element.reactive_with_dom.push(updater);
    }
}

/// A Text element.
pub trait Text {
    fn set_text(&self, builder: &mut ElementBuilder);
}

fn set_static_text<T: AsRef<str>>(text: &T, builder: &mut ElementBuilder) {
    let text_node = document().create_text_node(text.as_ref());
    builder.append_child(&text_node);
    builder.text_nodes.push(text_node);
}

impl<'a> Text for &'a str {
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(self, builder)
    }
}

impl<'a> Text for &'a String {
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(self, builder)
    }
}

impl Text for String {
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(self, builder)
    }
}

impl<T> Text for ReadSignal<T>
where
    T: 'static + AsRef<str>,
{
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(&self.current().as_ref(), builder);

        if let Some(text_node) = builder.text_nodes.last() {
            let updater = self.map({
                clone!(text_node);

                move |new_value| {
                    queue_update({
                        clone!(text_node);
                        let new_value = new_value.as_ref().to_string();
                        move || text_node.set_node_value(Some(new_value.as_ref()))
                    });
                }
            });

            builder.element.reactive_text.push(updater);
        }
    }
}

impl<'a, T> Text for &'a ReadSignal<T>
where
    T: 'static,
    ReadSignal<T>: Text,
{
    fn set_text(&self, builder: &mut ElementBuilder) {
        (*self).set_text(builder);
    }
}

// TODO(review): Find a better way to add all child types to dom
/// Get a raw Javascript, non-reactive DOM element.
pub trait DomElement {
    type Target: Into<dom::Element> + AsRef<dom::Element> + Clone;

    fn dom_element(&self) -> Self::Target;
}

impl<T> DomElement for Option<T>
where
    T: DomElement,
{
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        match self {
            Some(elem) => elem.dom_element().into(),
            None => {
                // We use a hidden `div` element as a placeholder. We'll call
                // `replace_with_with_node_1` if a reactive option changes to `Some`.
                //
                // Comments won't work as their interface is `Node` rather than `Element`, which
                // means we can't call `replace`.
                let none = document().create_element("div").unwrap();
                none.unchecked_ref::<dom::HtmlElement>().set_hidden(true);
                none
            }
        }
    }
}

/// An HTML element builder.
pub trait Builder {
    type Target;

    fn build(self) -> Self::Target;

    fn into_element(self) -> Element;
}

enum ElementKind {
    Static(ElementData),
    Reactive(ReadSignal<dom::Element>),
}

struct ElementData {
    dom_element: dom::Element,
    children: Vec<Element>,
    event_callbacks: Vec<EventCallback>,
    reactive_attrs: HashMap<String, ReadSignal<()>>,
    reactive_text: Vec<ReadSignal<()>>,
    reactive_with_dom: Vec<ReadSignal<()>>,
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

// TODO: We probably want a better storage API.
// We want to be able to iterator over it like a map using Object::entries and Object::keys
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
pub fn local_storage() -> Option<dom::Storage> {
    // TODO: Under what circumstances can these fail?
    window().local_storage().unwrap()
}

/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
pub fn session_storage() -> Option<dom::Storage> {
    window().session_storage().unwrap()
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
