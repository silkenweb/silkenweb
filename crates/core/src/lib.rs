#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate
)]
use std::{
    any::{Any, TypeId},
    cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
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

struct UpdateableElement<T, F>
where
    F: Fn(&T) -> Element,
{
    element: RefCell<Element>,
    generate: F,
    phantom: PhantomData<T>,
}

impl<T> OwnedChild for State<T> {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}

impl<T, F> StateUpdater<T> for UpdateableElement<T, F>
where
    F: Fn(&T) -> Element,
{
    fn apply(&self, new_value: &T) {
        let element = (self.generate)(new_value);
        self.element
            .borrow()
            .dom_element
            .replace_with_with_node_1(&element.dom_element)
            .unwrap();
        self.element.replace(element);
    }
}

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct GetState<T>(SharedState<T>);

impl<T: 'static> GetState<T> {
    pub fn with<ElemBuilder, Elem, Gen>(&self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        // TODO: Get rid of Into<Element>. Use another trait that takes a privately constructable
        // empty type on to/from methods.
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = generate(&self.0.borrow().current).build().into();
        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.clone());

        let root_state = Rc::new(UpdateableElement {
            // TODO: Somethings not right here.  We own the element and then create another Element
            // for the same dom node below.
            element: RefCell::new(element),
            generate: move |value| generate(value).build().into(),
            phantom: PhantomData,
        });

        self.0.borrow_mut().updaters.push(root_state.clone());

        Element {
            dom_element,
            states: vec![self.0.clone()],
            event_callbacks: Vec::new(),
        }
        .into()
    }
}

type Mutator<T> = Box<dyn FnOnce(&mut T)>;

pub struct SetState<T> {
    state: rc::Weak<RefCell<State<T>>>,
    new_state: Rc<Cell<Option<Mutator<T>>>>,
}

impl<T> Clone for SetState<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            new_state: self.new_state.clone(),
        }
    }
}

pub fn use_state<T: 'static>(init: T) -> (GetState<T>, SetState<T>) {
    let state = Rc::new(RefCell::new(State::new(init)));

    (
        GetState(state.clone()),
        SetState {
            state: Rc::downgrade(&state),
            new_state: Rc::new(Cell::new(None)),
        },
    )
}

impl<T: 'static> SetState<T> {
    pub fn set(&self, new_value: T) {
        if self
            .new_state
            .replace(Some(Box::new(|x| *x = new_value)))
            .is_none()
        {
            self.queue_update();
        }
    }

    pub fn map(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.edit(|x| *x = f(x));
    }

    pub fn edit(&self, f: impl 'static + FnOnce(&mut T)) {
        let existing = self.new_state.replace(None);

        if let Some(existing) = existing {
            self.new_state.replace(Some(Box::new(move |x| {
                existing(x);
                f(x);
            })));
        } else {
            self.new_state.replace(Some(Box::new(f)));
            self.queue_update();
        }
    }

    fn queue_update(&self) {
        UPDATE_QUEUE.with(|update_queue| {
            let len = {
                let mut update_queue = update_queue.borrow_mut();

                update_queue.push(Box::new(self.clone()));
                update_queue.len()
            };

            if len == 1 {
                request_process_updates();
            }
        });
    }
}

pub struct Reference<T>(Rc<RefCell<RefData<T>>>);

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct RefData<T> {
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
    elements: Vec<Element>,
    value: T,
}

impl<T> Reference<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(RefData {
            parent: None,
            elements: Vec::new(),
            value,
        })))
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |x| &x.value)
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.value)
    }

    pub fn replace(&self, new: T) -> T {
        mem::replace(&mut self.0.borrow_mut().value, new)
    }

    pub fn replace_with<F>(&self, f: F) -> T
    where
        F: FnOnce(&mut T) -> T,
    {
        let old = &mut self.0.borrow_mut().value;
        let new = f(old);
        mem::replace(old, new)
    }

    pub fn take(&self) -> T
    where
        T: Default,
    {
        mem::take(&mut self.0.borrow_mut().value)
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.0
            .try_borrow()
            .map(|borrowed| Ref::map(borrowed, |x| &x.value))
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        self.0
            .try_borrow_mut()
            .map(|borrowed| RefMut::map(borrowed, |x| &mut x.value))
    }
}

impl<T: 'static> Scopeable for Reference<T> {
    fn add_child(&mut self, element: Element) {
        self.0.borrow_mut().elements.push(element);
    }

    fn as_child(&self) -> Rc<RefCell<dyn OwnedChild>> {
        self.0.clone()
    }
}

impl<T> OwnedChild for RefData<T> {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}

#[derive(Clone, Default)]
pub struct Memo(Rc<RefCell<MemoData>>);

