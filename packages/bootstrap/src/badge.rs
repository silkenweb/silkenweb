use derive_more::Into;
use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html::{span, SpanBuilder},
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use crate::{
    css,
    utility::{Colour, SetBorder, SetSpacing},
};

#[derive(ElementBuilder, Into)]
#[element_target(Badge)]
pub struct BadgeBuilder(SpanBuilder);

pub fn badge(text: &str, background_colour: Colour) -> BadgeBuilder {
    BadgeBuilder(
        span()
            .class([css::BADGE, background_colour.text_background()])
            .text(text),
    )
}

pub fn badge_signal(
    text: impl Signal<Item = impl Into<String>> + 'static,
    background_colour: impl Signal<Item = Colour> + 'static,
) -> BadgeBuilder {
    BadgeBuilder(
        span()
            // TODO: `class [_signal]` should be renamed to `classes [_signal]` and `class
            // [_signal]` should take a single class
            .class([css::BADGE])
            .class_signal(background_colour.map(|colour| [colour.text_background()]))
            .text_signal(text),
    )
}

impl BadgeBuilder {
    pub fn rounded_pill_border(self) -> Self {
        Self(self.0.rounded_pill_border())
    }
}

impl ElementEvents for BadgeBuilder {}
impl HtmlElementEvents for BadgeBuilder {}
impl SetSpacing for BadgeBuilder {}

impl From<BadgeBuilder> for Node {
    fn from(badge: BadgeBuilder) -> Self {
        badge.0.into()
    }
}

#[derive(Into)]
pub struct Badge(Node);

impl From<SpanBuilder> for Badge {
    fn from(builder: SpanBuilder) -> Self {
        Self(builder.into())
    }
}
