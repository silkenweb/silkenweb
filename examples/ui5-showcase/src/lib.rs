use std::borrow::Cow;

use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    custom_html_element, parent_element, Value,
};

#[derive(Copy, Clone, Eq, PartialEq, Display, Value)]
pub enum ButtonDesign {
    Default,
    Emphasized,
    Positive,
    Negative,
    Transparent,
    Attention,
}

impl Attribute for ButtonDesign {
    fn text(&self) -> Option<Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<ButtonDesign> for ButtonDesign {}

custom_html_element!(
    ui5_button = {
        dom_type: web_sys::HtmlElement;
        attributes {
            accessible_name: String,
            accessible_name_ref: String,
            design: ButtonDesign,
            disabled: bool,
            icon_end: bool,
            submits: bool,
            tooltop: String,
        };

        custom_events {
            click: web_sys::CustomEvent,
        };
    }
);

parent_element!(ui5_button);
