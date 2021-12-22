//! A reactive interface to the DOM.
pub mod element_list;
pub mod render;
use std::{cell::RefCell, collections::HashMap, future::Future, mem, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    cancelable_future,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt, VecDiff},
    CancelableFutureHandle,
};
use render::{after_render, queue_update};
use silkenweb_reactive::{
    clone,
    containers::{ChangeTrackingVec, DeltaId, VecDelta},
    signal::ReadSignal,
};
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
    delta_id: Rc<RefCell<DeltaId>>,
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
                reactive_attrs: HashMap::new(),
                reactive_children: Vec::new(),
                reactive_with_dom: Vec::new(),
                attribute_signals: HashMap::new(),
                current_children: Rc::new(RefCell::new(Vec::new())),
                children_signal: None,
                signals: Vec::new(),
            },
            text_nodes: Vec::new(),
            delta_id: Rc::default(),
        }
    }

    /// Set an attribute. Attribute values can be reactive.
    pub fn attribute(mut self, name: impl AsRef<str>, value: impl Attribute) -> Self {
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

    pub fn dyn_children(
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

    // TODO: Docs and work out what to do with existing children. `Self::child` and
    // `Self::text` will interfere with reactivity to this.
    pub fn children<T>(mut self, children: &ReadSignal<ChangeTrackingVec<T>>) -> Element
    where
        T: 'static + DomElement,
    {
        for child in children.current().data() {
            self.append_child(&child.dom_element().into());
            // TODO: Can we get rid of `self.element.children`? It just holds on
            // to signals.
        }

        // TODO: We need a map_changes, so we don't respond to the first delta.
        let dom_element = self.dom_element();
        let delta_id = self.delta_id.clone();

        let reactor = children.map(move |children| {
            clone!(dom_element);

            if let Some(delta) = children.delta(&delta_id.borrow()) {
                match delta {
                    VecDelta::Insert { index } => {
                        let index = *index;
                        let len = children.data().len();
                        let child = children[index].dom_element().into();

                        queue_update(move || {
                            if index + 1 == len {
                                dom_element.append_child(&child).unwrap();
                            } else {
                                todo!();
                            }
                        });
                    }
                    VecDelta::Remove { item, .. } => {
                        let child = item.dom_element().into();

                        queue_update(move || {
                            dom_element.remove_child(&child).unwrap();
                        });
                    }
                    VecDelta::Extend { .. } | VecDelta::Set { .. } => todo!(),
                }
            } else {
                // TODO: Clear and re assign children
            }

            delta_id.replace(children.snapshot());
        });

        self.element.reactive_children.push(reactor);

        self.build()
    }

    // TODO: Remove reactivity (it's covered by `dyn_text`)
    /// Add a text node after existing children. The text node can be reactive.
    pub fn text(mut self, child: impl Text) -> Self {
        child.set_text(&mut self);
        mem::drop(child);
        self
    }

    pub fn dyn_text<Sig: 'static + Signal<Item = impl AsRef<str>>>(mut self, text: Sig) -> Self {
        let text_node = document().create_text_node(intern(""));
        self.append_child(&text_node);

        let updater = text.for_each({
            clone!(text_node);

            move |new_value| {
                queue_update({
                    clone!(text_node);
                    // TODO: Do we need to create a string here? Use Into<String> if we do.
                    let new_value = new_value.as_ref().to_string();
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
                    let dom_element: dom::Element = dom_element.borrow().clone();
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

pub trait AttributeValue {
    fn text(&self) -> String;
}

impl AttributeValue for f32 {
    fn text(&self) -> String {
        format!("{}", self)
    }
}

impl AttributeValue for String {
    fn text(&self) -> String {
        self.clone()
    }
}

impl<'a> AttributeValue for &'a str {
    fn text(&self) -> String {
        self.to_string()
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

fn set_attribute(dom_element: &dom::Element, name: impl AsRef<str>, value: impl AsRef<str>) {
    clone!(dom_element);
    let name = name.as_ref().to_string();
    let value = value.as_ref().to_string();

    queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
}

/// A potentially reactive attribute.
pub trait Attribute {
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder);
}

impl<T> Attribute for T
where
    T: StaticAttribute,
{
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        StaticAttribute::set_attribute(&self, name, &builder.element.dom_element);
    }
}

impl<T> Attribute for ReadSignal<T>
where
    T: 'static + StaticAttribute,
{
    fn set_attribute(self, name: impl AsRef<str>, builder: &mut ElementBuilder) {
        let name = name.as_ref().to_string();
        let dom_element = builder.element.dom_element.clone();
        let current: &T = &self.current();
        StaticAttribute::set_attribute(current, &name, &dom_element);

        let updater = self.map({
            clone!(name);
            move |new_value| {
                new_value.set_attribute(&name, &dom_element);
            }
        });

        builder.element.reactive_attrs.insert(name, updater);
    }
}

/* TODO: We need Attribute implementing for f32, bool, String, str etc + Signal types
Signal could be implemented for anything externally (even f32, String etc) - so it's impossible to pass signals to the same method.
We could have a newtype wrapper to distinguish signals:

fn mutable<I, T: Signal<Item = I>>(sig: T) -> Mutable<T> {}

elem.my_attr(mutable(x))

.
.
.

We should allow Option attribute values as well, that remove/don't set on None.
*/
impl<Sig, Attr> Attribute for SignalType<Sig>
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

pub fn signal<Sig: Signal<Item = T>, T>(sig: Sig) -> SignalType<Sig> {
    SignalType(sig)
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
        after_render(move || self(&dom_element));
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
            });
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
        set_static_text(self, builder);
    }
}

impl<'a> Text for &'a String {
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(self, builder);
    }
}

impl Text for String {
    fn set_text(&self, builder: &mut ElementBuilder) {
        set_static_text(self, builder);
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

            builder.element.reactive_children.push(updater);
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
    // TODO: We don't need reactive elements. Reactive children and attributes should cover
    // everything.
    Reactive(ReadSignal<dom::Element>),
}

struct ElementData {
    dom_element: dom::Element,
    children: Vec<Element>,
    event_callbacks: Vec<EventCallback>,
    reactive_attrs: HashMap<String, ReadSignal<()>>,
    reactive_children: Vec<ReadSignal<()>>,
    reactive_with_dom: Vec<ReadSignal<()>>,
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
