use derive_more::Into;
use silkenweb::{
    elements::html::{span, Span},
    node::{
        element::{Element, GenericElement},
        Node,
    },
    prelude::ParentElement,
    value::{RefSignalOrValue, SignalOrValue},
    Element, ElementEvents, HtmlElementEvents, Value,
};

use crate::{
    css,
    utility::{Colour, SetBorder, SetSpacing},
};

#[derive(Element, ElementEvents, HtmlElementEvents, Into, Value)]
pub struct Badge(Span);

pub fn badge<'a>(
    text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    background_colour: impl SignalOrValue<Item = Colour>,
) -> Badge {
    Badge(
        span()
            .class(css::BADGE)
            .class(background_colour.map(Colour::text_background))
            .text(text),
    )
}

impl Badge {
    pub fn rounded_pill_border(self) -> Self {
        Self(self.0.rounded_pill_border(true))
    }
}

impl SetSpacing for Badge {}

impl From<Badge> for GenericElement {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}

impl From<Badge> for Node {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}
