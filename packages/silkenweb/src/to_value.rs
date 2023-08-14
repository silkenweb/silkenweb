use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt, VecDiff};

use crate::{clone, task::spawn_local};

pub trait SignalVecToValue<T: Clone + Default + 'static> {
    fn to_mutable(self) -> MutableVec<T>;
}

impl<T, Sig> SignalVecToValue<T> for Sig
where
    T: Clone + Default + 'static,
    Sig: SignalVec<Item = T> + Sized + 'static,
{
    fn to_mutable(self) -> MutableVec<T> {
        // TODO get first value
        let mv = MutableVec::<T>::new();

        spawn_local(self.for_each({
            clone!(mv);
            move |diff| {
                let mut vec = mv.lock_mut();

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
                async {}
            }
        }));
        mv
    }
}
