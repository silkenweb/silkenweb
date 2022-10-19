use std::marker::PhantomData;

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

// TODO: Doc
pub struct Set;
// TODO: Doc
pub struct Unset;

pub enum State {}
#[derive(
    Value, Into, ElementBuilder, HtmlElement, ElementEvents, HtmlElementEvents, AriaElement,
)]
#[element_target(Button)]
pub struct ButtonBuilder<Content = Set>(HtmlElementBuilder, PhantomData<Content>);

pub fn button(
    button_type: &str,
    style: impl SignalOrValue<Item = ButtonStyle>,
) -> ButtonBuilder<Unset> {
    ButtonBuilder(
        HtmlElementBuilder(html::button().r#type(button_type).class(css::BTN).into())
            .class(style.map(ButtonStyle::class)),
        PhantomData,
    )
}

// TODO: Does this apply to more than buttons?
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

impl<Content> ButtonBuilder<Content> {
    pub fn text<'a>(
        self,
        text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    ) -> ButtonBuilder<Set> {
        ButtonBuilder::chain(HtmlElementBuilder(self.0 .0.text(text)))
    }

    pub fn icon(
        self,
        icon: impl SignalOrValue<Item = impl Value + Into<Icon> + 'static>,
    ) -> ButtonBuilder<Set> {
        ButtonBuilder::chain(HtmlElementBuilder(
            self.0 .0.child(icon.map(|icon| icon.into())),
        ))
    }
}

impl<Content> ButtonBuilder<Content> {
    fn chain(elem: HtmlElementBuilder) -> Self {
        Self(elem, PhantomData)
    }
}

impl From<ButtonBuilder<Set>> for Element {
    fn from(builder: ButtonBuilder<Set>) -> Self {
        builder.0.into()
    }
}

impl From<ButtonBuilder<Set>> for Node {
    fn from(builder: ButtonBuilder<Set>) -> Self {
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
