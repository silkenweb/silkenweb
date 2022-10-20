use derive_more::Into;
use silkenweb::{
    elements::html::{span, SpanBuilder},
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::ParentBuilder,
    value::{RefSignalOrValue, SignalOrValue},
    ElementBuilder, ElementEvents, HtmlElementEvents, Value,
};

use crate::{
    css,
    utility::{Colour, SetBorder, SetSpacing},
};

#[derive(ElementBuilder, ElementEvents, HtmlElementEvents, Into, Value)]
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

impl SetSpacing for BadgeBuilder {}

impl From<BadgeBuilder> for Element {
    fn from(badge: BadgeBuilder) -> Self {
        badge.0.into()
    }
}

impl From<BadgeBuilder> for Node {
    fn from(badge: BadgeBuilder) -> Self {
        badge.0.into()
    }
}

#[derive(Into)]
pub struct Badge(Element);

impl From<Badge> for Node {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}

impl From<SpanBuilder> for Badge {
    fn from(builder: SpanBuilder) -> Self {
        Self(builder.into())
    }
}
