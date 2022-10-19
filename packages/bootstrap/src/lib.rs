use silkenweb::{
    elements::html::{div, DivBuilder},
    node::{element::ElementBuilderBase, Node},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};
use utility::SetFlex;

pub mod badge;
pub mod button;
pub mod button_group;
pub mod dropdown;
pub mod icon;
pub mod utility;

pub mod css {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-5.2.2/css/bootstrap.min.css");
}

pub type Class = &'static str;

// TODO: Does anything need to use this?
/// A generic HTML element builder
///
/// Some bootstrap types, like [`Badge`](badge::Badge) will convert into this as
/// an "escape hatch".
#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct HtmlElementBuilder(ElementBuilderBase);

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
