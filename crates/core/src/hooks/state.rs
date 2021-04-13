use std::{
    cell::{Cell, Ref, RefCell},
    mem,
    rc::{self, Rc},
};

use super::{queue_update, Scope, Scopeable};
use crate::{hooks::Update, Element, ElementData, MkElem};

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct GetState<T>(SharedState<T>);

impl<T: 'static> Scopeable for GetState<T> {
    type Item = T;

    fn item(&self) -> Ref<Self::Item> {
        Ref::map(self.0.borrow(), |s| &s.current)
    }

    fn link_to_parent<F>(&self, parent: rc::Weak<RefCell<crate::ElementData>>, generate: F)
    where
        F: 'static + Fn(&Self::Item) -> Element,
    {
        if let Some(p) = parent.upgrade() {
            p.borrow_mut().generate = Some(Box::new(UpdateElement {
                state: self.0.clone(),
                generate,
            }))
        }

        self.0.borrow_mut().parents.push(parent);
    }
}

struct UpdateElement<T, F>
where
    F: Fn(&T) -> Element,
{
    state: SharedState<T>,
    generate: F,
}

impl<T, F> MkElem for UpdateElement<T, F>
where
    F: Fn(&T) -> Element,
{
    fn mk_elem(&self) -> Element {
        (self.generate)(&self.state.borrow().current)
    }
}

struct StateUpdate<T> {
    state: SetState<T>,
    parent: rc::Weak<RefCell<ElementData>>,
}

impl<T> Update for StateUpdate<T> {
    fn parent(&self) -> &rc::Weak<RefCell<ElementData>> {
        &self.parent
    }

    fn apply(&self) {
        if let Some(f) = self.state.new_state.take() {
            if let Some(state) = self.state.state.upgrade() {
                f(&mut state.borrow_mut().current);
            }
        }

        if let Some(parent) = self.parent.upgrade() {
            let mut parent = parent.borrow_mut();
            let element = parent.generate.as_ref().unwrap().mk_elem();
            parent
                .dom_element
                .replace_with_with_node_1(&element.dom_element())
                .unwrap();

            let element = element.data();
            // TODO: There must be a tidier way to write this (and the entire `apply`
            // function).
            parent.dom_element = element.dom_element.clone();
            parent.children = element.children.clone();
            parent.event_callbacks = element.event_callbacks.clone();
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

pub fn use_state<T: 'static>(init: T) -> (Scope<GetState<T>>, SetState<T>) {
    let state = Rc::new(RefCell::new(State::new(init)));

    (
        Scope(GetState(state.clone())),
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
            for parent in state.borrow().parents.iter().cloned() {
                queue_update(StateUpdate {
                    state: self.clone(),
                    parent,
                });
            }
        }
    }
}

struct State<T> {
    current: T,
    parents: Vec<rc::Weak<RefCell<ElementData>>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            parents: Vec::new(),
        }
    }
}
