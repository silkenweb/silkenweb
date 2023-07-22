use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::MutableVec,
};

pub fn drive_vector<TSig, TVec, S, F>(sig: S, mut fun: F) -> MutableVec<TVec>
where
    TSig: 'static,
    TVec: 'static,
    S: Signal<Item = TSig> + 'static,
    F: FnMut(&MutableVec<TVec>, TSig) + 'static,
{
    let vec = MutableVec::<TVec>::new();
    let vect = vec.clone();
    crate::spawn_local(sig.for_each(move |value| {
        fun(&vect, value);
        async {}
    }));
    vec
}

#[allow(dead_code)]
pub fn drive_value<T,S>(sig: S, start_value: T) -> Mutable<T>
where
    T: 'static,
    S: Signal<Item = T> + 'static,
{
    let mutable = Mutable::new(start_value);
    let m = mutable.clone();
    crate::spawn_local(sig.for_each(move |value| {
        m.set(value);
        async {}
    }));
    mutable
}
