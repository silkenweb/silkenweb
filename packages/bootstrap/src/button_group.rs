use silkenweb::{
    attribute::AsAttribute,
    dom::{DefaultDom, Dom},
    elements::{
        html::{div, Div},
        AriaElement,
    },
    node::{
        element::{Element, GenericElement},
        Node,
    },
    prelude::ParentElement,
    value::{RefSignalOrValue, SignalOrValue},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{button::Button, css, dropdown::Dropdown};

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct ButtonGroup<D: Dom = DefaultDom>(Div<D>);

pub fn button_group<'a, D: Dom>(
    name: impl RefSignalOrValue<'a, Item = impl AsAttribute<String>>,
) -> ButtonGroup<D> {
    ButtonGroup(div().role("group").aria_label(name).class(css::BTN_GROUP))
}

impl<D: Dom> ButtonGroup<D> {
    pub fn button(self, elem: impl SignalOrValue<Item = Button<D>>) -> Self {
        Self(self.0.child(elem))
    }

    pub fn dropdown(self, elem: impl SignalOrValue<Item = Dropdown<D>>) -> Self {
        Self(self.0.child(elem.map(|elem| elem.class(css::BTN_GROUP))))
    }
}

impl<D: Dom> From<ButtonGroup<D>> for GenericElement<D> {
    fn from(elem: ButtonGroup<D>) -> Self {
        elem.0.into()
    }
}

impl<D: Dom> From<ButtonGroup<D>> for Node<D> {
    fn from(elem: ButtonGroup<D>) -> Self {
        elem.0.into()
    }
}
