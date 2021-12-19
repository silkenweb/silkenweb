use std::{cell::RefCell, ops::Index, rc::Rc};

use crate::signal::{ReadSignal, SignalReceiver, ZipSignal};

// TODO: Move this into a `vec` sub-module?

// TODO: Rename to `ChangeTrackingVec`?
pub struct ChangeTrackingVec<T> {
    data: Vec<T>,
    delta: Option<VecDelta<T>>,
    delta_id: DeltaId,
}

impl<T: Clone> Clone for ChangeTrackingVec<T> {
    fn clone(&self) -> Self {
        // We can't derive `Clone` because `self` and the clone can evolve
        // independantly, so we need a new `delta_id`.
        Self {
            data: self.data.clone(),
            delta: None,
            delta_id: DeltaId::default(),
        }
    }
}

impl<T> Default for ChangeTrackingVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ChangeTrackingVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            delta: None,
            delta_id: DeltaId::default(),
        }
    }

    // TODO: Docs
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.set_delta(VecDelta::Insert {
            index: self.data().len() - 1,
        });
    }

    /// # Panics
    ///
    /// If the list is empty
    pub fn pop(&mut self) {
        let item = self.data.pop().unwrap();

        let index = self.data().len();
        self.set_delta(VecDelta::Remove { index, item });
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn snapshot(&self) -> DeltaId {
        self.delta_id.clone()
    }

    pub fn delta(&self, previous: &DeltaId) -> Option<&VecDelta<T>> {
        if self.delta_id.is_next(previous) {
            self.delta.as_ref()
        } else {
            None
        }
    }

    fn set_delta(&mut self, delta: VecDelta<T>) {
        self.delta = Some(delta);
        self.delta_id.next();
    }
}

// TODO: Should this be a method on filter?
impl<T: 'static + Clone> ChangeTrackingVec<T> {
    pub fn filter(vec: ReadSignal<Self>, filter: ReadSignal<Filter<T>>) -> ReadSignal<Self> {
        let filter_state = FilterState::default();

        (vec, filter).zip().map_to(filter_state)
    }
}

impl<T> Index<usize> for ChangeTrackingVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub enum VecDelta<T> {
    Extend { start_index: usize },
    Insert { index: usize },
    Remove { index: usize, item: T },
    Set { index: usize },
}

#[derive(Default, Clone)]
pub struct DeltaId {
    index: u128,
    object_id: Rc<u8>,
}

impl DeltaId {
    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_next(&self, previous: &DeltaId) -> bool {
        self.index == previous.index + 1 && Rc::ptr_eq(&self.object_id, &previous.object_id)
    }
}

// TODO: Filter:
// - Store fn, with method to change it.
// - Store bool for each value in original vec.
// - Optional map (maybe_map?), as chages to filter may not update the filtered
//   list.

#[derive(Clone)]
pub struct Filter<T> {
    _f: Rc<dyn Fn(&T) -> bool>,
    _f_delta_index: DeltaId,
}

struct FilterState<T> {
    _filter_delta_index: RefCell<DeltaId>,
    _data_delta_index: RefCell<DeltaId>,
    _data: Rc<RefCell<Vec<T>>>,
}

impl<T> Default for FilterState<T> {
    fn default() -> Self {
        Self {
            _filter_delta_index: RefCell::new(DeltaId::default()),
            _data_delta_index: RefCell::new(DeltaId::default()),
            _data: Rc::new(RefCell::new(Vec::default())),
        }
    }
}

impl<T> SignalReceiver<(ChangeTrackingVec<T>, Filter<T>), ChangeTrackingVec<T>> for FilterState<T>
where
    T: 'static,
{
    // TODO: Can we make self mutable here?
    fn receive(&self, _data: &(ChangeTrackingVec<T>, Filter<T>)) -> ChangeTrackingVec<T> {
        todo!()
    }
}
