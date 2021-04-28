//! Accumulate reactive variables into a reactive total
//!
//! The example below shows how to sum up `first_digit` and `second_digit` into
//! `total`. When we drop `first_digit`, it is removed from the total.
//!
//!```
//! # use silkenweb_reactive::{signal::SignalReceiver, accumulators::{SumTotal, SumElement}};
//! # use std::mem;
//! let total = SumTotal::<usize>::default();
//! let first_digit = SumElement::new(&total);
//! let second_digit = SumElement::new(&total);
//! let total = total.read();
//!
//! first_digit.receive(&1);
//! assert_eq!(1, *total.current());
//! second_digit.receive(&10);
//! assert_eq!(11, *total.current());
//!
//! mem::drop(first_digit);
//! assert_eq!(10, *total.current());
//! ```
use std::cell::RefCell;

use num_traits::{WrappingAdd, WrappingSub, Zero};

use crate::signal::{ReadSignal, Signal, SignalReceiver, WriteSignal};

/// This holds the current total of a sum
///
/// See module level documentation for an example of usage.
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

/// A single element of the sum
///
/// See module level documentation for an example of usage.
pub struct SumElement<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> {
    current: RefCell<T>,
    total: SumTotal<T>,
}

impl<T: 'static + Zero + Clone + Zero + WrappingAdd + WrappingSub> SumElement<T> {
    pub fn new(total: &SumTotal<T>) -> Self {
        Self {
            current: RefCell::new(T::zero()),
            total: total.clone(),
        }
    }
}

impl<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> SignalReceiver<T, SumHandle>
    for SumElement<T>
{
    fn receive(&self, x: &T) -> SumHandle {
        let delta = x.wrapping_sub(&self.current.borrow());
        self.current.replace(x.clone());
        self.total.deltas.set(delta);
        SumHandle()
    }
}

impl<T: 'static + Clone + Zero + WrappingAdd + WrappingSub> Drop for SumElement<T> {
    fn drop(&mut self) {
        self.total
            .deltas
            .set(T::zero().wrapping_sub(&self.current.borrow()));
    }
}

/// A handle to keep a value in the sum
///
/// See module level documentation for an example of usage.
pub struct SumHandle();

struct AccumulateSum<T>(RefCell<T>);

impl<T: 'static + Clone + WrappingAdd> SignalReceiver<T, T> for AccumulateSum<T> {
    fn receive(&self, x: &T) -> T {
        let mut total = self.0.borrow_mut();
        *total = total.wrapping_add(x);
        total.clone()
    }
}
