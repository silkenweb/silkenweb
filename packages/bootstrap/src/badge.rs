use derive_more::Into;
use silkenweb::{
    elements::html::{span, SpanBuilder},
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElementEvents, ParentBuilder},
    value::{RefSignalOrValue, SignalOrValue},
    ElementBuilder, Value,
};

use crate::{
    css,
    utility::{Colour, SetBorder, SetSpacing},
};

#[derive(ElementBuilder, Into, Value)]
#[element_target(Badge)]
pub struct BadgeBuilder(SpanBuilder);

pub fn badge<'a>(
    text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    background_colour: impl SignalOrValue<Item = Colour>,
) -> BadgeBuilder {
    BadgeBuilder(
        span()
            .class(css::BADGE)
            .class(background_colour.map(Colour::text_background))
            .text(text),
    )
}

impl BadgeBuilder {
    pub fn rounded_pill_border(self) -> Self {
        Self(self.0.rounded_pill_border(true))
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
