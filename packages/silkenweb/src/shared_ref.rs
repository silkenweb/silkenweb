use std::sync::{Arc, Mutex, MutexGuard};

// TODO: This will block on contention. Is this OK? We should never actually
// have contention as we never have things shared between threads (we may move
// them to another thread, but not copy). The microtask queue should be on the
// same thread.
#[derive(Default)]
pub struct SharedRef<T>(Arc<Mutex<T>>);

impl<T> SharedRef<T> {
    pub fn new(item: T) -> Self {
        Self(Arc::new(Mutex::new(item)))
    }

    pub fn read(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

    pub fn write(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }
}

impl<T> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
