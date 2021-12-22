//! A reactive interface to the DOM.
pub mod render;
use std::{cell::RefCell, collections::HashMap, future::Future, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    cancelable_future,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt, VecDiff},
    CancelableFutureHandle,
};
use render::{after_render, queue_update};
use wasm_bindgen::{intern, prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
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

pub fn window() -> dom::Window {
    dom::window().expect("Window must be available")
}

pub fn document() -> dom::Document {
    window().document().expect("Window must contain a document")
}

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: impl AsRef<str>, name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new_in_namespace(namespace, name)
}

/// Build an HTML element.
pub struct ElementBuilder {
    element: ElementData,
    text_nodes: Vec<dom::Text>,
}

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        Self::new_element(document().create_element(tag.as_ref()).unwrap())
    }

    pub fn new_in_namespace(namespace: impl AsRef<str>, tag: impl AsRef<str>) -> Self {
        Self::new_element(
            document()
                .create_element_ns(Some(namespace.as_ref()), tag.as_ref())
                .unwrap(),
        )
    }

    fn new_element(dom_element: dom::Element) -> Self {
        ElementBuilder {
            element: ElementData {
                dom_element,
                children: Vec::new(),
                event_callbacks: Vec::new(),
                attribute_signals: HashMap::new(),
                current_children: Rc::new(RefCell::new(Vec::new())),
                children_signal: None,
                signals: Vec::new(),
            },
            text_nodes: Vec::new(),
        }
    }

    /// Set an attribute. Attribute values can be reactive.
    pub fn attribute<T: StaticAttribute>(
        mut self,
        name: impl AsRef<str>,
        value: impl Attribute<T>,
    ) -> Self {
        self.element.attribute_signals.remove(name.as_ref());
        value.set_attribute(name, &mut self);
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

    pub fn child_signal(self, child: impl Signal<Item = impl Into<Element>>) -> Self {
        todo!()
    }

    pub fn optional_child_signal(
        self,
        child: impl Signal<Item = Option<impl Into<Element>>>,
    ) -> Self {
        todo!()
    }

    // TODO: Docs and work out what to do with existing children. `Self::child` and
    pub fn children_signal(
        mut self,
        children: impl 'static + SignalVec<Item = impl Into<Element>>,
    ) -> Self {
        let parent_elem = self.dom_element();
        let child_elems = self.element.current_children.clone();

        let updater = children.for_each(move |change| {
            let mut child_elems = child_elems.borrow_mut();
            clone!(parent_elem);

            // TODO: Test each match arm
            // TODO: Tidy this code up, and factor some things out.
            match change {
                VecDiff::Replace { values } => {
                    let existing_children = child_elems.clone();

                    *child_elems = values
                        .into_iter()
                        .map(|elem| elem.into().dom_element())
                        .collect();
                    clone!(child_elems);

                    queue_update(move || {
                        for child in existing_children {
                            parent_elem.remove_child(&child).unwrap();
                        }

                        for child in child_elems {
                            parent_elem.append_child(&child).unwrap();
                        }
                    });
                }
                VecDiff::InsertAt { index, value } => todo!(),
                VecDiff::UpdateAt { index, value } => todo!(),
                VecDiff::RemoveAt { index } => todo!(),
                VecDiff::Move {
                    old_index,
                    new_index,
                } => todo!(),
                VecDiff::Push { value } => {
                    let child = value.into().dom_element();
                    child_elems.push(child.clone());

                    queue_update(move || {
                        parent_elem.append_child(&child).unwrap();
                    });
                }
                VecDiff::Pop {} => {
                    let removed_child = child_elems.pop().unwrap();
                    clone!(parent_elem);

                    queue_update(move || {
                        parent_elem.remove_child(&removed_child).unwrap();
                    })
                }
                VecDiff::Clear {} => {
                    let existing_children = child_elems.clone();

                    child_elems.clear();

                    queue_update(move || {
                        for child in existing_children {
                            parent_elem.remove_child(&child).unwrap();
                        }
                    });
                }
            }
            async {}
        });

        let (handle, future) = cancelable_future(updater, || ());

        // TODO: Do we want to spawn this future on RAF
        spawn_local(future);

        self.element.children_signal = Some(handle);

        self
    }

    /// Add a text node after existing children.
    pub fn text(mut self, child: impl AsRef<str>) -> Self {
        let text_node = document().create_text_node(child.as_ref());
        self.append_child(&text_node);
        self.text_nodes.push(text_node);
        self
    }

    pub fn text_signal(mut self, child: impl 'static + Signal<Item = impl Into<String>>) -> Self {
        let text_node = document().create_text_node(intern(""));
        self.append_child(&text_node);

        let updater = child.for_each({
            clone!(text_node);

            move |new_value| {
                queue_update({
                    clone!(text_node);
                    // TODO: Do we need to create a string here? Use Into<String> if we do.
                    let new_value = new_value.into();
                    move || text_node.set_node_value(Some(new_value.as_ref()))
                });
                async {}
            }
        });

        // TODO: Naming of `store_signal` and `updater`
        self.store_signal(updater);
        self
    }

    fn store_signal(&mut self, signal: impl 'static + Future<Output = ()>) {
        let (handle, future) = cancelable_future(signal, || ());

        // TODO: Do we want to spawn this future on RAF
        spawn_local(future);

        self.element.signals.push(handle);
    }

    // TODO: Test
    // TODO: Update docs
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
    pub fn effect<DomType: 'static + JsCast>(self, f: impl 'static + FnOnce(&DomType)) -> Self {
        let dom_element = self.dom_element().dyn_into().unwrap();
        after_render(move || f(&dom_element));

        self
    }

    // TODO: Test
    pub fn effect_signal<T, DomType>(
        mut self,
        sig: impl 'static + Signal<Item = T>,
        f: impl 'static + Clone + Fn(&DomType, T),
    ) -> Self
    where
        T: 'static,
        DomType: 'static + Clone + JsCast,
    {
        let dom_element: DomType = self.dom_element().dyn_into().unwrap();

        let future = sig.for_each(move |x| {
            clone!(dom_element, f);
            after_render(move || f(&dom_element, x));
            async {}
        });

        self.store_signal(future);

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
        Element(Rc::new(self.element))
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
pub struct Element(Rc<ElementData>);

impl DomElement for Element {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.0.dom_element.clone()
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

pub trait AttributeValue {
    fn text(&self) -> String;
}

impl AttributeValue for i8 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for i16 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for i32 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for i64 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for u8 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for u16 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for u32 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for u64 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for f32 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for f64 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for String {
    fn text(&self) -> String {
        self.clone()
    }
}

/// A non-reactive attribute.
pub trait StaticAttribute {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        set_attribute(dom_element, name, self.text());
    }
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

/// Set the attribute, or remove it if the option is `None`.
///
/// Although this only really makes sense for attribute signals, we implement it
/// for `StaticAttribute`s because we fall foul of orphan rules if we try to
/// implement it for all signals of `AttributeValue`s.
impl<T: AttributeValue> StaticAttribute for Option<T> {
    fn set_attribute(&self, name: impl AsRef<str>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.as_ref().to_string();

        match self {
            Some(value) => set_attribute(&dom_element, name, value.text()),
            None => queue_update(move || {
                dom_element.remove_attribute(&name).unwrap();
            }),
        }
    }
}

fn set_attribute(dom_element: &dom::Element, name: impl AsRef<str>, value: impl AsRef<str>) {
    clone!(dom_element);
    let name = name.as_ref().to_string();
    let value = value.as_ref().to_string();

    queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
}

/// A potentially reactive attribute.
pub trait Attribute<T> {
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder);
}

impl<T> Attribute<T> for T
where
    T: StaticAttribute,
{
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        StaticAttribute::set_attribute(&self, name, &builder.element.dom_element);
    }
}

