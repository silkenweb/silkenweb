use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct State<T>(Rc<RefCell<T>>);

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> State<T> {
    pub fn new(x: T) -> Self {
        Self(Rc::new(RefCell::new(x)))
    }

    pub fn get_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    pub fn get(&self) -> Ref<T> {
        self.0.borrow()
    }
}
