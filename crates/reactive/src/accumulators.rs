use std::cell::RefCell;

use num_traits::{WrappingAdd, WrappingSub, Zero};

use crate::signal::{ReadSignal, Signal, SignalReceiver, WriteSignal};

#[derive(Clone)]
pub struct SumTotal<T> {
    deltas: WriteSignal<T>,
    total: ReadSignal<T>,
}

impl<T: 'static + Zero + Clone + WrappingAdd> Default for SumTotal<T> {
    fn default() -> Self {
        let deltas = Signal::new(T::zero());
        let total = deltas.read().map_to(AccumulateSum(RefCell::new(T::zero())));
        Self {
            deltas: deltas.write(),
            total,
        }
    }
}

impl<T: 'static> SumTotal<T> {
    pub fn read(&self) -> ReadSignal<T> {
        self.total.clone()
    }
}

pub struct Sum<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> {
    current: RefCell<T>,
    total: SumTotal<T>,
}

impl<T: 'static + Zero + Clone + Zero + WrappingAdd + WrappingSub> Sum<T> {
    pub fn new(total: &SumTotal<T>) -> Self {
        Self {
            current: RefCell::new(T::zero()),
            total: total.clone(),
        }
    }
}

impl<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> SignalReceiver<T, IncludeSum>
    for Sum<T>
{
    fn receive(&self, x: &T) -> IncludeSum {
        let delta = x.wrapping_sub(&self.current.borrow());
        self.current.replace(x.clone());
        self.total.deltas.set(delta);
        IncludeSum()
    }
}

impl<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> Drop for Sum<T> {
    fn drop(&mut self) {
        self.total
            .deltas
            .set(T::zero().wrapping_sub(&self.current.borrow()));
    }
}

pub struct IncludeSum();

struct AccumulateSum<T>(RefCell<T>);

impl<T: 'static + Clone + WrappingAdd> SignalReceiver<T, T> for AccumulateSum<T> {
    fn receive(&self, x: &T) -> T {
        let mut total = self.0.borrow_mut();
        *total = total.wrapping_add(x);
        total.clone()
    }
}
