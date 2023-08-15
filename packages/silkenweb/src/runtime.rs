use async_trait::async_trait;
use futures::StreamExt;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::MutableVec,
};

use crate::task::spawn_local;

#[async_trait(?Send)]
pub trait TaskSignal<TSig: 'static>: Signal<Item = TSig> + Sized {
    async fn try_to_mutable(self) -> Option<ReadOnlyMutable<TSig>>;
    fn spawn_for_each<TVec, F>(self, update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, TSig) + 'static;
}

#[async_trait(?Send)]
impl<TSig, Sig> TaskSignal<TSig> for Sig
where
    TSig: 'static,
    Sig: Signal<Item = TSig> + Sized + 'static,
{
    async fn try_to_mutable(self) -> Option<ReadOnlyMutable<TSig>> {
        let mut s = Box::pin(self.to_stream());
        let first_value = s.next().await?;
        let mutable = Mutable::new(first_value);
        let m = mutable.clone();
        spawn_local({
            async move {
                while let Some(value) = s.next().await {
                    m.set(value);
                }
            }
        });
        Some(mutable.read_only())
    }

    fn spawn_for_each<TVec, F>(self, mut update: F) -> MutableVec<TVec>
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
