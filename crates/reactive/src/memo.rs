//! Memoize functions across frames
//!
//! Typically a [`MemoCache`] will last the duration of a UI component, whereas a
//! [`MemoFrame`] will last the duration of a single render.
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    mem,
    rc::Rc,
};

type SharedMemoData = Rc<RefCell<MemoData>>;

/// [`MemoCache`] holds the map of keys to cached values.
#[derive(Clone, Default)]
pub struct MemoCache(SharedMemoData);

impl MemoCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new *frame*. Values will only be cached until the next *frame*.
    /// If a value is used before [`MemoFrame`] is destroyed, it will be cached
    /// for the next frame, otherwise it will be removed from the cache.
    pub fn frame(&self) -> MemoFrame {
        MemoFrame(self.0.clone())
    }
}

/// A [`MemoFrame`] represents the scope of a *frame* for the [`MemoCache`].
pub struct MemoFrame(SharedMemoData);

impl MemoFrame {
    /// Lookup a value in the cache.
    ///
    /// If a value is not there, it will be generated using `value_fn`. All
    /// functional dependencies of `value_fn` should be included in `key`.
    ///
    /// The value will be cached for the next frame, whether it's new or
    /// existing.
    ///
    /// It is up to the client to use a key that uniquely identifies the
    /// functional dependencies of variables captured by `value_fn`.
    pub fn cache<Key, Value, ValueFn>(&self, key: Key, value_fn: ValueFn) -> Value
    where
        Key: 'static + Eq + Hash,
        Value: 'static + Clone,
        ValueFn: FnOnce() -> Value,
    {
        let mut memo = self.0.borrow_mut();

        let current_memos = Self::memo_map::<Key, Value>(&mut memo.current_memoized);
        let value = current_memos.remove(&key).unwrap_or_else(value_fn);

        let next_memos = Self::memo_map::<Key, Value>(&mut memo.next_memoized);
        let previous_value = next_memos.insert(key, value.clone());

        assert!(
            previous_value.is_none(),
            "Keys can't be reused within a frame"
        );

        value
    }

    fn memo_map<'a, Key: 'static, Value: 'static>(
        any_map: &'a mut AnyMap,
    ) -> &'a mut HashMap<Key, Value> {
        let type_key = (TypeId::of::<Key>(), TypeId::of::<Value>());
        any_map
            .entry(type_key)
            .or_insert_with(|| Box::new(HashMap::<Key, Value>::new()))
            .downcast_mut()
            .unwrap()
    }
}

impl Drop for MemoFrame {
    fn drop(&mut self) {
        let mut memo = self.0.borrow_mut();
        memo.current_memoized = mem::take(&mut memo.next_memoized);
    }
}

type AnyMap = HashMap<(TypeId, TypeId), Box<dyn Any>>;

#[derive(Default)]
struct MemoData {
    current_memoized: AnyMap,
    next_memoized: AnyMap,
}
