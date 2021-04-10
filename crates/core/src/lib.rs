use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    marker::PhantomData,
    rc::{self, Rc},
};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys as dom;

pub fn mount(id: &str, elem: impl Into<Element>) {
    let elem = elem.into();
    let dom_element = elem.dom_element.clone();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));

    DOCUMENT.with(|doc| {
        doc.get_element_by_id(id)
            .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
            .append_child(&dom_element)
            .unwrap()
    });
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
    states: Vec<Rc<RefCell<dyn OwnedChild>>>,
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
    generate: F,
    phantom: PhantomData<T>,
}

impl<T> OwnedChild for State<T> {}

impl<T, F> StateUpdater<T> for UpdateableElement<T, F>
where
    F: for<'a> Fn(&'a T) -> Element,
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
        Gen: 'static + for<'a> Fn(&'a T) -> ElemBuilder,
    {
        let element = generate(&self.0.borrow().current).build().into();
        let dom_element = element.dom_element.clone();
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

struct State<T> {
    current: T,
    updaters: Vec<Rc<dyn StateUpdater<T>>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            updaters: Vec::new(),
        }
    }

    fn update(&mut self) {
        for updater in self.updaters.iter_mut() {
            updater.apply(&self.current);
        }
    }
}

impl<T: 'static> AnyStateUpdater for SetState<T> {
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

trait AnyStateUpdater {
    fn apply(&self);
}

trait StateUpdater<T> {
    fn apply(&self, new_value: &T);
}

trait OwnedChild {}

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
    UPDATE_QUEUE.with(|update_queue| {
        let update_queue = update_queue.take();

        // TODO: If updates len == 1, no need to disable child updates.

        for update in &update_queue {
            update.apply();
        }
    });
}

thread_local!(
    static WINDOW: dom::Window = dom::window().expect("Window must be available");
    static DOCUMENT: dom::Document =
        WINDOW.with(|window| window.document().expect("Window must contain a document"));
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
    static UPDATE_QUEUE: RefCell<Vec<Box<dyn AnyStateUpdater>>> = RefCell::new(Vec::new());
    static PROCESS_UPDATES: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_time_stamp: JsValue| {
            process_updates();
        }));
);
