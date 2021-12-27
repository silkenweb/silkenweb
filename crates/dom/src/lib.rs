//! A reactive interface to the DOM.
// TODO: Split this file up
pub mod render;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    future::Future,
    rc::Rc,
};

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

#[doc(hidden)]
pub mod private {
    pub use futures_signals::map_ref;
}

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
        .append_child(elem.dom_element())
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
pub fn tag(name: &str) -> ElementBuilder {
    ElementBuilder::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: &str, name: &str) -> ElementBuilder {
    ElementBuilder::new_in_namespace(namespace, name)
}

/// Build an HTML element.
pub struct ElementBuilder {
    element: Element,
    children: Rc<RefCell<Children>>,
    attribute_futures: HashMap<String, SignalHandle>,
}

impl ElementBuilder {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document().create_element(tag).unwrap())
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document().create_element_ns(Some(namespace), tag).unwrap())
    }

    fn new_element(dom_element: dom::Element) -> Self {
        Self {
            element: Element {
                dom_element: dom_element.clone(),
                event_callbacks: Vec::new(),
                futures: Vec::new(),
            },
            children: Rc::new(RefCell::new(Children::new(dom_element))),
            attribute_futures: HashMap::new(),
        }
    }

    /// Set an attribute. Attribute values can be reactive.
    pub fn attribute<T: StaticAttribute>(
        mut self,
        name: impl Into<String>,
        value: impl Attribute<T>,
    ) -> Self {
        let name = name.into();
        self.attribute_futures.remove(&name);
        value.set_attribute(name, &mut self);
        self
    }

    /// Add a child element after existing children.
    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();
        self.children
            .borrow_mut()
            .append_new_group(child.dom_element());
        self.element.event_callbacks.extend(child.event_callbacks);
        self.element.futures.extend(child.futures);

        self
    }

    pub fn child_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = impl Into<Element>>,
    ) -> Self {
        let group_index = self.children.borrow_mut().new_group();
        let children = self.children.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            let child = child.into();
            children
                .borrow_mut()
                .add_child(group_index, child.dom_element());
            _child_storage = Some(child);
            async {}
        });

        self.store_future(updater);

        self
    }

    pub fn optional_child_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = Option<impl Into<Element>>>,
    ) -> Self {
        let group_index = self.children.borrow_mut().new_group();
        let children = self.children.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            if let Some(child) = child {
                let child = child.into();
                children
                    .borrow_mut()
                    .add_child(group_index, child.dom_element());
                _child_storage = Some(child);
            } else {
                children.borrow_mut().remove_child(group_index);
                _child_storage = None;
            }

            async {}
        });

        self.store_future(updater);

        self
    }

    // TODO: Docs
    // TODO: tests
    pub fn children_signal(
        mut self,
        children: impl 'static + SignalVec<Item = impl Into<Element>>,
    ) -> Self {
        let group_index = self.children.borrow_mut().new_group();
        let child_vec = ChildVec::new(
            self.dom_element().clone(),
            self.children.clone(),
            group_index,
        );

        let updater = children.for_each(move |update| {
            child_vec.borrow_mut().apply_update(update);
            async {}
        });

        self.store_future(updater);
        self
    }

    /// Add a text node after existing children.
    pub fn text(self, child: &str) -> Self {
        let text_node = document().create_text_node(child);
        self.children.borrow_mut().append_new_group(&text_node);
        self
    }

    pub fn text_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = impl Into<String>>,
    ) -> Self {
        let text_node = document().create_text_node(intern(""));
        self.children.borrow_mut().append_new_group(&text_node);

        let updater = child_signal.for_each({
            clone!(text_node);

            move |new_value| {
                queue_update({
                    clone!(text_node);
                    let new_value = new_value.into();
                    move || text_node.set_data(&new_value)
                });
                async {}
            }
        });

        self.store_future(updater);
        self
    }

    fn store_future(&mut self, future: impl 'static + Future<Output = ()>) {
        self.element.futures.push(spawn_cancelable_future(future));
    }

    // TODO: Test
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
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use web_sys::HtmlInputElement;
    /// # let element = tag("input");
    /// let is_hidden = Mutable::new(false);
    ///
    /// element.effect_signal(is_hidden.signal(), move |elem: &HtmlInputElement, is_hidden| elem.set_hidden(is_hidden));
    /// ```
    pub fn effect<DomType: 'static + JsCast>(self, f: impl 'static + FnOnce(&DomType)) -> Self {
        let dom_element = self.dom_element().clone().dyn_into().unwrap();
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
        let dom_element: DomType = self.dom_element().clone().dyn_into().unwrap();

        let future = sig.for_each(move |x| {
            clone!(dom_element, f);
            after_render(move || f(&dom_element, x));
            async {}
        });

        self.store_future(future);

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
}