impl Memo {
    fn memo_map<'a, Key: 'static, Value: 'static>(
        any_map: &'a mut AnyMap,
    ) -> &'a mut HashMap<Key, Value> {
        let type_key = (TypeId::of::<Key>(), TypeId::of::<Value>());
        any_map
            .entry(type_key)
            .or_insert_with(|| Box::new(HashMap::<Key, Value>::new()))
            .downcast_mut()
            .unwrap()
    }

    pub fn cache<Key, Value, ValueFn>(&self, key: Key, value_fn: ValueFn) -> Value
    where
        Key: 'static + Eq + Hash,
        Value: 'static + Clone,
        ValueFn: FnOnce() -> Value,
    {
        let mut memo = self.0.borrow_mut();

        if memo.next_memoized.is_empty() {
            EFFECT_STACK.with(|effect_stack| {
                effect_stack
                    .borrow_mut()
                    .push(Box::new(Rc::downgrade(&self.0)))
            });
        }

        let current_memos = Self::memo_map::<Key, Value>(&mut memo.current_memoized);
        let value = current_memos.remove(&key).unwrap_or_else(value_fn);

        let next_memos = Self::memo_map::<Key, Value>(&mut memo.next_memoized);
        next_memos.insert(key, value.clone());
        value
    }
}

impl Scopeable for Memo {
    fn add_child(&mut self, element: Element) {
        self.0.borrow_mut().elements.push(element);
    }

    fn as_child(&self) -> Rc<RefCell<dyn OwnedChild>> {
        self.0.clone()
    }
}

type AnyMap = HashMap<(TypeId, TypeId), Box<dyn Any>>;

#[derive(Default)]
struct MemoData {
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
    elements: Vec<Element>,
    current_memoized: AnyMap,
    next_memoized: AnyMap,
}

#[derive(Default)]
pub struct Scope<T>(T);

impl<T: Scopeable> Scope<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn with<ElemBuilder, Elem, Gen>(&mut self, mut generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        // TODO: Get rid of Into<Element>. Use another trait that takes a privately constructable
        // empty type on to/from methods.
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: FnMut(T) -> ElemBuilder,
    {
        // TODO: Is there more of this we can factor out between `Reference` and
        // `GetState`? We'd need to pass `generate` into `Scopeable::add_child` and have
        // an associate type + accessor for `generate`s argument.
        let element = generate(self.0.clone()).build().into();

        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.as_child().clone());
        self.0.add_child(element);

        Element {
            dom_element,
            states: vec![self.0.as_child().clone()],
            event_callbacks: Vec::new(),
        }
        .into()
    }
}

impl OwnedChild for MemoData {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}

impl Effect for rc::Weak<RefCell<MemoData>> {
    fn apply(&self) {
        if let Some(memo) = self.upgrade() {
            let mut memo = memo.borrow_mut();
            memo.current_memoized = mem::take(&mut memo.next_memoized);
        }
    }
}

fn dom_depth(parent: &Option<rc::Weak<RefCell<dyn OwnedChild>>>) -> usize {
    parent
        .as_ref()
        .and_then(rc::Weak::upgrade)
        .map_or(0, |p| p.borrow().dom_depth() + 1)
}

struct State<T> {
    current: T,
    updaters: Vec<Rc<dyn StateUpdater<T>>>,
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            updaters: Vec::new(),
            parent: None,
        }
    }

    fn update(&mut self) {
        for updater in &mut self.updaters {
            updater.apply(&self.current);
        }
    }
}

impl<T: 'static> AnyStateUpdater for SetState<T> {
    fn dom_depth(&self) -> usize {
        self.state.upgrade().map_or(0, |s| s.borrow().dom_depth())
    }

    /// # Panics
    ///
    /// If there is no new state with which to update.
    fn apply(&self) {
        let f = self.new_state.take().unwrap();

        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();
            f(&mut state.current);
            state.update();
        }
    }
}

pub trait Scopeable: Clone {
    fn add_child(&mut self, element: Element);

    fn as_child(&self) -> Rc<RefCell<dyn OwnedChild>>;
}

trait AnyStateUpdater {
    fn dom_depth(&self) -> usize;

    fn apply(&self);
}

trait StateUpdater<T> {
    fn apply(&self, new_value: &T);
}

trait Effect {
    fn apply(&self);
}

pub trait OwnedChild {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>);

    fn dom_depth(&self) -> usize;
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

fn request_process_updates() {
    PROCESS_UPDATES.with(|process_updates| {
        // TODO: We need to call cancel_animation_frame if we want to exit
        window()
            .request_animation_frame(process_updates.as_ref().unchecked_ref())
            .unwrap()
    });
}

fn process_updates() {
    UPDATE_QUEUE.with(|update_queue| {
        let mut update_queue = update_queue.take();

        if update_queue.len() != 1 {
            let mut updates_by_depth: Vec<_> = update_queue
                .into_iter()
                .map(|u| (u.dom_depth(), u))
                .collect();

            updates_by_depth.sort_unstable_by_key(|(key, _)| *key);

            update_queue = updates_by_depth
                .into_iter()
                .map(|(_, value)| value)
                .collect();
        }

        for update in update_queue {
            update.apply();
        }
    });

    EFFECT_STACK.with(|effect_queue| {
        for effect in effect_queue.take() {
            effect.apply();
        }
    });
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
    static UPDATE_QUEUE: RefCell<Vec<Box<dyn AnyStateUpdater>>> = RefCell::new(Vec::new());
    static EFFECT_STACK: RefCell<Vec<Box<dyn Effect>>> = RefCell::new(Vec::new());
    static PROCESS_UPDATES: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_time_stamp: JsValue| {
            process_updates();
        }));
);
