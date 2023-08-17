#![allow(dead_code)]

use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::{clone, TaskSignal};

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
        let list = MutableVec::new();

        count.signal().spawn_for_each({
            clone!(list);
            move |value| {
                list.lock_mut().push(value);
                async {}
            }
        });
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
    use silkenweb::{
        task::{render_now, server},
        TaskSignal, TaskSignalVec,
    };

    use super::CounterState;

    #[tokio::test]
    async fn test_counter() {
        server::scope(async {
            let state = CounterState::default();
            let text = state.text().to_mutable().await;
            let list = state.list.signal_vec().to_mutable();

            render_now().await;
            assert_eq!(state.count().get(), 0);
            assert_eq!(&*list.lock_ref(), [0]);
            assert_eq!(text.get_cloned(), "0");

            state.add(1);
            render_now().await;
            assert_eq!(text.get_cloned(), "1");
            assert_eq!(&*list.lock_ref(), [0, 1]);

            state.add(-2);
            render_now().await;
            assert_eq!(text.get_cloned(), "-1");
            assert_eq!(&*list.lock_ref(), [0, 1, -1]);
        })
        .await;
    }
}
