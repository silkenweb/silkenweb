use std::marker::PhantomData;

use derive_more::Into;
use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html,
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use crate::{css, utility::Colour, Class, HtmlElementBuilder};

// TODO: Doc
pub struct Set;
// TODO: Doc
pub struct Unset;

pub enum State {}
#[derive(ElementBuilder, Into)]
#[element_target(Button)]
pub struct ButtonBuilder<Style, Content>(
    HtmlElementBuilder,
    PhantomData<Style>,
    PhantomData<Content>,
);

// TODO: Construct different types of button:
// (<button> | <a> | <input>)
// TODO: <a> and <input> buttons
pub fn button() -> ButtonBuilder<Unset, Unset> {
    ButtonBuilder(
        HtmlElementBuilder(html::button().class([css::BTN]).into()),
        PhantomData,
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

impl<Content> ButtonBuilder<Unset, Content> {
    pub fn appearance(self, style: ButtonStyle) -> ButtonBuilder<Set, Content> {
        ButtonBuilder::chain(self.0.class([style.class()]))
    }

    pub fn appearance_signal(
        self,
        style: impl Signal<Item = ButtonStyle> + 'static,
    ) -> ButtonBuilder<Set, Content> {
        ButtonBuilder::chain(self.0.class_signal(style.map(|style| [style.class()])))
    }
}

impl<Style, Content> ButtonBuilder<Style, Content> {
    pub fn text(self, text: &str) -> ButtonBuilder<Style, Set> {
        ButtonBuilder::chain(HtmlElementBuilder(self.0 .0.text(text)))
    }

    pub fn text_signal(
        self,
        text: impl Signal<Item = String> + 'static,
    ) -> ButtonBuilder<Set, Content> {
        ButtonBuilder::chain(HtmlElementBuilder(self.0 .0.text_signal(text)))
    }

    // TODO: Icon children
}

impl<Style, Content> ButtonBuilder<Style, Content> {
    fn chain(elem: HtmlElementBuilder) -> Self {
        Self(elem, PhantomData, PhantomData)
    }
}

impl<Style, Content> HtmlElement for ButtonBuilder<Style, Content> {}
impl<Style, Content> ElementEvents for ButtonBuilder<Style, Content> {}
impl<Style, Content> HtmlElementEvents for ButtonBuilder<Style, Content> {}

impl From<ButtonBuilder<Set, Set>> for Node {
    fn from(builder: ButtonBuilder<Set, Set>) -> Self {
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
