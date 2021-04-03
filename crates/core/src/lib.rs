use std::{
    cell::{Cell, RefCell},
    mem,
    rc::Rc,
};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

pub fn append_to_body(elem: impl Into<Element>) {
    web_log::println!("Setting document body");
    let elem = elem.into();

    // TODO: Remember this!
    mem::forget(elem.event_callbacks);
    mem::forget(elem.states);
    let dom_element = elem.dom_element;

    DOCUMENT.with(|doc| {
        doc.body()
            .expect("Document must contain a `body`")
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
        self.0
            .event_callbacks
            .push(EventCallback::new(self.0.dom_element.clone(), name, f));
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

pub struct Element {
    dom_element: dom::Element,
    states: Vec<Rc<dyn ChildState>>,
    event_callbacks: Vec<EventCallback>,
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

struct State<F> {
    dom_element: RefCell<dom::Element>,
    cancelled: Cell<bool>,
    generate: F,
    child_states: Cell<Vec<Rc<dyn ChildState>>>,
    event_callbacks: Cell<Vec<EventCallback>>,
}

impl<F> State<F> {
    fn cancel_children(&self) {
        for child in self.child_states.take() {
            child.cancel();
        }
    }
}

impl<F> ChildState for State<F> {
    fn cancel(&self) {
        self.cancelled.replace(true);
        self.cancel_children();
        self.event_callbacks.take();
    }
}

impl<T, F> StateUpdater<T> for State<F>
where
    F: Fn(T) -> Element,
{
    fn cancel_children(&self) {
        self.cancel_children();
    }

    fn apply(&self, new_value: T) {
        if self.cancelled.get() {
            return;
        }

        let element = (self.generate)(new_value);
        self.dom_element
            .borrow()
            .replace_with_with_node_1(&element.dom_element)
            .unwrap();
        self.dom_element.replace(element.dom_element);
        self.child_states.replace(element.states);
        self.event_callbacks.replace(element.event_callbacks);
    }
}

pub struct StateSetter<T> {
    current: Rc<RefCell<T>>,
    new_state: Rc<Cell<Option<T>>>,
    updaters: Rc<RefCell<Vec<Rc<dyn StateUpdater<T>>>>>,
}

impl<T> Clone for StateSetter<T> {
    fn clone(&self) -> Self {
        Self {
            current: self.current.clone(),
            new_state: self.new_state.clone(),
            updaters: self.updaters.clone(),
        }
    }
}

impl<T> StateSetter<T> {
    pub fn new(init: T) -> Self {
        Self {
            current: Rc::new(RefCell::new(init)),
            new_state: Rc::new(Cell::new(None)),
            updaters: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl<T: 'static + Clone> StateSetter<T> {
    pub fn with<ElemBuilder, Elem>(&self, generate: impl 'static + Fn(T) -> ElemBuilder) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
    {
        let element = generate(self.current()).build().into();
        let dom_element = element.dom_element;
        let root_state = Rc::new(State {
            dom_element: RefCell::new(dom_element.clone()),
            generate: move |value| generate(value).build().into(),
            child_states: Cell::new(element.states),
            event_callbacks: Cell::new(element.event_callbacks),
            cancelled: Cell::new(false),
        });

        self.updaters.borrow_mut().push(root_state.clone());

        Element {
            dom_element,
            states: vec![root_state],
            event_callbacks: Vec::new(),
        }
        .into()
    }

    pub fn map(&self, f: impl FnOnce(T) -> T) {
        self.set(f(self.current()))
    }

    pub fn set(&self, new_value: T) {
        if self.new_state.replace(Some(new_value)).is_none() {
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

    fn current(&self) -> T {
        self.current.as_ref().clone().into_inner()
    }
}

impl<T: Clone> AnyStateUpdater for StateSetter<T> {
    /// # Panics
    ///
    /// If there is no new state with which to update.
    fn apply(&self) {
        let new_value = self.new_state.take().unwrap();
        self.current.replace(new_value.clone());

        for updater in self.updaters.borrow_mut().iter_mut() {
            updater.apply(new_value.clone());
        }
    }

    fn cancel_children(&self) {
        for updater in self.updaters.borrow_mut().iter_mut() {
            updater.cancel_children();
        }
    }
}

trait AnyStateUpdater {
    fn apply(&self);

    fn cancel_children(&self);
}

trait StateUpdater<T> {
    fn cancel_children(&self);

    fn apply(&self, new_value: T);
}

trait ChildState {
    fn cancel(&self);
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
            .remove_event_listener_with_callback(self.name, self.callback.as_ref().unchecked_ref())
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
    UPDATE_QUEUE.with(|update_queue| {
        let update_queue = update_queue.take();

        for update in &update_queue {
            update.cancel_children();
        }

        for update in update_queue {
            update.apply();
        }
    })
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
