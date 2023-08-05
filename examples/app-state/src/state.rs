#![allow(dead_code)]

use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};

use crate::drive::SignalToMutable;

pub struct CounterState {
    count: Mutable<isize>,
    list: MutableVec<isize>,
}

impl Default for CounterState {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterState {
    fn new() -> Self {
        let count = Mutable::new(0);
        let list = count
            .signal()
            .to_mutable_vec(|vec, value| vec.lock_mut().push(value));
        Self { count, list }
    }

    pub fn count(&self) -> Mutable<isize> {
        self.count.clone()
    }

    pub fn add(&self, val: isize) {
        self.count.replace_with(|i| *i + val);
    }

    pub fn text(&self) -> impl Signal<Item = String> {
        self.count.signal().map(|i| i.to_string())
    }

    pub fn list(&self) -> impl Signal<Item = String> {
        self.list
            .signal_vec()
            .to_signal_cloned()
            .map(|vec| format!("{vec:?}"))
    }
}

#[cfg(test)]
mod test {
    use super::CounterState;
    use crate::test_utils::{SigValue, VecValue};

    #[tokio::test]
    async fn test_counter() {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let state = CounterState::default();

                let text = SigValue::new(state.text());
                let list = VecValue::new(state.list.signal_vec());

                assert_eq!(state.count().get(), 0);
                assert_eq!(list.get().await, [0]);
                assert_eq!(text.get().await, "0");

                state.add(1);
                assert_eq!(text.get().await, "1");
                assert_eq!(list.get().await, [0, 1]);

                state.add(-2);
                assert_eq!(text.get().await, "-1");
                assert_eq!(list.get().await, [0, 1, -1]);
            })
            .await;
    }
}
