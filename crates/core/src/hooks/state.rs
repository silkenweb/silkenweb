use std::{
    cell::{Ref, RefCell},
    rc::{self, Rc},
};

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct Signal<T>(SharedState<T>);

impl<T: 'static> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self(Rc::new(RefCell::new(State::new(initial))))
    }

    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |state| &state.current)
    }

    pub fn setter(&self) -> SetSignal<T> {
        SetSignal(Rc::downgrade(&self.0))
    }

    pub fn map<U, Generate>(&self, generate: Generate) -> Signal<U>
    where
        U: 'static,
        Generate: 'static + Fn(&T) -> U,
    {
        let value = Signal::new(generate(&self.0.borrow().current));

        // TODO: Handle removing the new value from dependents.
        self.0.borrow_mut().dependents.push(Rc::new({
            let existing = self.0.clone();
            let set_value = value.setter();

            move |new_value| {
                let _existing = existing.clone();
                set_value.set(generate(new_value))
            }
        }));

        value
    }
}

pub struct SetSignal<T>(rc::Weak<RefCell<State<T>>>);

impl<T> Clone for SetSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> SetSignal<T> {
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
