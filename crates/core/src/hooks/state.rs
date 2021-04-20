use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    hash::Hash,
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

        self.add_dependent(DependentCallback::new({
            let set_value = value.write();

            move |new_value| set_value.set(generate(new_value))
        }));

        value.read()
    }

    fn add_dependent(&self, dependent_callback: DependentCallback<T>) {
        // TODO: Failure to borrow shared state indicate a circular dependency. We
        // should report a nicer error. Is the borrow failure always a circular dependency?
        let mut state = self.0.borrow_mut();
        state
            .parents
            .push(Box::new(Parent::new(&dependent_callback, &self)));
        state.dependents.insert(dependent_callback);
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
        let x0 = &self.0;
        let x1 = &self.1;
        let value = Signal::new(generate(&x0.current(), &x1.current()));
        let generate0 = Rc::new(generate);
        let generate1 = generate0.clone();

        x0.add_dependent(DependentCallback::new({
            let set_value = value.write();
            let x1 = x1.clone();

            move |new_value| set_value.set(generate0(new_value, &x1.current()))
        }));

        x1.add_dependent(DependentCallback::new({
            let set_value = value.write();
            let x0 = x0.clone();

            move |new_value| set_value.set(generate1(&x0.current(), new_value))
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
    parents: Vec<Box<dyn AnyParent>>,
    dependents: HashSet<DependentCallback<T>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            parents: Vec::new(),
            dependents: HashSet::new(),
        }
    }

    fn update_dependents(&self) {
        for dep in &self.dependents {
            (dep.0)(&self.current);
        }
    }
}

trait AnyParent {}

struct Parent<T> {
    dependent_callback: rc::Weak<dyn Fn(&T)>,
    parent: rc::Weak<RefCell<State<T>>>,
}

impl<T> Parent<T> {
    fn new(dependent_callback: &DependentCallback<T>, parent: &ReadSignal<T>) -> Self {
        Self {
            dependent_callback: Rc::downgrade(&dependent_callback.0),
            parent: Rc::downgrade(&parent.0),
        }
    }
}

impl<T> AnyParent for Parent<T> {}

impl<T> Drop for Parent<T> {
    fn drop(&mut self) {
        if let (Some(dependent_callback), Some(parent)) =
            (self.dependent_callback.upgrade(), self.parent.upgrade())
        {
            let removed = parent
                .borrow_mut()
                .dependents
                .remove(&DependentCallback(dependent_callback));
            assert!(removed);
        }
    }
}

struct DependentCallback<T>(Rc<dyn Fn(&T)>);

impl<T> DependentCallback<T> {
    fn new(f: impl 'static + Fn(&T)) -> Self {
        Self(Rc::new(f))
    }
}

impl<T> PartialEq for DependentCallback<T> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Investigate this further
        #[allow(clippy::vtable_address_comparisons)]
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for DependentCallback<T> {}

impl<T> Hash for DependentCallback<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}
