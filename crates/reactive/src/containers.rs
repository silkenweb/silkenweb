use std::{
    cell::{Cell, RefCell},
    ops::Index,
    rc::Rc,
};

use crate::signal::{ReadSignal, Signal, SignalReceiver, WriteSignal, ZipSignal};

// TODO: Name
pub struct SignalVec<T> {
    data: Vec<T>,
    delta: VecDelta<T>,
    delta_index: DeltaIndex,
}

impl<T> Clone for SignalVec<T> {
    fn clone(&self) -> Self {
        todo!("Use Rc<RefCell<...>> for self.data")
    }
}

pub enum VecDelta<T> {
    Clear,
    Extend { start_index: usize },
    Insert { index: usize },
    Remove { index: usize, item: T },
    Swap { index0: usize, index1: usize },
    Set { index: usize },
}

impl<T: 'static> SignalVec<T> {
    // TODO: Are there any other ways to break the value = composition of deltas
    // invariant?

    /// Create a new [`Signal`] with an empty vec
    ///
    /// The underlying [`Vec`] is the composition of each delta seen in the
    /// signal. To maintain this invariant:
    /// - We only allow construction within a [`Signal`], so you can't assign to
    ///   the value within the signal
    /// - We don't provide a [`Clone`] impl to stop the [`SignalVec`] from
    ///   escaping the signal.
    /// This stops
    pub fn new() -> Signal<Self> {
        Signal::new(Self {
            data: Vec::new(),
            delta: VecDelta::Clear,
            delta_index: DeltaIndex::default(),
        })
    }

    fn set_delta(&mut self, delta: VecDelta<T>) {
        self.delta = delta;
        self.delta_index.next();
    }
}

impl<T: 'static> WriteSignal<SignalVec<T>> {
    // TODO: Docs
    pub fn push(&mut self, item: T) {
        self.mutate(|vec| {
            vec.data.push(item);
            vec.set_delta(VecDelta::Insert {
                index: vec.data().len() - 1,
            });
        })
    }

    /// # Panics
    ///
    /// If the list is empty
    pub fn pop(&mut self) {
        self.mutate(|vec| {
            let item = vec.data.pop().unwrap();

            let index = vec.data().len();
            vec.set_delta(VecDelta::Remove { index, item });
        });
    }
}

impl<T: 'static> SignalVec<T> {
    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn delta(&self) -> &VecDelta<T> {
        &self.delta
    }

    pub fn delta_index(&self) -> DeltaIndex {
        self.delta_index
    }

    pub fn filter(vec: ReadSignal<Self>, filter: ReadSignal<Filter<T>>) -> ReadSignal<Self> {
        let filter_state = FilterState::default();

        (vec, filter).zip().map_to(filter_state)
    }
}

impl<T> Index<usize> for SignalVec<T> {
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

impl<T> SignalReceiver<(SignalVec<T>, Filter<T>), SignalVec<T>> for FilterState<T>
where
    T: 'static,
{
    // TODO: Can we make self mutable here?
    fn receive(&self, data: &(SignalVec<T>, Filter<T>)) -> SignalVec<T> {
        let vec = &data.0;
        let filter = &data.1;

        if self.filter_delta_index.get() != filter.f_delta_index {
            // TODO: If filter has changed, rescan everything.
        }

        if self.data_delta_index.get() != vec.delta_index {
            self.data_delta_index.set(vec.delta_index);

            match vec.delta() {
                VecDelta::Clear => self.data.borrow_mut().clear(),
                _ => todo!(),
            }
        }

        todo!()
    }
}
