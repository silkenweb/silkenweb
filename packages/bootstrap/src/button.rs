use derive_more::Into;
use silkenweb::{
    dom::{DefaultDom, Dom},
    elements::html,
    node::{
        element::{Element, GenericElement},
        Node,
    },
    prelude::ParentElement,
    value::{RefSignalOrValue, SignalOrValue, Value},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, icon::Icon, utility::Colour, Class, GenericHtmlElement};

#[derive(Value, Into, Element, HtmlElement, ElementEvents, HtmlElementEvents, AriaElement)]
pub struct Button<D: Dom = DefaultDom>(GenericHtmlElement<D>);

pub fn button<'a, D: Dom>(
    button_type: &str,
    text: impl RefSignalOrValue<'a, Item = impl AsRef<str> + Into<String> + 'a>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> Button<D> {
    Button(
        GenericHtmlElement(html::button().r#type(button_type).class(css::BTN).into())
            .class(style.map(ButtonStyle::class)),
    )
    .text(text)
}

pub fn icon_button<D: Dom>(
    button_type: &str,
    icon: impl SignalOrValue<Item = impl Value + Into<Icon<D>> + 'static>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> Button<D> {
    Button(
        GenericHtmlElement(html::button().r#type(button_type).class(css::BTN).into())
            .class(style.map(ButtonStyle::class)),
    )
    .icon(icon)
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum ButtonStyle {
    Link,
    Solid(Colour),
    Outline(Colour),
}

impl ButtonStyle {
    fn class(self) -> Class {
        match self {
            ButtonStyle::Link => css::BTN_LINK,
            ButtonStyle::Solid(colour) => colour.button(false),
            ButtonStyle::Outline(colour) => colour.button(true),
        }
    }
}

impl<D: Dom> Button<D> {
    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> Self {
        Button(GenericHtmlElement(self.0 .0.text(text)))
    }

    pub fn icon(
        self,
        icon: impl SignalOrValue<Item = impl Value + Into<Icon<D>> + 'static>,
    ) -> Self {
        Button(GenericHtmlElement(
            self.0 .0.child(icon.map(|icon| icon.into())),
        ))
    }
}

impl<D: Dom> From<Button<D>> for GenericElement<D> {
    fn from(elem: Button<D>) -> Self {
        elem.0.into()
    }
}

impl<D: Dom> From<Button<D>> for Node<D> {
    fn from(elem: Button<D>) -> Self {
        elem.0.into()
    }
}
