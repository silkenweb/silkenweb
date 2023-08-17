use async_trait::async_trait;
use futures::{Future, StreamExt};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, MutableVecLockMut, SignalVec, SignalVecExt, VecDiff},
};

use crate::task::spawn_local;

#[async_trait(?Send)]
pub trait TaskSignal: Signal {
    async fn to_mutable(self) -> ReadOnlyMutable<Self::Item>;
    fn spawn_for_each<TVec, F>(self, update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, Self::Item) + 'static;
}

#[async_trait(?Send)]
impl<Sig> TaskSignal for Sig
where
    Sig: Signal + 'static,
{
    async fn to_mutable(self) -> ReadOnlyMutable<Self::Item> {
        let mut s = Box::pin(self.to_stream());
        let first_value = s
            .next()
            .await
            .expect("a Signal should always have an initial value");
        let mutable = Mutable::new(first_value);
        spawn_local({
            let mutable = mutable.clone();
            async move {
                while let Some(value) = s.next().await {
                    mutable.set(value);
                }
            }
        });
        mutable.read_only()
    }

    fn spawn_for_each<TVec, F>(self, mut update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, Self::Item) + 'static,
    {
        let vec = MutableVec::<TVec>::new();
        spawn_local(self.for_each({
            let vec = vec.clone();
            move |value| {
                update(&vec, value);
                async {}
            }
        }));
        vec
    }
}

pub trait TaskSignalVec: SignalVec {
    fn to_mutable(self) -> MutableVec<Self::Item>;

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(VecDiff<Self::Item>) -> U + 'static;
}

impl<Sig> TaskSignalVec for Sig
where
    Self::Item: Clone + 'static,
    Sig: SignalVec + 'static,
{
    fn to_mutable(self) -> MutableVec<Self::Item> {
        let mv = MutableVec::new();

        self.spawn_for_each({
            let mv = mv.clone();
            move |diff| {
                MutableVecLockMut::apply_vec_diff(&mut mv.lock_mut(), diff);
                async {}
            }
        });

        mv
    }

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(VecDiff<Self::Item>) -> U + 'static,
    {
        spawn_local(self.for_each(callback));
    }
}
