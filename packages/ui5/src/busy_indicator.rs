use parse_display::Display;
use silkenweb::attribute::{AsAttribute, Attribute};

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
        ui5_busy_indicator = {
            dom_type: web_sys::HtmlElement;
            attributes {
                active: bool,
                delay: u64,
                size: BusyIndicatorSize,
                text: String,
            }
        }
    );

    parent_element!(ui5_busy_indicator);
}

pub use element::{
    ui5_busy_indicator as busy_indicator, Ui5BusyIndicator as BusyIndicator,
    Ui5BusyIndicatorBuilder as BusyIndicatorBuilder,
};
