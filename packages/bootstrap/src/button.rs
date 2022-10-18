use std::marker::PhantomData;

use derive_more::Into;
use silkenweb::{
    elements::html,
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    value::{RefSignalOrValue, SignalOrValue, Value},
    ElementBuilder,
};

use crate::{css, icon::Icon, utility::Colour, Class, HtmlElementBuilder};

// TODO: Doc
pub struct Set;
// TODO: Doc
pub struct Unset;

pub enum State {}
#[derive(ElementBuilder, Into)]
#[element_target(Button)]
pub struct ButtonBuilder<Content>(HtmlElementBuilder, PhantomData<Content>);

impl Value for ButtonBuilder<Set> {}

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

#[derive(Copy, Clone, Eq, PartialEq)]
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

impl Value for ButtonStyle {}

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

impl<Content> HtmlElement for ButtonBuilder<Content> {}
impl<Content> ElementEvents for ButtonBuilder<Content> {}
impl<Content> HtmlElementEvents for ButtonBuilder<Content> {}

impl From<ButtonBuilder<Set>> for Node {
    fn from(builder: ButtonBuilder<Set>) -> Self {
        builder.0.into()
    }
}

#[derive(Into)]
pub struct Button(Node);

impl From<HtmlElementBuilder> for Button {
    fn from(builder: HtmlElementBuilder) -> Self {
        Self(builder.0.into())
    }
}
