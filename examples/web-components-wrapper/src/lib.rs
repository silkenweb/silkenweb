use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    custom_html_element,
    elements::CustomEvent,
    parent_element, Value,
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
    type Text<'a> = String;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(self.to_string())
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

        events {
            click: CustomEvent<web_sys::HtmlElement>,
        };
    }
);

parent_element!(ui5_button);
