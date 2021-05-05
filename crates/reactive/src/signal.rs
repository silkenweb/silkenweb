//! Signals are like variables that update their dependencies.
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
    hash::Hash,
    rc::{self, Rc},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::clone;

type SharedState<T> = Rc<State<T>>;
type WeakSharedState<T> = rc::Weak<State<T>>;

/// A [`Signal`] is like a varible, but it can update it's dependencies when it
/// changes.
///
/// ```
/// # use silkenweb_reactive::signal::*;
/// let x = Signal::new(0);
/// let next_x = x.read().map(|x| x + 1);
/// assert_eq!(*next_x.current(), 1);
/// x.write().set(2);
/// assert_eq!(*next_x.current(), 3);
/// ```
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

#[cfg(feature = "serde")]
impl<T: 'static + Serialize> Serialize for Signal<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.current.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: 'static + Deserialize<'de>> Deserialize<'de> for Signal<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Signal::new(T::deserialize(deserializer)?))
    }
}

/// Receive changes from a signal.
///
/// Changes will stop being received when this is destroyed:
///
/// ```
/// # use silkenweb_reactive::{clone, signal::*};
/// # use std::{mem, cell::Cell, rc::Rc};
/// let x = Signal::new(1);
/// let seen_by_y = Rc::new(Cell::new(0));
/// let y = x.read().map({
///     clone!(seen_by_y);
///     move|&x| seen_by_y.set(x)
/// });
/// assert_eq!(seen_by_y.get(), 1);
/// x.write().set(2);
/// assert_eq!(seen_by_y.get(), 2);
/// mem::drop(y);
/// // We won't see this update
/// x.write().set(3);
/// assert_eq!(seen_by_y.get(), 2);
/// ```
#[must_use]
pub struct ReadSignal<T>(SharedState<T>);

impl<T> Clone for ReadSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> ReadSignal<T> {
    /// The current value of the signal
    pub fn current(&self) -> Ref<T> {
        self.0.current()
    }

    /// Only propagate actual changes to the signal value.
    ///
    /// ```
    /// # use silkenweb_reactive::{clone, signal::*};
    /// # use std::{mem, cell::Cell, rc::Rc};
    /// let all_updates_count = Rc::new(Cell::new(0));
    /// let only_changes_count = Rc::new(Cell::new(0));
    /// let x = Signal::new(0);
    /// let all_updates = x.read().map({
    ///     clone!(all_updates_count);
    ///     move |_| all_updates_count.set(all_updates_count.get() + 1)
    /// });
    /// let only_changes = x.read().only_changes().map({
    ///     clone!(only_changes_count);
    ///     move |_| only_changes_count.set(only_changes_count.get() + 1)
    /// });
    ///
    /// x.write().set(1);
    /// x.write().set(1);
    /// assert_eq!(all_updates_count.get(), 3, "One for init + 2 updates");
    /// assert_eq!(only_changes_count.get(), 2, "One for init + 1 actual change");
    /// ```
    pub fn only_changes(&self) -> ReadSignal<T>
    where
        T: Clone + Eq,
    {
        let child = Signal::new(self.current().clone());

        self.add_dependent(
            &child,
            Rc::new({
                clone!(child);

                move |new_value| {
                    if *child.read().current() != *new_value {
                        child.write().set(new_value.clone())
                    }
                }
            }),
        );

        child.read()
    }

    /// Map a function onto the inner value to produce a new [`ReadSignal`].
    ///
    /// This only exists to make type inference easier, and just forwards its
    /// arguments to [`map_to`](Self::map_to).
    pub fn map<Output, Generate>(&self, generate: Generate) -> ReadSignal<Output>
    where
        Output: 'static,
        Generate: 'static + Fn(&T) -> Output,
    {
        self.map_to(generate)
    }

    /// Receive changes to a signal.
    pub fn map_to<Output>(&self, receiver: impl SignalReceiver<T, Output>) -> ReadSignal<Output>
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