impl<'a> Attribute<String> for &'a str {
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        self.to_string().set_attribute(name, builder);
    }
}

impl<Sig, Attr> Attribute<Attr> for SignalType<Sig>
where
    Attr: StaticAttribute,
    Sig: 'static + Signal<Item = Attr>,
{
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        let name = name.as_ref().to_string();
        let dom_element = builder.dom_element();

        let signal = self.0.for_each({
            clone!(name);
            move |new_value| {
                StaticAttribute::set_attribute(&new_value, &name, &dom_element);
                async {}
            }
        });

        let (handle, future) = cancelable_future(signal, || ());

        // TODO: Do we want to spawn this future on RAF
        spawn_local(future);

        builder.element.attribute_signals.insert(name, handle);
    }
}

pub struct SignalType<T>(T);

/// Create a newtype wrapper around a signal.
pub fn signal<Sig: Signal<Item = T>, T>(sig: Sig) -> SignalType<Sig> {
    SignalType(sig)
}

// TODO(review): Find a better way to add all child types to dom
/// Get a raw Javascript, non-reactive DOM element.
pub trait DomElement {
    type Target: Into<dom::Element> + AsRef<dom::Element> + Clone;

    fn dom_element(&self) -> Self::Target;
}

/// An HTML element builder.
pub trait Builder {
    type Target;

    fn build(self) -> Self::Target;

    fn into_element(self) -> Element;
}

struct ElementData {
    dom_element: dom::Element,
    children: Vec<Element>,
    event_callbacks: Vec<EventCallback>,
    attribute_signals: HashMap<String, SignalHandle>,
    current_children: Rc<RefCell<Vec<dom::Element>>>,
    children_signal: Option<SignalHandle>,
    signals: Vec<SignalHandle>,
}

type SignalHandle = DiscardOnDrop<CancelableFutureHandle>;

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

// TODO: We probably want a better storage API.
// We want to be able to iterator over it like a map using Object::entries and
// Object::keys
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

/// Clone all the identifiers supplied as arguments.
///
/// `clone!(x, y, z);` will generate:
///
/// ```
/// # #[macro_use] extern crate silkenweb_reactive;
/// # let (x, y, z) = (0, 0, 0);
/// let x = x.clone();
/// let y = y.clone();
/// let z = z.clone();
/// ```
///
/// This is useful for capturing variables by copy in closures. For example:
///
/// ```
/// # #[macro_use] extern crate silkenweb_reactive;
/// # let (x, y, z) = (0, 0, 0);
/// # let signal = vec![0].into_iter();
/// # fn do_something(x: u32, y: u32, z: u32) {}
/// signal.map({
///     clone!(x, y, z);
///     move |_| do_something(x, y, z)
/// });
/// ```
#[macro_export]
macro_rules! clone{
    ($($name:ident),* $(,)?) => {
        $(
            let $name = $name.clone();
        )*
    }
}
