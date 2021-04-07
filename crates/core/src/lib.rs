pub mod memo;

use std::{
    cell::{Cell, RefCell},
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    mem,
    rc::Rc,
};

use memo::GetMemoKey;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

pub fn mount(id: &str, elem: impl Into<Element>) {
    let elem = elem.into();

    // TODO: Remember this!
    mem::forget(elem.event_callbacks);
    mem::forget(elem.states);
    let dom_element = elem.dom_element;

    DOCUMENT.with(|doc| {
        doc.get_element_by_id(id)
            .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
            .append_child(&dom_element)
            .unwrap()
    });
}

pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

pub struct ElementBuilder(Element);

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder(Element {
            dom_element: DOCUMENT.with(|doc| doc.create_element(tag.as_ref()).unwrap()),
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
        DOCUMENT.with(|doc| self.0.append_child(&doc.create_text_node(child.as_ref())));
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
    states: Vec<Rc<dyn ChildState>>,
    event_callbacks: Vec<Rc<EventCallback>>,
}

impl Element {
    fn append_child(&self, node: &dom::Node) {
        self.dom_element.append_child(node).unwrap();
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
    F: for<'a> Fn(&'a T) -> Element,
{
    element: RefCell<Element>,
    updates_enabled: Cell<bool>,
    generate: F,
    phantom: PhantomData<T>,
}

impl<T, F> UpdateableElement<T, F>
where
    F: for<'a> Fn(&'a T) -> Element,
{
    fn enable_child_updates(&self, enabled: bool) {
        for child in &self.element.borrow().states {
            child.enable_updates(enabled);
        }
    }
}

impl<T, F> ChildState for UpdateableElement<T, F>
where
    F: for<'a> Fn(&'a T) -> Element,
{
    fn enable_updates(&self, enabled: bool) {
        self.updates_enabled.set(enabled);
        self.enable_child_updates(enabled);
    }
}

impl<T, F> StateUpdater<T> for UpdateableElement<T, F>
where
    F: for<'a> Fn(&'a T) -> Element,
{
    fn enable_child_updates(&self, enabled: bool) {
        self.enable_child_updates(enabled);
    }

    fn apply(&self, new_value: &T) {
        if !self.updates_enabled.get() {
            return;
        }

        let element = (self.generate)(new_value);
        self.element
            .borrow()
            .dom_element
            .replace_with_with_node_1(&element.dom_element)
            .unwrap();
        self.element.replace(element);
    }
}

// TODO: Split into getter and setter. Setter has memo key of (). Getter has
// memo key of current value, and still has `with` method.
pub struct State<T>(Setter<T>);

impl<T: 'static> State<T> {
    pub fn new(init: impl 'static + Fn() -> T) -> Self {
        Self(Setter::new(init))
    }

    pub fn with<ElemBuilder, Elem, Gen>(&self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + for<'a> Fn(&'a T) -> ElemBuilder,
    {
        self.0.with(generate)
    }

    pub fn setter(&self) -> Setter<T> {
        self.0.clone()
    }
}

impl<T> GetMemoKey<T> for State<T>
where
    T: Hash + Eq + Clone + Display,
{
    fn memo_key(&self) -> T {
        self.0.current.as_ref().borrow().clone()
    }
}

impl<T> GetMemoKey<T> for &State<T>
where
    T: Hash + Eq + Clone + Display,
{
    fn memo_key(&self) -> T {
        self.0.current.as_ref().borrow().clone()
    }
}

type Mutator<T> = Box<dyn FnOnce(&mut T)>;

pub struct Setter<T> {
    initial: Rc<dyn Fn() -> T>,
    current: Rc<RefCell<T>>,
    modified: Rc<Cell<bool>>,
    new_state: Rc<Cell<Option<Mutator<T>>>>,
    updaters: Rc<RefCell<Vec<Rc<dyn StateUpdater<T>>>>>,
}

impl<T> Clone for Setter<T> {
    fn clone(&self) -> Self {
        Self {
            initial: self.initial.clone(),
            current: self.current.clone(),
            modified: self.modified.clone(),
            new_state: self.new_state.clone(),
            updaters: self.updaters.clone(),
        }
    }
}

