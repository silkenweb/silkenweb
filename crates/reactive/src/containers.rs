use crate::signal::Signal;

// TODO: Name
pub struct SignalVec<T> {
    data: Vec<T>,
    delta: VecDelta,
}

pub enum VecDelta {
    Clear,
    Extend { start_index: usize },
    Insert { index: usize },
    Remove { index: usize },
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

    pub fn delta(&self) -> &VecDelta {
        &self.delta
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.delta = VecDelta::Insert { index: self.len() }
    }
}

// TODO: Filter:
// - Store fn, with method to change it.
// - Store bool for each value in original vec.
// - Optional map (maybe_map?), as chages to filter may not update the filtered
//   list.
