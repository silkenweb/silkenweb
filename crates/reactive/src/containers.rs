use std::ops::Index;

use crate::signal::Signal;

// TODO: Name
pub struct SignalVec<T> {
    data: Vec<T>,
    delta: VecDelta<T>,
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
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn delta(&self) -> &VecDelta<T> {
        &self.delta
    }

    // TODO: Docs
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.delta = VecDelta::Insert {
            index: self.len() - 1,
        }
    }

    /// # Panics
    ///
    /// If the list is empty
    pub fn pop(&mut self) {
        let item = self.data.pop().unwrap();

        let index = self.len();
        self.delta = VecDelta::Remove { index, item };
    }
}

impl<T> Index<usize> for SignalVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// TODO: Filter:
// - Store fn, with method to change it.
// - Store bool for each value in original vec.
// - Optional map (maybe_map?), as chages to filter may not update the filtered
//   list.
