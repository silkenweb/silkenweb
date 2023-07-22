use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use silkenweb::clone;
use std::sync::{Arc, RwLock};

pub struct SigValue<T>(Arc<RwLock<T>>);

impl<T> SigValue<T>
where
    T: Clone + Default + 'static,
{
    pub fn new<S: Signal<Item = T> + 'static>(sig: S) -> Self {
        let val = Arc::new(RwLock::new(T::default()));

        crate::spawn_local(sig.for_each({
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

pub struct VecValue<T>(Arc<RwLock<Vec<T>>>);

impl<T> VecValue<T>
where
    T: Clone + Default + 'static,
{
    pub fn new<S: SignalVec<Item = T> + 'static>(sig: S) -> Self {
        let val = Arc::new(RwLock::new(Vec::default()));

        crate::spawn_local(sig.for_each({
            clone!(val);
            move |t| {
                let mut vec = val.write().unwrap();
                t.apply_to_vec(&mut vec);
                async {}
            }
        }));
        Self(val)
    }

    pub async fn get(&self) -> Vec<T> {
        tokio::task::yield_now().await;
        self.0.read().unwrap().clone()
    }
}
