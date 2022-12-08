use silkenweb::{
    elements::html::{div, Div, Ol, Ul},
    node::{element::GenericElement, Node},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
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
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-5.2.2/css/bootstrap.min.css");
}

pub type Class = &'static str;

// TODO: Rename to GenericHtmlElement
/// A generic HTML element
///
/// Bootstrap elem types that don't implement `HtmlElement`, like
/// [`Badge`](badge::Badge) will convert into this as an "escape
/// hatch".
#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct HtmlElementBuilder(GenericElement);

impl From<HtmlElementBuilder> for GenericElement {
    fn from(elem: HtmlElementBuilder) -> Self {
        elem.0
    }
}

impl From<HtmlElementBuilder> for Node {
    fn from(elem: HtmlElementBuilder) -> Self {
        elem.0.into()
    }
}

/// Shorthand for `div().flex_row()`
pub fn row() -> Div {
    div().flex_row()
}

/// Shorthand for `div().flex_column()`
pub fn column() -> Div {
    div().flex_column()
}

/// Marker trait for lists
pub trait List {}

impl List for Ol {}
impl List for Ul {}
