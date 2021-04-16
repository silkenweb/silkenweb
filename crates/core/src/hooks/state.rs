use std::{
    cell::{Ref, RefCell},
    rc::{self, Rc},
};

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

pub struct SetState<T>(rc::Weak<RefCell<State<T>>>);

impl<T> Clone for SetState<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub fn use_state<T: 'static>(init: T) -> (GetState<T>, SetState<T>) {
    let state = Rc::new(RefCell::new(State::new(init)));

    (GetState(state.clone()), SetState(Rc::downgrade(&state)))
}

impl<T: 'static> SetState<T> {
    pub fn set(&self, new_value: T) {
        if let Some(state) = self.0.upgrade() {
            state.borrow_mut().current = new_value;
            state.borrow().update_dependents();
        }
    }

    pub fn map(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.mutate(|x| *x = f(x));
    }

    pub fn mutate(&self, f: impl 'static + FnOnce(&mut T)) {
        if let Some(state) = self.0.upgrade() {
            f(&mut state.borrow_mut().current);
            state.borrow().update_dependents();
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

    fn update_dependents(&self) {
        for dep in &self.dependents {
            dep(&self.current);
        }
    }
}
