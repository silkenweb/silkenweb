use derive_more::Into;
use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html::{span, ABuilder},
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use crate::{
    css,
    utility::{Colour, SetBorder},
    HtmlElementBuilder,
};

#[derive(ElementBuilder, Into)]
#[element_target(Badge)]
pub struct BadgeBuilder(HtmlElementBuilder);

pub fn badge(text: &str, background_colour: Colour) -> BadgeBuilder {
    BadgeBuilder(HtmlElementBuilder(
        span()
            .class([css::BADGE, background_colour.text_background()])
            .text(text)
            .into(),
    ))
}

pub fn badge_signal(
    text: impl Signal<Item = impl Into<String>> + 'static,
    background_colour: impl Signal<Item = Colour> + 'static,
) -> BadgeBuilder {
    BadgeBuilder(HtmlElementBuilder(
        span()
            // TODO: `class [_signal]` should be renamed to `classes [_signal]` and `class
            // [_signal]` should take a single class
            .class([css::BADGE])
            .class_signal(background_colour.map(|colour| [colour.text_background()]))
            .text_signal(text)
            .into(),
    ))
}

pub fn link_badge(link: ABuilder, background_colour: Colour) -> BadgeBuilder {
    BadgeBuilder(HtmlElementBuilder(
        link.class([css::BADGE, background_colour.text_background()])
            .into(),
    ))
}

pub fn link_badge_signal(
    link: ABuilder,
    background_colour: impl Signal<Item = Colour> + 'static,
) -> BadgeBuilder {
    BadgeBuilder(HtmlElementBuilder(
        link.class([css::BADGE])
            .class_signal(background_colour.map(|colour| [colour.text_background()]))
            .into(),
    ))
}

impl BadgeBuilder {
    pub fn rounded_pill_border(self) -> Self {
        Self(self.0.rounded_pill_border())
    }
}

impl ElementEvents for BadgeBuilder {}
impl HtmlElementEvents for BadgeBuilder {}

impl From<BadgeBuilder> for Node {
    fn from(badge: BadgeBuilder) -> Self {
        badge.0.into()
    }
}

#[derive(Into)]
pub struct Badge(Node);

impl From<HtmlElementBuilder> for Badge {
    fn from(builder: HtmlElementBuilder) -> Self {
        Self(builder.0.into())
    }
}
