use futures_signals::signal::{Signal, SignalExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::{
        element::{Element, ParentBuilder},
        Node,
    },
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    ElementBuilder,
};

use self::element::{ui5_bar, Ui5BarBuilder};
use crate::macros::attributes0;

#[derive(Copy, Clone, Eq, PartialEq, Display)]
pub enum BarDesign {
    Header,
    Subheader,
    Footer,
    FloatingFooter,
}

impl Attribute for BarDesign {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<BarDesign> for BarDesign {}

mod element {
    use silkenweb::{html_element, parent_element};

    use super::BarDesign;

    html_element!(
        ui5_bar = { dom_type: web_sys::HtmlElement;
            attributes {
                design: BarDesign
            }
        }
    );

    parent_element!(ui5_bar);
}

pub use element::Ui5Bar as Bar;

#[derive(ElementBuilder)]
pub struct BarBuilder(Ui5BarBuilder);

impl HtmlElement for BarBuilder {}

impl HtmlElementEvents for BarBuilder {}

impl ElementEvents for BarBuilder {}

pub fn bar() -> BarBuilder {
    BarBuilder(ui5_bar())
}

impl BarBuilder {
    attributes0! {design: BarDesign}

    pub fn start_content(self, child: impl HtmlElement + Into<Element>) -> Self {
        Self(self.0.child(child.slot("startContent").into()))
    }

    pub fn start_content_signal(
        self,
        child: impl Signal<Item = impl HtmlElement + Into<Element>> + 'static,
    ) -> Self {
        Self(
            self.0
                .child_signal(child.map(|child| child.slot("startContent").into())),
        )
    }

    pub fn middle_content(self, child: impl Into<Node>) -> Self {
        Self(self.0.child(child))
    }

    pub fn middle_content_signal(
        self,
        child: impl Signal<Item = impl Into<Node>> + 'static,
    ) -> Self {
        Self(self.0.child_signal(child))
    }

    pub fn end_content(self, child: impl HtmlElement + Into<Element>) -> Self {
        Self(self.0.child(child.slot("endContent").into()))
    }

    pub fn end_content_signal(
        self,
        child: impl Signal<Item = impl HtmlElement + Into<Element>> + 'static,
    ) -> Self {
        Self(
            self.0
                .child_signal(child.map(|child| child.slot("endContent").into())),
        )
    }
}
