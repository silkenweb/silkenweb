use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    hash::Hash,
    rc::{self, Rc},
};

// TODO: Can we provide a way to collapse ReadSignal<ReadSignal<T>> into
// ReadSignal<T>?

type SharedState<T> = Rc<State<T>>;
type WeakSharedState<T> = rc::Weak<State<T>>;

pub struct Signal<T>(SharedState<T>);

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self(Rc::new(State::new(initial)))
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
        self.0.current.borrow()
    }

    pub fn only_changes(&self) -> ReadSignal<T>
    where
        T: Clone + Eq,
    {
        let child = Signal::new(self.current().clone());

        self.add_dependent(
            &child,
            Rc::new({
                let child = child.clone();

                move |new_value| {
                    if *child.read().current() != *new_value {
                        child.write().set(new_value.clone())
                    }
                }
            }),
        );

        child.read()
    }

    pub fn send_to<Output>(&self, receiver: impl SignalReceiver<T, Output>) -> ReadSignal<Output>
    where
        Output: 'static,
    {
        let child = Signal::new(receiver.receive(&self.current()));

        self.add_dependent(
            &child,
            Rc::new({
                let set_value = child.write();

                move |new_value| set_value.set(receiver.receive(new_value))
            }),
        );

        child.read()
    }

    pub fn map<Output, Generate>(&self, generate: Generate) -> ReadSignal<Output>
    where
        Output: 'static,
        Generate: 'static + Fn(&T) -> Output,
    {
        self.send_to(generate)
    }

    fn add_dependent<U>(&self, child: &Signal<U>, dependent_callback: Rc<dyn Fn(&T)>) {
        // TODO: Failure to borrow shared state indicate a circular dependency. We
        // should report a nicer error. Is the borrow failure always a circular
        // dependency?
        self.0
            .dependents
            .borrow_mut()
            .insert(DependentCallback::new(&dependent_callback));
        child
            .0
            .parents
            .borrow_mut()
            .push(Box::new(Parent::new(dependent_callback, &self)));
    }
}

pub trait SignalReceiver<Input, Output>: 'static
where
    Input: 'static,
    Output: 'static,
{
    fn receive(&self, x: &Input) -> Output;
}

impl<Input, Output, F> SignalReceiver<Input, Output> for F
where
    Input: 'static,
    Output: 'static,
    F: 'static + Fn(&Input) -> Output,
{
    fn receive(&self, x: &Input) -> Output {
        self(x)
    }
}

pub trait ZipSignal<Generate> {
    type Target;

    fn map(&self, generate: Generate) -> ReadSignal<Self::Target>;
}

// TODO: We need to do this for larger tuples. Is it easier if we have
// flattening of `ReadSignal<ReadSignal<T>>`?
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
        let child = Signal::new(generate(&x0.current(), &x1.current()));
        let generate0 = Rc::new(generate);
        let generate1 = generate0.clone();

        x0.add_dependent(
            &child,
            Rc::new({
                let set_value = child.write();
                let x1 = x1.clone();

                move |new_value| set_value.set(generate0(new_value, &x1.current()))
            }),
        );

        x1.add_dependent(
            &child,
            Rc::new({
                let set_value = child.write();
                let x0 = x0.clone();

                move |new_value| set_value.set(generate1(&x0.current(), new_value))
            }),
        );

        child.read()
    }
}

pub struct WriteSignal<T>(WeakSharedState<T>);

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> WriteSignal<T> {
    pub fn set(&self, new_value: T) {
        if let Some(state) = self.0.upgrade() {
            *state.current.borrow_mut() = new_value;
            state.update_dependents();
        }
    }

    pub fn replace(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.mutate(|x| *x = f(x));
    }

    pub fn mutate(&self, f: impl 'static + FnOnce(&mut T)) {
        if let Some(state) = self.0.upgrade() {
            f(&mut state.current.borrow_mut());
            state.update_dependents();
        }
    }
}

struct State<T> {
    current: RefCell<T>,
    parents: RefCell<Vec<Box<dyn AnyParent>>>,
    dependents: RefCell<HashSet<DependentCallback<T>>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: RefCell::new(init),
            parents: RefCell::new(Vec::new()),
            dependents: RefCell::new(HashSet::new()),
        }
    }

    fn update_dependents(&self) {
        // Take a copy here, as updating dependencies could add or remove dependencies
        // here, causing a multiple borrows.
        let dependents = self.dependents.borrow().clone();

        // If a dependency is added while updating, it won't need updating because it
        // will be initialized with the current value.
        //
        // If a dependency is removed before we get to it in the loop, we'll have a null
        // weak reference and ignore it.
        //
        // If a dependency is removed after we get to it in the loop, we'll still update
        // it.
        for dep in &dependents {
            if let Some(f) = dep.0.upgrade() {
                f(&self.current.borrow());
            }
        }
    }
}

trait AnyParent {}

struct Parent<T> {
    dependent_callback: Rc<dyn Fn(&T)>,
    parent: Rc<State<T>>,
}

impl<T> Parent<T> {
    fn new(dependent_callback: Rc<dyn Fn(&T)>, parent: &ReadSignal<T>) -> Self {
        Self {
            dependent_callback,
            parent: parent.0.clone(),
        }
    }
}

impl<T> AnyParent for Parent<T> {}

impl<T> Drop for Parent<T> {
    fn drop(&mut self) {
        let removed = self
            .parent
            .dependents
            .borrow_mut()
            .remove(&DependentCallback(Rc::downgrade(&self.dependent_callback)));
        assert!(removed);
    }
}

struct DependentCallback<T>(rc::Weak<dyn Fn(&T)>);

impl<T> Clone for DependentCallback<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> DependentCallback<T> {
    fn new(f: &Rc<dyn 'static + Fn(&T)>) -> Self {
        Self(Rc::downgrade(f))
    }

    fn upgrade(&self) -> Rc<dyn 'static + Fn(&T)> {
        self.0.upgrade().unwrap()
    }
}

impl<T> PartialEq for DependentCallback<T> {
    fn eq(&self, other: &Self) -> bool {
        // We need to discard the vtable by casting as we only care if the data pointers
        // are equal. See https://github.com/rust-lang/rust/issues/46139
        Rc::as_ptr(&self.upgrade()).cast::<()>() == Rc::as_ptr(&other.upgrade()).cast::<()>()
    }
}

impl<T> Eq for DependentCallback<T> {}

impl<T> Hash for DependentCallback<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.upgrade()).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, mem};

    use super::*;

    #[test]
    fn callback_cleanup() {
        let state = Rc::new(Cell::new(0));
        let x = Signal::new(0);
        let y = x.read().map({
            let state = state.clone();
            move |x| state.replace(*x)
        });

        x.write().set(1);
        mem::drop(y);
        x.write().set(2);
        assert_eq!(state.get(), 1);
    }
}
