use derive_more::Into;
use silkenweb::{
    elements::html,
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::ParentBuilder,
    value::{RefSignalOrValue, SignalOrValue, Value},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, icon::Icon, utility::Colour, Class, HtmlElementBuilder};

#[derive(
    Value, Into, ElementBuilder, HtmlElement, ElementEvents, HtmlElementEvents, AriaElement,
)]
#[element_target(Button)]
pub struct ButtonBuilder(HtmlElementBuilder);

pub fn button<'a>(
    button_type: &str,
    text: impl RefSignalOrValue<'a, Item = impl AsRef<str> + Into<String> + 'a>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> ButtonBuilder {
    ButtonBuilder(
        HtmlElementBuilder(html::button().r#type(button_type).class(css::BTN).into())
            .class(style.map(ButtonStyle::class)),
    )
    .text(text)
}

pub fn icon_button(
    button_type: &str,
    icon: impl SignalOrValue<Item = impl Value + Into<Icon> + 'static>,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> ButtonBuilder {
    ButtonBuilder(
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

impl ButtonBuilder {
    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> ButtonBuilder {
        ButtonBuilder(HtmlElementBuilder(self.0 .0.text(text)))
    }

    pub fn icon(
        self,
        icon: impl SignalOrValue<Item = impl Value + Into<Icon> + 'static>,
    ) -> ButtonBuilder {
        ButtonBuilder(HtmlElementBuilder(
            self.0 .0.child(icon.map(|icon| icon.into())),
        ))
    }
}

impl From<ButtonBuilder> for Element {
    fn from(builder: ButtonBuilder) -> Self {
        builder.0.into()
    }
}

impl From<ButtonBuilder> for Node {
    fn from(builder: ButtonBuilder) -> Self {
        builder.0.into()
    }
}

#[derive(Into)]
pub struct Button(Element);

impl From<HtmlElementBuilder> for Button {
    fn from(builder: HtmlElementBuilder) -> Self {
        Self(builder.0.into())
    }
}

impl From<Button> for Node {
    fn from(elem: Button) -> Self {
        elem.0.into()
    }
}
