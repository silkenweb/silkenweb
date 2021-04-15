use std::{
    cell::{Cell, Ref, RefCell},
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
        element.0.borrow_mut().generate = Some(Box::new(UpdateElement {
            state: self.0.clone(),
            generate: move |scoped| generate(scoped).build().into(),
        }));

        let parent = Rc::downgrade(&element.0);
        self.0.borrow_mut().parents.push(parent);

        // TODO: If there are any pending updates, queue them for this parent.
        // TODO: Remove null weak refs when we update parents.

        element.into()
    }

    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |state| &state.current)
    }

    // TODO: Remove existing `with` and rename this to `with`
    pub fn with_derived<U, Generate>(&self, generate: Generate) -> GetState<U>
    where
        U: 'static,
        Generate: 'static + Fn(&T) -> U,
    {
        let (value, set_value) = use_state(generate(&self.0.borrow().current));

        self.0
            .borrow_mut()
            .dependents
            .push(Rc::new({let value = self.0.clone(); move |new_value| {
                let existing = value.clone();
                web_log::println!("heher");
                set_value.set(generate(new_value))
            }}));

        // TODO: Store state updates in SharedState. If there are any pending, queue
        // updates for the new dependent.

        value
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
    fn parent(&self) -> rc::Weak<RefCell<ElementData>> {
        self.parent.clone()
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

struct UpdateDependent<T> {
    state: SetState<T>,
    dependent: rc::Weak<dyn Fn(&T)>,
}

impl<T> Update for UpdateDependent<T> {
    fn apply(&self) {
        if let Some(f) = self.state.new_state.take() {
            if let Some(state) = self.state.state.upgrade() {
                f(&mut state.borrow_mut().current);
            }
        }

        if let Some(dependent) = self.dependent.upgrade() {
            if let Some(state) = self.state.state.upgrade() {
                dependent(&mut state.borrow_mut().current);
            }
        }
    }

    // TODO: Remove this from trait
    fn parent(&self) -> rc::Weak<RefCell<ElementData>> {
        todo!("Remove this from trait")
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
        web_log::println!("Here4");

        if let Some(state) = self.state.upgrade() {
            web_log::println!("Here3");
            for parent in state.borrow().parents.iter().cloned() {
                web_log::println!("Here2");
                queue_update(StateUpdate {
                    state: self.clone(),
                    parent,
                });
            }

            for dependent in &state.borrow().dependents {
                web_log::println!("Here1");
                queue_update(UpdateDependent {
                    state: self.clone(),
                    dependent: Rc::downgrade(dependent),
                });
            }
        }
    }
}

struct State<T> {
    current: T,
    dependents: Vec<Rc<dyn Fn(&T)>>,
    // TODO: Remove parents and use dependents.
    parents: Vec<rc::Weak<RefCell<ElementData>>>,
}

impl<T: 'static> State<T> {
    fn new(init: T) -> Self {
        Self {
            current: init,
            dependents: Vec::new(),
            parents: Vec::new(),
        }
    }
}
