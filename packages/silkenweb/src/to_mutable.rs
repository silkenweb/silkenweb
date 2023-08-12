use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::MutableVec,
};

use crate::task::spawn_local;

pub trait SignalToMutable<TSig: Default + 'static>: Signal<Item = TSig> + Sized {
    fn to_mutable(self) -> ReadOnlyMutable<TSig>;
    fn to_mutable_vec<TVec, F>(self, update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, TSig) + 'static;
}

impl<TSig, Sig> SignalToMutable<TSig> for Sig
where
    TSig: Default + 'static,
    Sig: Signal<Item = TSig> + Sized + 'static,
{
    fn to_mutable(self) -> ReadOnlyMutable<TSig> {
        let mutable = Mutable::new(TSig::default());
        let m = mutable.clone();
        spawn_local(self.for_each(move |value| {
            m.set(value);
            async {}
        }));
        mutable.read_only()
    }

    fn to_mutable_vec<TVec, F>(self, mut update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, TSig) + 'static,
    {
        let vec = MutableVec::<TVec>::new();
        let vect = vec.clone();
        spawn_local(self.for_each(move |value| {
            update(&vect, value);
            async {}
        }));
        vec
    }
}
