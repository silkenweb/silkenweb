#![allow(dead_code)]

use futures_signals::signal::{Mutable, Signal, SignalExt};

pub struct CounterState {
    count: Mutable<isize>,
}

impl Default for CounterState {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterState {
    fn new() -> Self {
        let count = Mutable::new(0);
        Self { count }
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
}

#[cfg(test)]
mod test {
    use super::CounterState;
    use futures_signals::signal::{Signal, SignalExt};
    use silkenweb::clone;
    use std::sync::{Arc, RwLock};

    #[tokio::test]
    async fn test_counter() {
        let state = CounterState::default();
        let val = Value::new(state.text());
        assert_eq!(state.count().get(), 0);

        assert_eq!(val.get().await, "0");

        state.add(1);
        assert_eq!(val.get().await, "1");

        state.add(-2);
        assert_eq!(val.get().await, "-1");
    }

    struct Value<T>(Arc<RwLock<T>>);

    impl<T> Value<T>
    where
        T: Clone + Default + Sync + Send + 'static,
    {
        pub fn new<S: Signal<Item = T> + Sync + Send + 'static>(sig: S) -> Self {
            let val = Arc::new(RwLock::new(T::default()));

            tokio::spawn(sig.for_each({
                clone!(val);
                move |t| {
                    let mut v = val.write().unwrap();
                    *v = t;
                    async {}
                }
            }));
            Self(val)
        }

        pub async fn get(&self) -> T {
            tokio::task::yield_now().await;
            self.0.read().unwrap().clone()
        }
    }
}
