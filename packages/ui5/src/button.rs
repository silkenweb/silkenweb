use parse_display::Display;
use silkenweb::attribute::{AsAttribute, Attribute};

#[derive(Copy, Clone, Eq, PartialEq, Display)]
pub enum ButtonDesign {
    Default,
    Emphasized,
    Positive,
    Negative,
    Transparent,
    Attention,
}

impl Attribute for ButtonDesign {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<ButtonDesign> for ButtonDesign {}

mod element {
    use silkenweb::{html_element, parent_element};

    use super::ButtonDesign;
    use crate::icon::Icon;

    html_element!(
        ui5-button<web_sys::HtmlElement> {
            attributes {
                accessible-name: String,
                accessible-name-ref: String,
                design: ButtonDesign,
                disabled:bool,
                icon : Icon,
                icon-end: bool,
                submits: bool,
                tooltop: String,
            }

            custom_events {
                click: web_sys::CustomEvent,
            }
        }
    );

    parent_element!(ui5 - button);
}

pub use element::{ui5_button as button, Ui5Button as Button, Ui5ButtonBuilder as ButtonBuilder};
