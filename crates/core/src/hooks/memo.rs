use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    mem,
    rc::Rc,
};

#[derive(Clone, Default)]
pub struct Memo(Rc<RefCell<MemoData>>);

impl Memo {
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

    // TODO: Safer interface for this. Need something like `memo.use(&self) ->
    // UseMemo` which calls `finish_render` on drop.
    /// Clients must call `self.finish_render()` after each render has finished.
    pub fn finish_render(&self) {
        let mut memo = self.0.borrow_mut();
        memo.current_memoized = mem::take(&mut memo.next_memoized);
    }

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
        next_memos.insert(key, value.clone());
        value
    }
}

type AnyMap = HashMap<(TypeId, TypeId), Box<dyn Any>>;

#[derive(Default)]
struct MemoData {
    current_memoized: AnyMap,
    next_memoized: AnyMap,
}
