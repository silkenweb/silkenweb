use derive_more::Into;
use silkenweb::{
    attribute::AsAttribute,
    elements::{
        html::{div, DivBuilder},
        AriaElement,
    },
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    value::{RefSignalOrValue, SignalOrValue, Value},
    ElementBuilder,
};

use crate::{button::ButtonBuilder, css, dropdown::DropdownBuilder};

#[derive(ElementBuilder)]
#[element_target(ButtonGroup)]
pub struct ButtonGroupBuilder(DivBuilder);

// TODO: Checkbox and radio (how do we see what's checked), active flag,
// toolbars
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

impl Value for ButtonGroupBuilder {}
impl HtmlElement for ButtonGroupBuilder {}
impl HtmlElementEvents for ButtonGroupBuilder {}
impl ElementEvents for ButtonGroupBuilder {}
impl AriaElement for ButtonGroupBuilder {}

impl From<ButtonGroupBuilder> for Node {
    fn from(elem: ButtonGroupBuilder) -> Self {
        elem.0.into()
    }
}

#[derive(Into)]
pub struct ButtonGroup(Node);

impl Value for ButtonGroup {}
