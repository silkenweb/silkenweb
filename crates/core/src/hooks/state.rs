use std::{
    cell::{Ref, RefCell},
    rc::{self, Rc},
};

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct Signal<T>(SharedState<T>);

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self(Rc::new(RefCell::new(State::new(initial))))
    }

    pub fn read(&self) -> ReadSignal<T> {
        ReadSignal(self.0.clone())
    }

    pub fn write(&self) -> WriteSignal<T> {
        WriteSignal(Rc::downgrade(&self.0))
    }
}

pub struct ReadSignal<T>(SharedState<T>);

impl<T> Clone for ReadSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> ReadSignal<T> {
    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |state| &state.current)
    }

    pub fn map<U, Generate>(&self, generate: Generate) -> ReadSignal<U>
    where
        U: 'static,
        Generate: 'static + Fn(&T) -> U,
    {
        let value = Signal::new(generate(&self.current()));

        // TODO: Handle removing the new value from dependents.
        self.0.borrow_mut().dependents.push(Rc::new({
            let existing = self.0.clone();
            let set_value = value.write();

            move |new_value| {
                let _existing = existing.clone();
                set_value.set(generate(new_value))
            }
        }));

        value.read()
    }
}

pub trait ZipSignal<Generate> {
    type Target;

    fn map(&self, generate: Generate) -> ReadSignal<Self::Target>;
}

impl<T0, T1, U, Generate> ZipSignal<Generate> for (ReadSignal<T0>, ReadSignal<T1>)
where
    T0: 'static,
    T1: 'static,
    U: 'static,
    Generate: 'static + Fn(&T0, &T1) -> U,
{
    type Target = U;

    fn map(&self, generate: Generate) -> ReadSignal<Self::Target> {
        let v0 = self.0.clone();
        let v1 = self.1.clone();
        let value = Signal::new(generate(&v0.current(), &v1.current()));
        let generate0 = Rc::new(generate);
        let generate1 = generate0.clone();

        // TODO: Handle removing the new value from dependents.
        v0.0.borrow_mut().dependents.push(Rc::new({
            let v0 = v0.0.clone();
            let set_value = value.write();
            let v1 = v1.clone();

            move |new_value| {
                let _existing = v0.clone();
                set_value.set(generate0(new_value, &v1.current()))
            }
        }));

        v1.0.borrow_mut().dependents.push(Rc::new({
            let v1 = v1.0.clone();
            let set_value = value.write();

            move |new_value| {
                let _existing = v1.clone();
                set_value.set(generate1(&v0.current(), new_value))
            }
        }));

        value.read()
    }
}

pub struct WriteSignal<T>(rc::Weak<RefCell<State<T>>>);

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> WriteSignal<T> {
    pub fn set(&self, new_value: T) {
        if let Some(state) = self.0.upgrade() {
            state.borrow_mut().current = new_value;
            state.borrow().update_dependents();
        }
    }

    pub fn replace(&self, f: impl 'static + FnOnce(&T) -> T) {
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
        // TODO: Experiment with finer grained borrowing.
        //
        // `current` and `dependents` should be RefCells, instead of the whole of
        // `State`.
        // Seperate exisiting dependents from new dependents, so we can borrow
        // dependents and add to new dependents from within a dep. Then we need to take
        // and process new dependencies, until we're done. Would this recurse forever?
        for dep in &self.dependents {
            dep(&self.current);
        }
    }
}
