use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    node::element::ParentBuilder,
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    ElementBuilder,
};

use self::elements::ui5_badge;
use crate::icon::Ui5Icon;

mod elements {
    use silkenweb::{html_element, parent_element};

    html_element!(
        ui5-badge<web_sys::HtmlElement> {
            attributes {
                color-scheme: u8
            }
        }
    );

    parent_element!(ui5 - badge);
}

pub type Badge = elements::Ui5Badge;

pub fn badge() -> BadgeBuilder {
    BadgeBuilder(ui5_badge())
}

#[derive(ElementBuilder)]
pub struct BadgeBuilder(elements::Ui5BadgeBuilder);

impl BadgeBuilder {
    pub fn color_scheme(self, value: u8) -> Self {
        Self(self.0.color_scheme(value))
    }

    pub fn color_scheme_signal(self, value: impl Signal<Item = u8> + 'static) -> Self {
        Self(self.0.color_scheme_signal(value))
    }

    pub fn icon(self, icon: impl Into<Ui5Icon>) -> Self {
        Self(self.0.child(icon.into()))
    }

    pub fn icon_signal(self, icon: impl Signal<Item = impl Into<Ui5Icon>> + 'static) -> Self {
        Self(self.0.child_signal(icon.map(|img| img.into())))
    }

    pub fn text(self, text: &str) -> BadgeBuilder {
        Self(self.0.text(text))
    }

    pub fn text_signal(self, text: impl Signal<Item = String> + 'static) -> BadgeBuilder {
        Self(self.0.text_signal(text))
    }
}

impl HtmlElement for BadgeBuilder {}

impl HtmlElementEvents for BadgeBuilder {}

impl ElementEvents for BadgeBuilder {}
