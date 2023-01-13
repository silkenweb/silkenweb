use silkenweb::{
    dom::Dom,
    elements::html::{div, Div, Ol, Ul},
    node::{element::GenericElement, Node},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};
use utility::SetDisplay;

pub mod badge;
pub mod button;
pub mod button_group;
pub mod dropdown;
pub mod icon;
pub mod tab_bar;
pub mod utility;

pub mod css {
    silkenweb::css!("bootstrap-5.2.2/css/bootstrap.min.css");

    pub use class::*;
}

pub type Class = &'static str;

/// A generic HTML element
///
/// Bootstrap elem types that don't implement `HtmlElement`, like
/// [`Badge`](badge::Badge) will convert into this as an "escape
/// hatch".
#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct GenericHtmlElement<D: Dom>(GenericElement<D>);

impl<D: Dom> From<GenericHtmlElement<D>> for GenericElement<D> {
    fn from(elem: GenericHtmlElement<D>) -> Self {
        elem.0
    }
}

impl<D: Dom> From<GenericHtmlElement<D>> for Node<D> {
    fn from(elem: GenericHtmlElement<D>) -> Self {
        elem.0.into()
    }
}

/// Shorthand for `div().flex_row()`
pub fn row<D: Dom>() -> Div<D> {
    div().flex_row()
}

/// Shorthand for `div().flex_column()`
pub fn column<D: Dom>() -> Div<D> {
    div().flex_column()
}

/// Marker trait for lists
pub trait List {}

impl List for Ol {}
impl List for Ul {}
