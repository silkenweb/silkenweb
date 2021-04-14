use std::{
    cell::{Cell, RefCell},
    rc::{self, Rc},
};

use super::queue_update;
use crate::{hooks::Update, Builder, Element, ElementData, MkElem};

type SharedState<T> = Rc<RefCell<State<T>>>;

pub struct GetState<T>(SharedState<T>);

impl<T: 'static> GetState<T> {
    pub fn with<ElemBuilder, Elem, Gen>(&self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = generate(&self.0.borrow().current).build().into();

        // TODO: What happens when we nest `with` calls on the same element. e.g.
        // parent.with(|x| child.with(|x| ...))? Replacing the generator should work Ok.
        let parent = Rc::downgrade(&element.0);

        if let Some(p) = parent.upgrade() {
            p.borrow_mut().generate = Some(Box::new(UpdateElement {
                state: self.0.clone(),
                generate: move |scoped| generate(scoped).build().into(),
            }))
        }

        self.0.borrow_mut().parents.push(parent);

        element.into()
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

            let element = Rc::try_unwrap(element.0).ok().unwrap().into_inner();
            // TODO: There must be a tidier way to write this (and the entire `apply`
            // function).
            parent.dom_element = element.dom_element;
            parent.children = element.children;
            parent.event_callbacks = element.event_callbacks;
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
