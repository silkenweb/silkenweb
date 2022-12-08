use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::{element::{ParentElement, GenericElement}, Node},
    prelude::HtmlElement,
    value::{SignalOrValue, Value},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use self::element::{ui5_bar, Ui5Bar};
use crate::macros::attributes0;

#[derive(Copy, Clone, Eq, PartialEq, Display, Value)]
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
    use silkenweb::{custom_html_element, parent_element};

    use super::BarDesign;

    custom_html_element!(
        ui5_bar = {
            dom_type: web_sys::HtmlElement;
            attributes { design: BarDesign };
        }
    );

    parent_element!(ui5_bar);
}

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct Bar(Ui5Bar);

pub fn bar() -> Bar {
    Bar(ui5_bar())
}

impl Bar {
    attributes0! {design: BarDesign}

    pub fn start_content(
        self,
        child: impl SignalOrValue<Item = impl HtmlElement + Into<GenericElement> + Value> + 'static,
    ) -> Self {
        Self(
            self.0
                .child(child.map(|child| child.slot("startContent").into())),
        )
    }

    pub fn middle_content(
        self,
        child: impl SignalOrValue<Item = impl Value + Into<Node> + 'static> + 'static,
    ) -> Self {
        Self(self.0.child(child))
    }

    pub fn end_content(
        self,
        child: impl SignalOrValue<Item = impl HtmlElement + Into<GenericElement>> + 'static,
    ) -> Self {
        Self(
            self.0
                .child(child.map(|child| child.slot("endContent").into())),
        )
    }
}

impl From<Bar> for GenericElement {
    fn from(elem: Bar) -> Self {
        elem.0.into()
    }
}

