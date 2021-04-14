use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    mem,
    rc::Rc,
};

pub struct Reference<T>(Rc<RefCell<T>>);

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Reference<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    pub fn replace(&self, new: T) -> T {
        mem::replace(&mut self.borrow_mut(), new)
    }

    pub fn replace_with<F>(&self, f: F) -> T
    where
        F: FnOnce(&mut T) -> T,
    {
        let old = &mut self.borrow_mut();
        let new = f(old);
        mem::replace(old, new)
    }

    pub fn take(&self) -> T
    where
        T: Default,
    {
        mem::take(&mut self.borrow_mut())
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.0.try_borrow()
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        self.0.try_borrow_mut()
    }
}
