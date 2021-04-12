use std::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::{self, Rc},
};

use crate::{
    dom_depth,
    hooks::{request_process_updates, AnyStateUpdater, StateUpdater, UPDATE_QUEUE},
    Builder,
    Element,
    OwnedChild,
};

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
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = generate(&self.0.borrow().current).build().into();
        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.clone());

        let root_state = Rc::new(UpdateableElement {
            element: RefCell::new(element),
            generate: move |value| generate(value).build().into(),
            phantom: PhantomData,
        });

        self.0.borrow_mut().updaters.push(root_state.clone());

        // This is kind of the parent element, except we don't know about parents yet.
        // When we add it as a child, its members will be added to the new parent.
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
