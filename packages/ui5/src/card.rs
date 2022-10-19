mod elements {
    use silkenweb::{custom_html_element, parent_element};

    custom_html_element!(
        ui5_card = {
            dom_type: web_sys::Element;
            attributes {
                accessible_name: String,
                accessible_name_ref: String,
            };
        }
    );

    parent_element!(ui5_card);

    custom_html_element!(
        ui5_card_header = {
            dom_type: web_sys::Element;
            attributes {
                interactive: bool,
                status: String,
                subtitle_text: String,
                title_text: String,
            };

            custom_events {
                click: web_sys::CustomEvent,
            };
        }
    );

    parent_element!(ui5_card_header);
}

pub use elements::{Ui5Card as Card, Ui5CardHeader as CardHeader};
use silkenweb::ElementBuilder;

use self::elements::{ui5_card_header, Ui5CardHeaderBuilder};
use crate::macros::attributes0;

pub fn card_header_builder() -> CardHeaderBuilder {
    CardHeaderBuilder(ui5_card_header())
}

#[derive(ElementBuilder)]
pub struct CardHeaderBuilder(Ui5CardHeaderBuilder);

impl CardHeaderBuilder {
    attributes0! {
        interactive: bool,
        status: String,
        subtitle_text: String,
        title_text: String,
    }

    pub fn on_click(self, f: impl FnMut(web_sys::CustomEvent, web_sys::Element) + 'static) -> Self {
        Self(self.0.on_click(f))
    }
}
