use std::marker::PhantomData;

use derive_more::Into;
use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html,
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    value::{RefSignalOrValue, Sig, SignalOrValue, Value},
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

// TODO: Construct different types of button:
// (<button> | <a> | <input>)
// TODO: <a> and <input> buttons
pub fn button(button_type: &str, style: ButtonStyle) -> ButtonBuilder<Unset> {
    ButtonBuilder::chain(base_button(button_type).0.class(style.class()))
}

pub fn button_signal(
    button_type: &str,
    style: impl Signal<Item = ButtonStyle> + 'static,
) -> ButtonBuilder<Unset> {
    ButtonBuilder::chain(
        base_button(button_type)
            .0
            .class(Sig(style.map(ButtonStyle::class))),
    )
}

fn base_button(button_type: &str) -> ButtonBuilder<Unset> {
    ButtonBuilder(
        HtmlElementBuilder(html::button().r#type(button_type).class(css::BTN).into()),
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
