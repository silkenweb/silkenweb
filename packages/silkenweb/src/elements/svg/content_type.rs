//! SVG content types.
//!
//! See [MDN SVG Content Types](https://developer.mozilla.org/en-US/docs/Web/SVG/Content_type)

use silkenweb_signals_ext::value::Value;

use crate::attribute::{AsAttribute, Attribute};

/// An SVG length with units. For percentages, use [`Percentage`] and for
/// unitless lengths, use `f64`.
pub enum Length {
    Em(f64),
    Ex(f64),
    Px(f64),
    Cm(f64),
    Mm(f64),
    In(f64),
    Pt(f64),
    Pc(f64),
}

impl Attribute for Length {
    type Text<'a> = String;

    fn text(&self) -> Option<Self::Text<'_>> {
        let (length, units) = match self {
            Length::Em(l) => (l, "em"),
            Length::Ex(l) => (l, "ex"),
            Length::Px(l) => (l, "px"),
            Length::Cm(l) => (l, "cm"),
            Length::Mm(l) => (l, "mm"),
            Length::In(l) => (l, "in"),
            Length::Pt(l) => (l, "pt"),
            Length::Pc(l) => (l, "pc"),
        };

        Some(format!("{length}{units}"))
    }
}

impl Value for Length {}

impl AsAttribute<Length> for Length {}
impl AsAttribute<Length> for Percentage {}
impl AsAttribute<Length> for f64 {}

/// SVG attribute type for percentages
pub struct Percentage(pub f64);

impl Attribute for Percentage {
    type Text<'a> = String;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(format!("{}%", self.0))
    }
}

impl AsAttribute<Percentage> for Percentage {}
impl Value for Percentage {}

/// For SVG attributes that accept "auto"
pub struct Auto;

impl Attribute for Auto {
    type Text<'a> = &'static str;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some("auto")
    }
}

impl AsAttribute<Auto> for Auto {}

impl Value for Auto {}

/// Marker type for SVG attributes that can be a number or percentage
pub struct NumberOrPercentage;

impl AsAttribute<NumberOrPercentage> for f64 {}
impl AsAttribute<NumberOrPercentage> for Percentage {}

/// Marker type for SVG attributes that can be "auto" or a length
pub struct AutoOrLength;

impl AsAttribute<AutoOrLength> for Auto {}
impl AsAttribute<AutoOrLength> for f64 {}
impl AsAttribute<AutoOrLength> for Length {}
impl AsAttribute<AutoOrLength> for Percentage {}