impl<T: 'static> Setter<T> {
    fn new(init: impl 'static + Fn() -> T) -> Self {
        let current = Rc::new(RefCell::new(init()));

        Self {
            initial: Rc::new(init),
            current,
            modified: Rc::new(Cell::new(false)),
            new_state: Rc::new(Cell::new(None)),
            updaters: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn update(&self) {
        let new_value = self.current.borrow();

        for updater in self.updaters.borrow_mut().iter_mut() {
            updater.apply(&new_value);
        }
    }

    // TODO:
    fn _reset_to_initial(&self) {
        if self.modified.get() {
            let initial = (self.initial)();
            self.current.replace(initial);
            self.update();
            self.modified.set(false);
        }
    }

    fn with<ElemBuilder, Elem, Gen>(&self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        // TODO: Get rid of Into<Element>. Use another trait that takes a privately constructable
        // empty type on to/from methods.
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + for<'a> Fn(&'a T) -> ElemBuilder,
    {
        let element = generate(&self.current.borrow()).build().into();
        let dom_element = element.dom_element.clone();
        let root_state = Rc::new(UpdateableElement {
            // TODO: Somethings not right here.  We own the element and then create another Element
            // for the same dom node below.
            element: RefCell::new(element),
            generate: move |value| generate(value).build().into(),
            updates_enabled: Cell::new(true),
            phantom: PhantomData,
        });

        self.updaters.borrow_mut().push(root_state.clone());

        Element {
            dom_element,
            states: vec![root_state],
            event_callbacks: Vec::new(),
        }
        .into()
    }

    pub fn set(&self, new_value: T) {
        self.map(|_| new_value);
    }

    pub fn map(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.edit(|x| *x = f(x));
    }

    pub fn edit(&self, f: impl 'static + FnOnce(&mut T)) {
        // TODO: This should chain updates.
        if self.new_state.replace(Some(Box::new(f))).is_none() {
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
}

impl<T: 'static> AnyStateUpdater for Setter<T> {
    /// # Panics
    ///
    /// If there is no new state with which to update.
    fn apply(&self) {
        let f = self.new_state.take().unwrap();
        f(&mut self.current.borrow_mut());
        self.modified.set(true);
        self.update();
    }

    fn enable_child_updates(&self, enabled: bool) {
        for updater in self.updaters.borrow_mut().iter_mut() {
            updater.enable_child_updates(enabled);
        }
    }
}

trait AnyStateUpdater {
    fn apply(&self);

    fn enable_child_updates(&self, enabled: bool);
}

trait StateUpdater<T> {
    fn apply(&self, new_value: &T);

    fn enable_child_updates(&self, enabled: bool);
}

trait ChildState {
    fn enable_updates(&self, enabled: bool);
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

fn request_process_updates() {
    PROCESS_UPDATES.with(|process_updates| {
        WINDOW.with(|window| {
            // TODO: We need to call cancel_animation_frame if we want to exit
            window
                .request_animation_frame(process_updates.as_ref().unchecked_ref())
                .unwrap()
        })
    });
}

fn process_updates() {
    memo::next_frame();

    UPDATE_QUEUE.with(|update_queue| {
        let update_queue = update_queue.take();

        // TODO: If updates len == 1, no need to disable child updates.
        for update in &update_queue {
            update.enable_child_updates(false);
        }

        for update in &update_queue {
            update.apply();
        }

        // TODO: Disable child updates by frame number, then we don't need to re enable
        // them. This doesn't work properly anyway, as child structure may have changed.
        for update in update_queue {
            update.enable_child_updates(true);
        }
    });
}

thread_local!(
    static WINDOW: dom::Window = dom::window().expect("Window must be available");
    static DOCUMENT: dom::Document =
        WINDOW.with(|window| window.document().expect("Window must contain a document"));
    static UPDATE_QUEUE: RefCell<Vec<Box<dyn AnyStateUpdater>>> = RefCell::new(Vec::new());
    static PROCESS_UPDATES: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_time_stamp: JsValue| {
            process_updates();
        }));
);
