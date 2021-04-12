use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    mem,
    rc::{self, Rc},
};

use crate::{dom_depth, hooks::Scopeable, Element, OwnedChild};

pub struct Reference<T>(Rc<RefCell<RefData<T>>>);

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Reference<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(RefData {
            parent: None,
            elements: Vec::new(),
            value,
        })))
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |x| &x.value)
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.value)
    }

    pub fn replace(&self, new: T) -> T {
        mem::replace(&mut self.0.borrow_mut().value, new)
    }

    pub fn replace_with<F>(&self, f: F) -> T
    where
        F: FnOnce(&mut T) -> T,
    {
        let old = &mut self.0.borrow_mut().value;
        let new = f(old);
        mem::replace(old, new)
    }

    pub fn take(&self) -> T
    where
        T: Default,
    {
        mem::take(&mut self.0.borrow_mut().value)
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.0
            .try_borrow()
            .map(|borrowed| Ref::map(borrowed, |x| &x.value))
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        self.0
            .try_borrow_mut()
            .map(|borrowed| RefMut::map(borrowed, |x| &mut x.value))
    }
}

impl<T: 'static> Scopeable for Reference<T> {
    fn add_child(&mut self, element: Element) {
        self.0.borrow_mut().elements.push(element);
    }

    fn as_child(&self) -> Rc<RefCell<dyn OwnedChild>> {
        self.0.clone()
    }
}
struct RefData<T> {
    parent: Option<rc::Weak<RefCell<dyn OwnedChild>>>,
    elements: Vec<Element>,
    value: T,
}

impl<T> OwnedChild for RefData<T> {
    fn set_parent(&mut self, parent: rc::Weak<RefCell<dyn OwnedChild>>) {
        self.parent = Some(parent);
    }

    fn dom_depth(&self) -> usize {
        dom_depth(&self.parent)
    }
}
