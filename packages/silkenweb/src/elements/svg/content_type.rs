//! SVG content types.
//!
//! See [MDN SVG Content Types](https://developer.mozilla.org/en-US/docs/Web/SVG/Content_type)

// TODO: Docs
use std::{borrow::Cow, fmt::Display, marker::PhantomData};

use num_traits::Num;

use crate::attribute::{AsAttribute, Attribute};

pub trait Number: Num + Display + Copy {}

impl<T: Num + Display + Copy> Number for T {}

macro_rules! length{
    ($($name: ident),* $(,)?) =>{
        $(
            pub fn $name<N: Number>(value: N) -> Quantity<N, Length> {
                Quantity::new(value, stringify!($name))
            }
        )*
    }
}

length!(em, ex, px, cm, mm, pt, pc);

pub fn inches<N: Number>(value: N) -> Quantity<N, Length> {
    Quantity::new(value, "in")
}

pub struct Length;

impl Length {
    pub fn zero() -> Quantity<u32, Length> {
        Quantity::new(0, "")
    }
}

impl<N: Number> AsAttribute<Length> for Quantity<N, Length> {}
impl<N: Number> AsAttribute<LengthOrPercentage> for Quantity<N, Length> {}
impl<N: Number> AsAttribute<AutoOrLengthOrPercentage> for Quantity<N, Length> {}

pub fn percentage<N: Number>(value: N) -> Quantity<N, Percentage> {
    Quantity::new(value, "%")
}

pub struct Percentage;

impl<N: Number> AsAttribute<Percentage> for Quantity<N, Percentage> {}
impl<N: Number> AsAttribute<LengthOrPercentage> for Quantity<N, Percentage> {}
impl<N: Number> AsAttribute<AutoOrLengthOrPercentage> for Quantity<N, Percentage> {}

pub struct Auto;

impl Attribute for Auto {
    fn text(&self) -> Option<Cow<str>> {
        Some("auto".into())
    }
}

impl AsAttribute<AutoOrLengthOrPercentage> for Auto {}

pub struct LengthOrPercentage;
pub struct AutoOrLengthOrPercentage;

pub struct Quantity<N, T> {
    value: N,
    units: &'static str,
    ty: PhantomData<T>,
}

impl<N, T> Quantity<N, T> {
    pub fn new(value: N, units: &'static str) -> Self {
        Self {
            value,
            units,
            ty: PhantomData,
        }
    }
}

impl<N: Number, T> Attribute for Quantity<N, T> {
    fn text(&self) -> Option<Cow<str>> {
        Some(format!("{}{}", self.value, self.units).into())
    }
}
