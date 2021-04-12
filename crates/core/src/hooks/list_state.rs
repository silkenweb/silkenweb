use std::{
    cell::RefCell,
    rc::{self, Rc},
};

use super::queue_update;
use crate::{dom_depth, hooks::AnyStateUpdater, Builder, Element, ElementBuilder, OwnedChild};

impl<T> OwnedChild for ListState<T> {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}

type SharedState<T> = Rc<RefCell<ListState<T>>>;

pub struct GetListState<T>(SharedState<T>);

impl<T: 'static> GetListState<T> {
    pub fn with<ElemBuilder, Elem, Gen>(self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = self.0.borrow().root.clone_build();
        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.clone());

        // TODO: There should be only one of these (as `with` can only be called once).
        self.0.borrow_mut().gen_elem = Some(Box::new(move |value| generate(value).build().into()));

        // This is kind of the parent element, except we don't know about parents yet.
        // When we add it as a child, its members will be added to the new parent.
        Element {
            dom_element,
            states: vec![self.0],
            event_callbacks: Vec::new(),
        }
        .into()
    }
}

pub struct SetListState<T> {
    state: rc::Weak<RefCell<ListState<T>>>,
    // TODO: This should be a vec of modifications.
    new_elems: Rc<RefCell<Vec<T>>>,
}

impl<T> Clone for SetListState<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            new_elems: self.new_elems.clone(),
        }
    }
}

// TODO: Accept other element builder types (maybe take a function like `div`)
// How would we set attributes? Could take a Builder type and build it.
pub fn use_list_state<T: 'static>(
    root: ElementBuilder,
    initial: impl Iterator<Item = T>,
) -> (GetListState<T>, SetListState<T>) {
    let state = Rc::new(RefCell::new(ListState::new(root)));
    let new_elems: Vec<_> = initial.collect();
    let new_elems_empty = new_elems.is_empty();

    let set_list_state = SetListState {
        state: Rc::downgrade(&state),
        new_elems: Rc::new(RefCell::new(new_elems)),
    };

    if !new_elems_empty {
        queue_update(set_list_state.clone());
    }

    (GetListState(state), set_list_state)
}

impl<T: 'static> SetListState<T> {
    pub fn append(&self, new_value: T) {
        let mut new_elems = self.new_elems.borrow_mut();

        new_elems.push(new_value);

        if new_elems.len() == 1 {
            queue_update(self.clone());
        }
    }
}

impl<T: 'static> AnyStateUpdater for SetListState<T> {
    fn dom_depth(&self) -> usize {
        self.state.upgrade().map_or(0, |s| s.borrow().dom_depth())
    }

    fn apply(&self) {
        let new_elems = self.new_elems.take();

        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();

            state.extend(new_elems);
        }
    }
}

struct ListState<T> {
    root: ElementBuilder,
    gen_elem: Option<Box<dyn Fn(&T) -> Element>>,
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
}

impl<T: 'static> ListState<T> {
    fn new(root: ElementBuilder) -> Self {
        Self {
            root,
            gen_elem: None,
            parent: None,
        }
    }

    fn extend(&mut self, new_elems: Vec<T>) {
        if let Some(gen_elem) = self.gen_elem.as_ref() {
            for elem in new_elems {
                // TODO: How do we remove a child? Need to remove `states` and
                // `event_callbacks`.
                let child = gen_elem(&elem);
                // TODO: This is just a copy of the `child()` method - find a better way.
                self.root.0.append_child(&child.dom_element);
                self.root.0.states.extend(child.states);
                self.root.0.event_callbacks.extend(child.event_callbacks);
            }
        }
    }
}
