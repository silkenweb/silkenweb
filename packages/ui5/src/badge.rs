use silkenweb::{
    node::element::{ParentElement, GenericElement},
    value::{RefSignalOrValue, SignalOrValue},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
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

pub fn badge() -> Badge {
    Badge(ui5_badge())
}

#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct Badge(elements::Ui5Badge);

impl Badge {
    attributes0! {color_scheme: u8}

    pub fn icon(self, icon: impl SignalOrValue<Item = Ui5Icon>) -> Self {
        Self(self.0.child(icon))
    }

    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> Badge {
        Self(self.0.text(text))
    }
}

impl From<Badge> for GenericElement {
    fn from(elem: Badge) -> Self {
        elem.0.into()
    }
}
