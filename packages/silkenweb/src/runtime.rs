use clonelet::clone;
use futures::{Future, FutureExt, StreamExt};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, MutableVecLockMut, SignalVec, SignalVecExt, VecDiff},
};

use crate::task::spawn_local;

// TODO: Docs
pub trait RuntimeSignal: Signal {
    fn to_mutable(self) -> ReadOnlyMutable<Self::Item>;

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(Self::Item) -> U + 'static;
}

impl<Sig> RuntimeSignal for Sig
where
    Sig: Signal + 'static,
{
    fn to_mutable(self) -> ReadOnlyMutable<Self::Item> {
        let mut s = Box::pin(self.to_stream());
        let first_value = s
            .next()
            .now_or_never()
            .expect("A `Signal`'s initial value must be `Ready` immediately")
            .expect("`Signal`s must have an initial value");
        let mutable = Mutable::new(first_value);

        spawn_local({
            clone!(mutable);

            async move {
                while let Some(value) = s.next().await {
                    mutable.set(value);
                }
            }
        });

        mutable.read_only()
    }

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(Self::Item) -> U + 'static,
    {
        spawn_local(self.for_each(callback));
    }
}

// TODO: Docs
pub trait RuntimeSignalVec: SignalVec {
    fn to_mutable(self) -> MutableVec<Self::Item>;

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(VecDiff<Self::Item>) -> U + 'static;
}

impl<Sig> RuntimeSignalVec for Sig
where
    Self::Item: Clone + 'static,
    Sig: SignalVec + 'static,
{
    fn to_mutable(self) -> MutableVec<Self::Item> {
        let mv = MutableVec::new();

        self.spawn_for_each({
            clone!(mv);

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
