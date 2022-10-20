use derive_more::Into;
use silkenweb::{
    attribute::AsAttribute,
    elements::{
        html::{div, DivBuilder},
        AriaElement,
    },
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::ParentBuilder,
    value::{RefSignalOrValue, SignalOrValue},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{button::ButtonBuilder, css, dropdown::DropdownBuilder};

#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
#[element_target(ButtonGroup)]
pub struct ButtonGroupBuilder(DivBuilder);

pub fn button_group<'a>(
    name: impl RefSignalOrValue<'a, Item = impl AsAttribute<String>>,
) -> ButtonGroupBuilder {
    ButtonGroupBuilder(div().role("group").aria_label(name).class(css::BTN_GROUP))
}

impl ButtonGroupBuilder {
    pub fn button(self, elem: impl SignalOrValue<Item = ButtonBuilder>) -> Self {
        Self(self.0.child(elem))
    }

    pub fn dropdown(self, elem: impl SignalOrValue<Item = DropdownBuilder>) -> Self {
        Self(self.0.child(elem.map(|elem| elem.class(css::BTN_GROUP))))
    }
}

impl From<ButtonGroupBuilder> for Element {
    fn from(elem: ButtonGroupBuilder) -> Self {
        elem.0.into()
    }
}

impl From<ButtonGroupBuilder> for Node {
    fn from(elem: ButtonGroupBuilder) -> Self {
        elem.0.into()
    }
}

#[derive(Into, Value)]
pub struct ButtonGroup(Element);

impl From<ButtonGroup> for Node {
    fn from(elem: ButtonGroup) -> Self {
        elem.0.into()
    }
}
