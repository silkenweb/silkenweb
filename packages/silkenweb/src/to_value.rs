use std::sync::{Arc, RwLock};

use crate::{
    clone,
    task::{render_now, spawn_local},
};
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};

pub trait SignalToValue<T: Default + 'static> {
    fn to_value(self) -> SigValue<T>;
}

impl<T, Sig> SignalToValue<T> for Sig
where
    T: Default + 'static,
    Sig: Signal<Item = T> + Sized + 'static,
{
    fn to_value(self) -> SigValue<T> {
        SigValue::new(self)
    }
}

pub struct SigValue<T>(Arc<RwLock<T>>);

impl<T: Default + 'static> SigValue<T> {
    fn new<S: Signal<Item = T> + 'static>(sig: S) -> Self {
        let val = Arc::new(RwLock::new(T::default()));

        spawn_local(sig.for_each({
            clone!(val);
            move |t| {
                let mut v = val.write().unwrap();
                *v = t;
                async {}
            }
        }));
        Self(val)
    }

    pub async fn with_ref<R, F: Fn(&T) -> R>(&self, func: F) -> R {
        render_now().await;
        let r = self.0.read().unwrap();
        func(&r)
    }
}

impl<T: Clone + 'static> SigValue<T> {
    pub async fn cloned(&self) -> T {
        render_now().await;
        self.0.read().unwrap().clone()
    }
}

pub trait SignalVecToValue<T: Default + 'static> {
    fn to_value(self) -> VecValue<T>;
}

impl<T, Sig> SignalVecToValue<T> for Sig
where
    T: Default + 'static,
    Sig: SignalVec<Item = T> + Sized + 'static,
{
    fn to_value(self) -> VecValue<T> {
        VecValue::new(self)
    }
}

pub struct VecValue<T>(Arc<RwLock<Vec<T>>>);

impl<T: Default + 'static> VecValue<T> {
    fn new<S: SignalVec<Item = T> + 'static>(sig: S) -> Self {
        let val = Arc::new(RwLock::new(Vec::default()));

        spawn_local(sig.for_each({
            clone!(val);
            move |t| {
                let mut vec = val.write().unwrap();
                t.apply_to_vec(&mut vec);
                async {}
            }
        }));
        Self(val)
    }

    pub async fn with_ref<R, F: Fn(&Vec<T>) -> R>(&self, func: F) -> R {
        render_now().await;
        let r = self.0.read().unwrap();
        func(&r)
    }
}

impl<T: Clone + 'static> VecValue<T> {
    pub async fn cloned(&self) -> Vec<T> {
        render_now().await;
        self.0.read().unwrap().clone()
    }
}