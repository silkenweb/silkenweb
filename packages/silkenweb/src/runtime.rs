use async_trait::async_trait;
use futures::StreamExt;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, MutableVecLockMut, SignalVec, SignalVecExt, VecDiff},
};

use crate::task::spawn_local;

#[async_trait(?Send)]
pub trait TaskSignal: Signal + Sized {
    async fn to_mutable(self) -> ReadOnlyMutable<Self::Item>;
    fn spawn_for_each<TVec, F>(self, update: F) -> MutableVec<TVec>
    where
        TVec: 'static,
        F: FnMut(&MutableVec<TVec>, Self::Item) + 'static;
}

#[async_trait(?Send)]
impl<Sig> TaskSignal for Sig
where
    Sig: Signal + Sized + 'static,
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
}

impl<Sig> TaskSignalVec for Sig
where
    Self::Item: Clone + 'static,
    Sig: SignalVec + 'static,
{
    fn to_mutable(self) -> MutableVec<Self::Item> {
        let mv = MutableVec::new();

        spawn_local(self.for_each({
            let mv = mv.clone();
            move |diff| {
                apply_diff(diff, mv.lock_mut());
                async {}
            }
        }));
        mv
    }
}

fn apply_diff<T: Clone>(diff: VecDiff<T>, mut vec: MutableVecLockMut<T>) {
    match diff {
        VecDiff::Replace { values } => {
            vec.clear();
            values.into_iter().for_each(|v| vec.push_cloned(v));
        }
        VecDiff::InsertAt { index, value } => {
            vec.insert_cloned(index, value);
        }
        VecDiff::UpdateAt { index, value } => {
            vec.set_cloned(index, value);
        }
        VecDiff::Push { value } => {
            vec.push_cloned(value);
        }
        VecDiff::RemoveAt { index } => {
            vec.remove(index);
        }
        VecDiff::Move {
            old_index,
            new_index,
        } => {
            let value = vec.remove(old_index);
            vec.insert_cloned(new_index, value);
        }
        VecDiff::Pop {} => {
            vec.pop().unwrap();
        }
        VecDiff::Clear {} => {
            vec.clear();
        }
    }
}