impl Builder for ElementBuilder {
    type Target = Element;

    fn build(mut self) -> Self::Target {
        self.element
            .futures
            .extend(self.attribute_futures.into_values());
        self.element
    }

    fn into_element(self) -> Element {
        self.build()
    }
}

impl DomElement for ElementBuilder {
    type Target = dom::Element;

    fn dom_element(&self) -> &Self::Target {
        &self.element.dom_element
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

fn spawn_cancelable_future(
    future: impl 'static + Future<Output = ()>,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    spawn_local(cancelable_future);

    handle
}

/// An HTML element.
///
/// Elements can only appear once in the document. If an element is added again,
/// it will be moved.
pub struct Element {
    dom_element: dom::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<SignalHandle>,
}

impl DomElement for Element {
    type Target = dom::Element;

    fn dom_element(&self) -> &Self::Target {
        &self.dom_element
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

struct ChildVec {
    parent: dom::Element,
    first_children_of_groups: Rc<RefCell<Children>>,
    group_index: usize,
    children: Vec<Element>,
}

impl ChildVec {
    pub fn new(
        parent: dom::Element,
        first_children_of_groups: Rc<RefCell<Children>>,
        group_index: usize,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            first_children_of_groups,
            group_index,
            children: Vec::new(),
        }))
    }

    pub fn apply_update(&mut self, update: VecDiff<impl Into<Element>>) {
        match update {
            VecDiff::Replace { values } => self.replace(values),
            VecDiff::InsertAt { index, value } => self.insert(index, value),
            VecDiff::UpdateAt { index, value } => self.set_at(index, value),
            VecDiff::RemoveAt { index } => {
                self.remove(index);
            }
            VecDiff::Move {
                old_index,
                new_index,
            } => self.relocate(old_index, new_index),
            VecDiff::Push { value } => self.push(value),
            VecDiff::Pop {} => self.pop(),
            VecDiff::Clear {} => self.clear(),
        }
    }

    pub fn replace(&mut self, new_children: Vec<impl Into<Element>>) {
        self.clear();
        self.children = new_children
            .into_iter()
            .map(Into::<Element>::into)
            .collect();

        {
            let mut first_children_of_groups = self.first_children_of_groups.borrow_mut();

            match self.children.first() {
                None => first_children_of_groups.clear_child(self.group_index),
                Some(first_child) => {
                    first_children_of_groups.set_child(self.group_index, first_child.dom_element())
                }
            }
        }

        let children = self.child_dom_elements();
        let parent = self.parent.clone();

        queue_update(move || {
            for child in children {
                parent.append_child(&child).unwrap();
            }
        });
    }

    pub fn insert(&mut self, index: usize, new_child: impl Into<Element>) {
        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        let new_child = new_child.into();

        if index == 0 {
            self.first_children_of_groups
                .borrow_mut()
                .set_child(self.group_index, new_child.dom_element());
        }

        let new_dom_elem = new_child.dom_element();

        assert!(index < self.children.len());
        insert_child_before(
            &self.parent,
            new_dom_elem,
            self.children[index].dom_element(),
        );
        self.children.insert(index, new_child);
    }

    pub fn set_at(&mut self, index: usize, new_child: impl Into<Element>) {
        let new_child = new_child.into();

        if index == 0 {
            self.first_children_of_groups
                .borrow_mut()
                .set_child(self.group_index, new_child.dom_element());
        }

        let old_child = &mut self.children[index];

        replace_child(
            &self.parent,
            new_child.dom_element(),
            old_child.dom_element(),
        );

        *old_child = new_child;
    }

    pub fn remove(&mut self, index: usize) -> Element {
        let old_child = self.children.remove(index);
        remove_child(&self.parent, old_child.dom_element());

        let mut first_children_of_groups = self.first_children_of_groups.borrow_mut();

        match self.children.first() {
            None => first_children_of_groups.clear_child(self.group_index),
            Some(first) => {
                if index == 0 {
                    first_children_of_groups.set_child(self.group_index, first.dom_element())
                }
            }
        }

        old_child
    }

    pub fn relocate(&mut self, old_index: usize, new_index: usize) {
        let child = self.remove(old_index);
        self.insert(new_index, child);
    }

    pub fn push(&mut self, new_child: impl Into<Element>) {
        let new_child = new_child.into();

        if self.children.is_empty() {
            self.first_children_of_groups
                .borrow_mut()
                .set_child(self.group_index, new_child.dom_element());
        }

        append_child(&self.parent, new_child.dom_element());
        self.children.push(new_child);
    }

    pub fn pop(&mut self) {
        let removed_child = self.children.pop();

        if self.children.is_empty() {
            self.first_children_of_groups
                .borrow_mut()
                .clear_child(self.group_index);
        }

        if let Some(removed_child) = removed_child {
            remove_child(&self.parent, removed_child.dom_element());
        }
    }

    pub fn clear(&mut self) {
        let existing_children = self.child_dom_elements();
        self.children.clear();
        let parent = self.parent.clone();

        queue_update(move || {
            for child in existing_children {
                parent.remove_child(&child).unwrap();
            }
        });

        self.first_children_of_groups
            .borrow_mut()
            .clear_child(self.group_index);
    }

    fn child_dom_elements(&self) -> Vec<dom::Element> {
        self.children
            .iter()
            .map(Element::dom_element)
            .cloned()
            .collect()
    }
}

// Keep track of children
// TODO: Rename this to reflect that it only keeps the first child from each
// group
struct Children {
    parent: dom::Element,
    next_group_index: usize,
    // TODO: We should just use a `Vec<Option<dom::Node>>`.
    // The stack size of `BTreeMap` is the same as `Vec`, but it allocs 192 bytes on the first
    // insert and cannot be shrunk to fit.
    children: BTreeMap<usize, dom::Node>,
}

impl Children {
    fn new(parent: dom::Element) -> Self {
        Self {
            parent,
            next_group_index: 0,
            children: BTreeMap::new(),
        }
    }

