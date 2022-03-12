use futures_signals::signal::Signal;
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use self::element::{ui5_busy_indicator, Ui5BusyIndicator, Ui5BusyIndicatorBuilder};

#[derive(Copy, Clone, Display, Eq, PartialEq)]
pub enum BusyIndicatorSize {
    Small,
    Medium,
    Large,
}

impl Attribute for BusyIndicatorSize {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<BusyIndicatorSize> for BusyIndicatorSize {}

mod element {
    use silkenweb::{html_element, parent_element};

    use super::BusyIndicatorSize;

    html_element!(
        ui5-busy-indicator<web_sys::HtmlElement> {
            attributes {
                active: bool,
                delay: u64,
                size: BusyIndicatorSize,
                text: String,
            }
        }
    );

    parent_element!(ui5 - busy - indicator);
}

pub fn busy_indicator() -> BusyIndicatorBuilder {
    BusyIndicatorBuilder(ui5_busy_indicator())
}

pub type BusyIndicator = Ui5BusyIndicator;

#[derive(ElementBuilder)]
pub struct BusyIndicatorBuilder(Ui5BusyIndicatorBuilder);

impl BusyIndicatorBuilder {
    pub fn active(self) -> Self {
        Self(self.0.active(true))
    }

    pub fn active_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.active_signal(value))
    }

    pub fn delay(self, value: u64) -> Self {
        Self(self.0.delay(value))
    }

    pub fn delay_signal(self, value: impl Signal<Item = u64> + 'static) -> Self {
        Self(self.0.delay_signal(value))
    }

    pub fn size(self, value: BusyIndicatorSize) -> Self {
        Self(self.0.size(value))
    }

    pub fn size_signal(self, value: impl Signal<Item = BusyIndicatorSize> + 'static) -> Self {
        Self(self.0.size_signal(value))
    }

    pub fn text(self, value: String) -> Self {
        Self(self.0.text(value))
    }

    pub fn text_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.text_signal(value))
    }

    pub fn child(self, child: impl Into<Node>) -> BusyIndicator {
        self.0.children([child]).build()
    }

    pub fn child_signal(
        self,
        child: impl Signal<Item = impl Into<Node>> + 'static,
    ) -> BusyIndicator {
        self.0.child_signal(child)
    }
}

impl HtmlElement for BusyIndicatorBuilder {}

impl HtmlElementEvents for BusyIndicatorBuilder {}

impl ElementEvents for BusyIndicatorBuilder {}
