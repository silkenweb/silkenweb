use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::MutableVec,
};

pub fn signal_drive_vector<TSig, TVec, S, F>(sig: S, mut fun: F) -> MutableVec<TVec>
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
