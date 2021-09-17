use std::{
    cell::{Cell, RefCell},
    ops::Index,
    rc::Rc,
};

use crate::signal::{ReadSignal, SignalReceiver, ZipSignal};

pub struct ChangingVec<T> {
    data: Vec<T>,
    delta: VecDelta<T>,
    delta_index: DeltaIndex,
}

impl<T> Clone for ChangingVec<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

pub enum VecDelta<T> {
    Assign,
    Extend { start_index: usize },
    Insert { index: usize },
    Remove { index: usize, item: T },
    Swap { index0: usize, index1: usize },
    Set { index: usize },
}

impl<T> Default for ChangingVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ChangingVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            delta: VecDelta::Assign,
            delta_index: DeltaIndex::default(),
        }
    }

    fn set_delta(&mut self, delta: VecDelta<T>) {
        self.delta = delta;
        self.delta_index.next();
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

    pub fn delta(&self) -> &VecDelta<T> {
        &self.delta
    }

    pub fn delta_index(&self) -> DeltaIndex {
        self.delta_index
    }
}

// TODO: Should this be a method on filter?
impl<T: 'static> ChangingVec<T> {
    pub fn filter(vec: ReadSignal<Self>, filter: ReadSignal<Filter<T>>) -> ReadSignal<Self> {
        let filter_state = FilterState::default();

        (vec, filter).zip().map_to(filter_state)
    }
}

impl<T> Index<usize> for ChangingVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct DeltaIndex(u128);

impl DeltaIndex {
    pub fn next(&mut self) {
        self.0 += 1;
    }
}

// TODO: Filter:
// - Store fn, with method to change it.
// - Store bool for each value in original vec.
// - Optional map (maybe_map?), as chages to filter may not update the filtered
//   list.

pub struct Filter<T> {
    f: Rc<dyn Fn(&T) -> bool>,
    f_delta_index: DeltaIndex,
}

impl<T> Clone for Filter<T> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            f_delta_index: self.f_delta_index,
        }
    }
}

struct FilterState<T> {
    filter_delta_index: Cell<DeltaIndex>,
    data_delta_index: Cell<DeltaIndex>,
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> Default for FilterState<T> {
    fn default() -> Self {
        Self {
            filter_delta_index: Cell::new(DeltaIndex::default()),
            data_delta_index: Cell::new(DeltaIndex::default()),
            data: Rc::new(RefCell::new(Vec::default())),
        }
    }
}

impl<T> SignalReceiver<(ChangingVec<T>, Filter<T>), ChangingVec<T>> for FilterState<T>
where
    T: 'static,
{
    // TODO: Can we make self mutable here?
    fn receive(&self, data: &(ChangingVec<T>, Filter<T>)) -> ChangingVec<T> {
        let vec = &data.0;
        let filter = &data.1;

        if self.filter_delta_index.get() != filter.f_delta_index {
            // TODO: If filter has changed, rescan everything.
        }

        if self.data_delta_index.get() != vec.delta_index {
            self.data_delta_index.set(vec.delta_index);

            match vec.delta() {
                VecDelta::Assign => self.data.borrow_mut().clear(),
                _ => todo!(),
            }
        }

        todo!()
    }
}