    fn new_group(&mut self) -> usize {
        let index = self.next_group_index;
        self.next_group_index += 1;
        index
    }

    fn append_new_group(&mut self, child: &dom::Node) {
        let group_index = self.new_group();
        self.add_child(group_index, child);
    }

    fn add_child(&mut self, index: usize, child: &dom::Node) {
        if let Some(existing) = self.children.insert(index, child.clone()) {
            remove_child(&self.parent, &existing);
        }

        match self.children.range((index + 1)..).next() {
            Some((_index, next_child)) => insert_child_before(&self.parent, child, next_child),
            None => append_child(&self.parent, child),
        }
    }

    fn set_child(&mut self, index: usize, child: &dom::Node) {
        self.children.insert(index, child.clone());
    }

    fn remove_child(&mut self, index: usize) {
        if let Some(existing) = self.children.remove(&index) {
            remove_child(&self.parent, &existing);
        }
    }

    fn clear_child(&mut self, index: usize) {
        self.children.remove(&index);
    }
}

fn insert_child_before(parent: &dom::Node, new_child: &dom::Node, next_child: &dom::Node) {
    clone!(parent, new_child, next_child);

    queue_update(move || {
        parent.insert_before(&new_child, Some(&next_child)).unwrap();
    });
}

fn append_child(parent: &dom::Node, child: &dom::Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.append_child(&child).unwrap();
    });
}

