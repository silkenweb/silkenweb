use derive_more::Into;
use silkenweb::{
    elements::html,
    node::{
        element::{GenericElement, Element},
        Node,
    },
    prelude::ParentElement,
    value::{RefSignalOrValue, SignalOrValue, Value},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, icon::Icon, utility::Colour, Class, HtmlElementBuilder};

#[derive(
    Value, Into, Element, HtmlElement, ElementEvents, HtmlElementEvents, AriaElement,
)]
pub struct Button(HtmlElementBuilder);

pub fn button<'a>(
    button_type: &str,
    text: impl RefSignalOrValue<'a, Item = impl AsRef<str> + Into<String> + 'a>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> Button {
    Button(
        HtmlElementBuilder(html::button().r#type(button_type).class(css::BTN).into())
            .class(style.map(ButtonStyle::class)),
    )
    .text(text)
}

pub fn icon_button(
    button_type: &str,
    icon: impl SignalOrValue<Item = impl Value + Into<Icon> + 'static>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> Button {
    Button(
        HtmlElementBuilder(html::button().r#type(button_type).class(css::BTN).into())
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

impl Button {
    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> Button {
        Button(HtmlElementBuilder(self.0 .0.text(text)))
    }

    pub fn icon(
        self,
        icon: impl SignalOrValue<Item = impl Value + Into<Icon> + 'static>,
    ) -> Button {
        Button(HtmlElementBuilder(
            self.0 .0.child(icon.map(|icon| icon.into())),
        ))
    }
}

impl From<Button> for GenericElement {
    fn from(elem: Button) -> Self {
        elem.0.into()
    }
}

impl From<Button> for Node {
    fn from(elem: Button) -> Self {
        elem.0.into()
    }
}
