use std::{
    cell::{Cell, Ref, RefCell},
    rc::{self, Rc},
};

use super::queue_update;
use crate::hooks::Update;

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct GetState<T>(SharedState<T>);

impl<T: 'static> GetState<T> {
    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |state| &state.current)
    }

    pub fn with<U, Generate>(&self, generate: Generate) -> GetState<U>
    where
        U: 'static,
        Generate: 'static + Fn(&T) -> U,
    {
        let (value, set_value) = use_state(generate(&self.0.borrow().current));

        self.0.borrow_mut().dependents.push(Rc::new({
            let value = self.0.clone();
            move |new_value| {
                // TODO: Keep a reference to the source in a Dependent struct
                let _existing = value.clone();
                set_value.set(generate(new_value))
            }
        }));

        // TODO: Store state updates in SharedState. If there are any pending, queue
        // updates for the new dependent.
        // TODO: New value needs to keep a ref to this.

        value
    }
}

struct UpdateDependent<T> {
    state: SetState<T>,
    dependent: rc::Weak<dyn Fn(&T)>,
}

impl<T> Update for UpdateDependent<T> {
    fn apply(&self) {
        if let Some(f) = self.state.new_state.take() {
            if let Some(state) = self.state.state.upgrade() {
                f(&mut state.borrow_mut().current);
            }
        }

        if let Some(dependent) = self.dependent.upgrade() {
            if let Some(state) = self.state.state.upgrade() {
                dependent(&mut state.borrow_mut().current);
            }
        }
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
            self.queue_updates();
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
            self.queue_updates();
        }
    }

    fn queue_updates(&self) {
        if let Some(state) = self.state.upgrade() {
            for dependent in &state.borrow().dependents {
                queue_update(UpdateDependent {
                    state: self.clone(),
                    dependent: Rc::downgrade(dependent),
                });
            }
        }
    }
}

struct State<T> {
    current: T,
    dependents: Vec<Rc<dyn Fn(&T)>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            dependents: Vec::new(),
        }
    }
}