fn replace_child(parent: &dom::Node, new_child: &dom::Node, old_child: &dom::Node) {
    clone!(parent, new_child, old_child);

    queue_update(move || {
        parent.replace_child(&new_child, &old_child).unwrap();
    });
}

fn remove_child(parent: &dom::Node, child: &dom::Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.remove_child(&child).unwrap();
    });
}

pub trait AttributeValue {
    fn text(&self) -> String;
}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl AttributeValue for $typ {
                fn text(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);
define_attribute_values!(String);

/// A non-reactive attribute.
pub trait StaticAttribute {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();
        let value = self.text();

        queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
    }
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();

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
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();

        match self {
            Some(value) => value.set_attribute(name, &dom_element),
            None => queue_update(move || {
                dom_element.remove_attribute(&name).unwrap();
            }),
        }
    }
}

/// A potentially reactive attribute.
pub trait Attribute<T> {
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder);
}

impl<T> Attribute<T> for T
where
    T: StaticAttribute,
{
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder) {
        StaticAttribute::set_attribute(&self, name, &builder.element.dom_element);
    }
}

impl<'a> Attribute<String> for &'a str {
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder) {
        self.to_string().set_attribute(name, builder);
    }
}

impl<Sig, Attr> Attribute<Attr> for SignalType<Sig>
where
    Attr: StaticAttribute,
    Sig: 'static + Signal<Item = Attr>,
{
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder) {
        let name = name.into();
        let dom_element = builder.dom_element().clone();

        let updater = self.0.for_each({
            clone!(name);
            move |new_value| {
                StaticAttribute::set_attribute(&new_value, &name, &dom_element);
                async {}
            }
        });

        builder
            .attribute_futures
            .insert(name, spawn_cancelable_future(updater));
    }
}

pub struct SignalType<T>(T);

/// Create a newtype wrapper around a signal.
pub fn signal<Sig: Signal<Item = T>, T>(sig: Sig) -> SignalType<Sig> {
    SignalType(sig)
}

/// Get a raw Javascript, non-reactive DOM element.
pub trait DomElement {
    type Target: Into<dom::Element> + AsRef<dom::Element> + Clone;

    fn dom_element(&self) -> &Self::Target;
}

/// An HTML element builder.
pub trait Builder {
    type Target;

    fn build(self) -> Self::Target;

    fn into_element(self) -> Element;
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
/// # #[macro_use] extern crate silkenweb_dom;
/// # let (x, y, z) = (0, 0, 0);
/// let x = x.clone();
/// let y = y.clone();
/// let z = z.clone();
/// ```
///
/// This is useful for capturing variables by copy in closures. For example:
///
/// ```
/// # #[macro_use] extern crate silkenweb_dom;
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

#[doc(hidden)]
#[macro_export]
macro_rules! named_product{
    ( $($name:ident),* ; ; $($id:ident = $e:expr),*) => {
        $crate::private::map_ref!(
            $(let $id = $e),* => ($(*$id),*)
        )
    };
    ($name:ident $(, $name_tail:ident)* ; $expression:expr $(, $expression_tail:expr)* ; $($id:ident = $e:expr),*) => {
        $crate::named_product!($($name_tail),*; $($expression_tail),*; $($id = $e, )* $name = $expression )
    };
    ( ; $($expression:expr),* ; $($id:ident = $e:expr),*) => { compile_error!("Exceeded maximum of 10 arguments") }
}

#[macro_export]
macro_rules! product{
    ($($e:expr),* $(,)?) => {
        $crate::named_product!(a, b, c, d, e, f, g, h, i, j; $($e),*; )
    };
}
