use silkenweb::{
    node::{element::ElementBuilderBase, Node},
    prelude::HtmlElement,
    ElementBuilder,
};

pub mod badge;
pub mod utility;

pub mod css {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-5.2.2/css/bootstrap.min.css");
}

pub mod icon {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons-1.9.1/bootstrap-icons.css");
}

pub type Class = &'static str;

/// A generic HTML element builder
///
/// Some bootstrap types, like [`Badge`](badge::Badge) will convert into this as
/// an "escape hatch".
#[derive(ElementBuilder)]
pub struct HtmlElementBuilder(ElementBuilderBase);

impl HtmlElement for HtmlElementBuilder {}

impl From<HtmlElementBuilder> for Node {
    fn from(builder: HtmlElementBuilder) -> Self {
        builder.0.into()
    }
}
