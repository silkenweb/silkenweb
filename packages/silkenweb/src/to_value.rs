use std::sync::{Arc, RwLock};

use futures_signals::signal_vec::{SignalVec, SignalVecExt};

use crate::{
    clone,
    task::{render_now, spawn_local},
};

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
