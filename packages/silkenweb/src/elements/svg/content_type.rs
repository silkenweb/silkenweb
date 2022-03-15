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
            pub fn $name<N: Number>(value: N) -> WithUnits<N, Length> {
                WithUnits::new(value, stringify!($name))
            }
        )*
    }
}

length!(em, ex, px, cm, mm, pt, pc);

pub fn inches<N: Number>(value: N) -> WithUnits<N, Length> {
    WithUnits::new(value, "in")
}

pub struct Length;

impl Length {
    pub fn zero() -> WithUnits<u32, Length> {
        WithUnits::new(0, "")
    }
}

impl<N: Number> AsAttribute<Length> for WithUnits<N, Length> {}
impl<N: Number> AsAttribute<LengthOrPercentage> for WithUnits<N, Length> {}
impl<N: Number> AsAttribute<AutoOrLengthOrPercentage> for WithUnits<N, Length> {}

pub fn percentage<N: Number>(value: N) -> WithUnits<N, Percentage> {
    WithUnits::new(value, "%")
}

pub struct Percentage;

impl<N: Number> AsAttribute<Percentage> for WithUnits<N, Percentage> {}
impl<N: Number> AsAttribute<LengthOrPercentage> for WithUnits<N, Percentage> {}
impl<N: Number> AsAttribute<AutoOrLengthOrPercentage> for WithUnits<N, Percentage> {}

pub struct Auto;

impl Attribute for Auto {
    fn text(&self) -> Option<Cow<str>> {
        Some("auto".into())
    }
}

impl AsAttribute<AutoOrLengthOrPercentage> for Auto {}

pub struct LengthOrPercentage;
pub struct AutoOrLengthOrPercentage;

pub struct WithUnits<N, T> {
    value: N,
    units: &'static str,
    ty: PhantomData<T>,
}

impl<N, T> WithUnits<N, T> {
    pub fn new(value: N, units: &'static str) -> Self {
        Self {
            value,
            units,
            ty: PhantomData,
        }
    }
}

impl<N: Number, T> Attribute for WithUnits<N, T> {
    fn text(&self) -> Option<Cow<str>> {
        Some(format!("{}{}", self.value, self.units).into())
    }
}
