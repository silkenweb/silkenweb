use silkenweb::{
    custom_html_element, element_slot_single, elements::CustomEvent, parent_element, StrAttribute,
    Value,
};
use strum::AsRefStr;

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
element_slot_single!(ui5_button, badge, "badge", Ui5ButtonBadge);

custom_html_element!(
    ui5_button_badge = {
        dom_type: web_sys::HtmlElement;
        attributes {
            text: String,
        };

        events {
            click: CustomEvent<web_sys::HtmlElement>,
        };
    }
);

#[derive(Copy, Clone, Eq, PartialEq, AsRefStr, StrAttribute, Value)]
pub enum ButtonDesign {
    Default,
    Emphasized,
    Positive,
    Negative,
    Transparent,
    Attention,
}
