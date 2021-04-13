use std::{
    cell::RefCell,
    rc::{self, Rc},
};

use super::queue_update;
use crate::{dom_depth, hooks::Update, Builder, Element, ElementBuilder, OwnedChild};

impl<T> OwnedChild for ListState<T> {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}

type SharedListState<T> = Rc<RefCell<ListState<T>>>;

pub struct GetListState<T>(SharedListState<T>);

impl<T: 'static> GetListState<T> {
    pub fn with<ElemBuilder, Elem, Gen>(self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = self.0.borrow().root.clone();
        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.clone());

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

enum ListUpdate<T> {
    Push(T),
    Pop,
}

pub struct SetListState<T> {
    state: rc::Weak<RefCell<ListState<T>>>,
    updates: Rc<RefCell<Vec<ListUpdate<T>>>>,
}

impl<T> Clone for SetListState<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            updates: self.updates.clone(),
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
    let updates: Vec<_> = initial.map(ListUpdate::Push).collect();
    let updates_empty = updates.is_empty();

    let set_list_state = SetListState {
        state: Rc::downgrade(&state),
        updates: Rc::new(RefCell::new(updates)),
    };

    if !updates_empty {
        queue_update(set_list_state.clone());
    }

    (GetListState(state), set_list_state)
}

impl<T: 'static> SetListState<T> {
    pub fn push(&self, new_value: T) {
        self.push_update(ListUpdate::Push(new_value));
    }

    pub fn pop(&self) {
        self.push_update(ListUpdate::Pop);
    }

    fn push_update(&self, update: ListUpdate<T>) {
        let mut updates = self.updates.borrow_mut();

        updates.push(update);

        if updates.len() == 1 {
            queue_update(self.clone());
        }
    }
}

impl<T: 'static> Update for SetListState<T> {
    fn dom_depth(&self) -> usize {
        self.state.upgrade().map_or(0, |s| s.borrow().dom_depth())
    }

    fn apply(&self) {
        let updates = self.updates.take();

        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();

            state.apply(updates);
        }
    }
}

struct ListState<T> {
    root: Element,
    children: Vec<Element>,
    gen_elem: Option<Box<dyn Fn(&T) -> Element>>,
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
}

impl<T: 'static> ListState<T> {
    fn new(root: ElementBuilder) -> Self {
        Self {
            root: root.build(),
            children: Vec::new(),
            gen_elem: None,
            parent: None,
        }
    }

    fn apply(&mut self, updates: Vec<ListUpdate<T>>) {
        let gen_elem = match self.gen_elem.as_ref() {
            Some(gen_elem) => gen_elem,
            None => return,
        };

        for update in updates {
            match update {
                ListUpdate::Push(elem) => {
                    let child = gen_elem(&elem);

                    if let Some(parent) = self.parent.as_ref().and_then(rc::Weak::upgrade) {
                        child.set_parents(parent);
                    }

                    self.root.append_child(&child.dom_element);
                    self.children.push(child);
                }
                ListUpdate::Pop => {
                    let child = self.children.pop().expect("List must be non-empty");
                    self.root
                        .dom_element
                        .remove_child(&child.dom_element)
                        .unwrap();
                }
            }
        }
    }
}
