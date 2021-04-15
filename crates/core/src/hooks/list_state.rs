use std::{
    cell::RefCell,
    rc::{self, Rc},
};

use super::queue_update;
use crate::{hooks::Update, Builder, Element, ElementBuilder, ElementData, MkElem};

struct SharedListState<T>(Rc<RefCell<ListState<T>>>);

impl<T> Clone for SharedListState<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct GetListState<T>(SharedListState<T>);

impl<T: 'static> GetListState<T> {
    pub fn with<ElemBuilder, Elem, Gen>(self, generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: 'static + Fn(&T) -> ElemBuilder,
    {
        let element = {
            let mut state = self.0 .0.as_ref().borrow_mut();
            let element = state.root.clone();
            state.generate_child = Some(Box::new(move |scoped| generate(scoped).build().into()));

            let parent = Rc::downgrade(&element.0);
            state.parent = Some(parent);
            element
        };

        element.0.as_ref().borrow_mut().generate = Some(Box::new(self.0));

        element.into()
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
    let state = SharedListState(Rc::new(RefCell::new(ListState::new(root))));
    let updates: Vec<_> = initial.map(ListUpdate::Push).collect();
    let updates_empty = updates.is_empty();

    let set_list_state = SetListState {
        state: Rc::downgrade(&state.0),
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
        let mut updates = self.updates.as_ref().borrow_mut();

        updates.push(update);

        if updates.len() == 1 {
            queue_update(self.clone());
        }
    }
}

impl<T: 'static> Update for SetListState<T> {
    fn apply(&self) {
        let updates = self.updates.take();
        if let Some(s) = self.state.upgrade() {
            s.borrow_mut().apply(updates)
        }
    }

    fn parent(&self) -> rc::Weak<RefCell<ElementData>> {
        self.state.upgrade().map_or_else(rc::Weak::default, |s| {
            s.borrow().parent.as_ref().unwrap().clone()
        })
    }
}

struct ListState<T> {
    root: Element,
    children: Vec<Element>,
    generate_child: Option<Box<dyn Fn(&T) -> Element>>,
    parent: Option<rc::Weak<RefCell<ElementData>>>,
}

impl<T: 'static> ListState<T> {
    fn new(root: ElementBuilder) -> Self {
        Self {
            root: root.build(),
            children: Vec::new(),
            generate_child: None,
            parent: None,
        }
    }

    fn apply(&mut self, updates: Vec<ListUpdate<T>>) {
        let generate_child = match self.generate_child.as_ref() {
            Some(gen_elem) => gen_elem,
            None => return,
        };

        for update in updates {
            match update {
                ListUpdate::Push(elem) => {
                    let child = generate_child(&elem);

                    child.0.borrow_mut().parent = Some(Rc::downgrade(&self.root.0));

                    self.root.append_child(&child.dom_element());
                    self.children.push(child);
                }
                ListUpdate::Pop => {
                    let child = self.children.pop().expect("List must be non-empty");
                    self.root.remove_child(&child.dom_element());
                }
            }
        }
    }
}