    fn add_dependent<U>(&self, child: &Signal<U>, dependent_callback: Rc<dyn Fn(&T)>) {
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

/// Receive changes to a signal.
///
/// Pass a `SignalReceiver` to [`ReadSignal::map_to`], as an alternative to
/// passing a closure to [`ReadSignal::map`].
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

/// Write changes to a signal.
pub struct WriteSignal<T>(WeakSharedState<T>);

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> WriteSignal<T> {
    /// Set the inner value of a signal, and update downstream signals.
    pub fn set(&self, new_value: T) {
        if let Some(state) = self.0.upgrade() {
            *state.current_mut() = new_value;
            state.update_dependents();
        }
    }

    /// Replace inner value of a signal using `f`, and update downstream
    /// signals.
    pub fn replace(&self, f: impl 'static + FnOnce(&T) -> T) {
        self.mutate(|x| *x = f(x));
    }

    /// Mutate the inner value of a signal using `f`, and update downstream
    /// signals.
    pub fn mutate(&self, f: impl 'static + FnOnce(&mut T)) {
        if let Some(state) = self.0.upgrade() {
            f(&mut state.current_mut());
            state.update_dependents();
        }
    }
}

/// Zip signals together to create a new one. The tuple implementation allows
/// you to write `(signal0, signal1).zip().map(...)`.
pub trait ZipSignal {
    /// The inner type of the target signal.
    type Target;

    /// Zip the elements of `self` together.
    fn zip(self) -> ReadSignal<Self::Target>;
}

impl<Lhs, Rhs> ZipSignal for (ReadSignal<Lhs>, ReadSignal<Rhs>)
where
    Lhs: 'static + Clone,
    Rhs: 'static + Clone,
{
    type Target = (Lhs, Rhs);

    fn zip(self) -> ReadSignal<Self::Target> {
        let (lhs, rhs) = self;

        let current = (lhs.current().clone(), rhs.current().clone());
        let product = Signal::new(current);

        lhs.add_dependent(
            &product,
            Rc::new({
                let set_value = product.write();
                clone!(rhs);

                move |new_value| set_value.set((new_value.clone(), rhs.current().clone()))
            }),
        );

        rhs.add_dependent(
            &product,
            Rc::new({
                let set_value = product.write();
                clone!(lhs);

                move |new_value| set_value.set((lhs.current().clone(), new_value.clone()))
            }),
        );

        product.read()
    }
}

macro_rules! zip_signal{
    ( $($x:ident : $typ:ident),* ) => {
        impl<T0, $($typ),*> ZipSignal for (ReadSignal<T0>, $(ReadSignal<$typ>),*)
        where
            T0: 'static + Clone,
            $($typ: 'static + Clone),*
        {
            type Target = (T0, $($typ),*);

            fn zip(self) -> ReadSignal<Self::Target> {
                let (x0, $($x),*) = self;

                (x0, ( $($x),* ).zip())
                    .zip()
                    .map(|(x0, ( $($x),*) )| (x0.clone(), $($x.clone()),*))
            }
        }
    }
}

macro_rules! zip_all_signals{
    () => { zip_signal!(x1: T1, x2: T2); };
    ($head:ident : $head_typ: ident $(, $tail:ident : $tail_typ:ident)*) => {
        zip_all_signals!( $($tail: $tail_typ),* );
        zip_signal!(x1: T1, x2: T2, $head: $head_typ $(, $tail: $tail_typ)*);
    }
}

zip_all_signals!(
    x3: T3,
    x4: T4,
    x5: T5,
    x6: T6,
    x7: T7,
    x8: T8,
    x9: T9,
    x10: T10
);

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
                f(&self.current());
            }
        }
    }

    fn current(&self) -> Ref<T> {
        self.current
            .try_borrow()
            .expect("Possible circular dependency")
    }

    fn current_mut(&self) -> RefMut<T> {
        self.current
            .try_borrow_mut()
            .expect("Possible circular dependency")
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
