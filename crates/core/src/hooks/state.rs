use std::{
    cell::{Ref, RefCell},
    rc::Rc,
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
        WriteSignal(self.0.clone())
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
            let set_value = value.write();

            move |new_value| set_value.set(generate(new_value))
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
            let set_value = value.write();
            let v1 = v1.clone();

            move |new_value| set_value.set(generate0(new_value, &v1.current()))
        }));

        v1.0.borrow_mut().dependents.push(Rc::new({
            let set_value = value.write();

            move |new_value| set_value.set(generate1(&v0.current(), new_value))
        }));

        value.read()
    }
}

pub struct WriteSignal<T>(SharedState<T>);

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> WriteSignal<T> {
    pub fn set(&self, new_value: T) {
        self.0.borrow_mut().current = new_value;
        self.0.borrow().update_dependents();
    }

    pub fn replace(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.mutate(|x| *x = f(x));
    }

    pub fn mutate(&self, f: impl 'static + FnOnce(&mut T)) {
        f(&mut self.0.borrow_mut().current);
        self.0.borrow().update_dependents();
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
