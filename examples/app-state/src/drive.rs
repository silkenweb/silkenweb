use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::MutableVec,
};

pub fn signal_drive_vector<TSig, TVec, S, F>(sig: S, mut fun: F) -> MutableVec<TVec>
where
    TSig: Send + Sync + 'static,
    TVec: Send + Sync + 'static,
    S: Signal<Item = TSig> + Send + 'static,
    F: FnMut(&MutableVec<TVec>, TSig) + Send + 'static,
{
    let vec = MutableVec::<TVec>::new();
    let vect = vec.clone();
    tokio::spawn(sig.for_each(move |value| {
        fun(&vect, value);
        async {}
    }));
    vec
}
