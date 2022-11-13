use silkenweb::{
    elements::html::{div, DivBuilder, Ol, Ul},
    node::{
        element::{Element, ElementBuilderBase},
        Node,
    },
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

/// A generic HTML element builder
///
/// Bootstrap builder types that don't implement `HtmlElement`, like
/// [`BadgeBuilder`](badge::BadgeBuilder) will convert into this as an "escape
/// hatch".
#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct HtmlElementBuilder(ElementBuilderBase);

impl From<HtmlElementBuilder> for Element {
    fn from(builder: HtmlElementBuilder) -> Self {
        builder.0.into()
    }
}

impl From<HtmlElementBuilder> for Node {
    fn from(builder: HtmlElementBuilder) -> Self {
        builder.0.into()
    }
}

/// Shorthand for `div().flex_row()`
pub fn row() -> DivBuilder {
    div().flex_row()
}

/// Shorthand for `div().flex_column()`
pub fn column() -> DivBuilder {
    div().flex_column()
}

/// Marker trait for lists
pub trait List {}

impl List for Ol {}
impl List for Ul {}
