#![allow(dead_code)]

use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::SignalToMutable;

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
            .spawn_for_each(|vec, value| vec.lock_mut().push(value));
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
    use silkenweb::{task::server, SignalToValue, SignalVecToValue};

    use super::CounterState;

    #[tokio::test]
    async fn test_counter() {
        server::scope(async {
            let state = CounterState::default();

            let text = state.text().to_value();
            let list = state.list.signal_vec().to_value();

            assert_eq!(state.count().get(), 0);
            assert_eq!(list.cloned().await, [0]);
            assert_eq!(text.cloned().await, "0");

            state.add(1);
            assert_eq!(text.cloned().await, "1");
            assert_eq!(list.cloned().await, [0, 1]);

            state.add(-2);
            assert_eq!(text.cloned().await, "-1");
            assert_eq!(list.cloned().await, [0, 1, -1]);
        })
        .await;
    }
}
