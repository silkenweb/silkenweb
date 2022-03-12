mod elements {
    use silkenweb::{html_element, parent_element};

    html_element!(
        ui5-card<web_sys::Element> {
            attributes {
                accessible-name: String,
                accessible-name-ref: String,
            }
        }
    );

    parent_element!(ui5 - card);

    html_element!(
        ui5-card-header<web_sys::Element> {
            attributes {
                interactive: bool,
                status: String,
                subtitle-text: String,
                title-text: String,
            }

            custom_events {
                click: web_sys::CustomEvent,
            }
        }
    );

    parent_element!(ui5 - card - header);
}

pub use elements::{Ui5Card as Card, Ui5CardHeader as CardHeader};
use futures_signals::signal::Signal;
use silkenweb::{
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    ElementBuilder,
};

use self::elements::{ui5_card_header, Ui5CardHeaderBuilder};

pub fn card_header_builder() -> CardHeaderBuilder {
    CardHeaderBuilder(ui5_card_header())
}

#[derive(ElementBuilder)]
pub struct CardHeaderBuilder(Ui5CardHeaderBuilder);

impl CardHeaderBuilder {
    pub fn interactive(self, value: bool) -> Self {
        Self(self.0.interactive(value))
    }

    pub fn interactive_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.interactive_signal(value))
    }

    pub fn status(self, value: &str) -> Self {
        Self(self.0.status(value))
    }

    pub fn status_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.status_signal(value))
    }

    pub fn subtitle_text(self, value: &str) -> Self {
        Self(self.0.subtitle_text(value))
    }

    pub fn subtitle_text_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.subtitle_text_signal(value))
    }

    pub fn title_text(self, value: &str) -> Self {
        Self(self.0.title_text(value))
    }

    pub fn title_text_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.title_text_signal(value))
    }

    pub fn on_click(self, f: impl FnMut(web_sys::CustomEvent, web_sys::Element) + 'static) -> Self {
        Self(self.0.on_click(f))
    }
}

impl HtmlElement for CardHeaderBuilder {}

impl HtmlElementEvents for CardHeaderBuilder {}

impl ElementEvents for CardHeaderBuilder {}
