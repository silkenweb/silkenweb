use silkenweb::{
    node::element::ParentBuilder,
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    value::{RefSignalOrValue, SignalOrValue},
    ElementBuilder,
};

use self::elements::ui5_badge;
use crate::{icon::Ui5Icon, macros::attributes0};

mod elements {
    use silkenweb::{custom_html_element, parent_element};

    custom_html_element!(
        ui5_badge = {
            dom_type: web_sys::HtmlElement;
            attributes { color_scheme: u8 };
        }
    );

    parent_element!(ui5_badge);
}

pub type Badge = elements::Ui5Badge;

pub fn badge() -> BadgeBuilder {
    BadgeBuilder(ui5_badge())
}

#[derive(ElementBuilder)]
pub struct BadgeBuilder(elements::Ui5BadgeBuilder);

impl BadgeBuilder {
    attributes0! {color_scheme: u8}

    pub fn icon(self, icon: impl SignalOrValue<Item = Ui5Icon>) -> Self {
        Self(self.0.child(icon))
    }

    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> BadgeBuilder {
        Self(self.0.text(text))
    }
}

impl HtmlElement for BadgeBuilder {}

impl HtmlElementEvents for BadgeBuilder {}

impl ElementEvents for BadgeBuilder {}
