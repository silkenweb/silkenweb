use silkenweb::{
    attribute::AsAttribute,
    elements::{
        html::{div, Div},
        AriaElement,
    },
    node::{
        element::{GenericElement, Element},
        Node,
    },
    prelude::ParentElement,
    value::{RefSignalOrValue, SignalOrValue},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{button::Button, css, dropdown::Dropdown};

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct ButtonGroup(Div);

pub fn button_group<'a>(
    name: impl RefSignalOrValue<'a, Item = impl AsAttribute<String>>,
) -> ButtonGroup {
    ButtonGroup(div().role("group").aria_label(name).class(css::BTN_GROUP))
}

impl ButtonGroup {
    pub fn button(self, elem: impl SignalOrValue<Item = Button>) -> Self {
        Self(self.0.child(elem))
    }

    pub fn dropdown(self, elem: impl SignalOrValue<Item = Dropdown>) -> Self {
        Self(self.0.child(elem.map(|elem| elem.class(css::BTN_GROUP))))
    }
}

impl From<ButtonGroup> for GenericElement {
    fn from(elem: ButtonGroup) -> Self {
        elem.0.into()
    }
}

impl From<ButtonGroup> for Node {
    fn from(elem: ButtonGroup) -> Self {
        elem.0.into()
    }
}
