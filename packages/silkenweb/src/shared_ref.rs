use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

// TODO: This will block on contention. Is this OK? We should never actually
// have contention as we never have things shared between threads (we may move
// them to another thread, but not copy). The microtask queue should be on the
// same thread.
#[derive(Default)]
pub struct SharedRef<T>(Arc<RwLock<T>>);

impl<T> SharedRef<T> {
    pub fn new(item: T) -> Self {
        Self(Arc::new(RwLock::new(item)))
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}

impl<T> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
