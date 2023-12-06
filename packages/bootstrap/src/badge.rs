use derive_more::Into;
use silkenweb::{
    dom::{DefaultDom, Dom},
    elements::html::{span, Span},
    node::{
        element::{Element, GenericElement, ParentElement},
        Node,
    },
    value::{RefSignalOrValue, SignalOrValue},
    Element, ElementEvents, HtmlElementEvents, Value,
};

use crate::{
    css,
    utility::{Colour, SetBorder, SetSpacing},
};

#[derive(Element, ElementEvents, HtmlElementEvents, Into, Value)]
pub struct Badge<D: Dom = DefaultDom>(Span<D>);

pub fn badge<'a, D: Dom>(
    text: impl RefSignalOrValue<'a, Item = impl Into<String> + AsRef<str> + 'a>,
    background_colour: impl SignalOrValue<Item = Colour>,
) -> Badge<D> {
    Badge(
        span()
            .class(css::BADGE)
            .class(background_colour.map(Colour::text_background))
            .text(text),
    )
}

impl<D: Dom> Badge<D> {
    pub fn rounded_pill_border(self) -> Self {
        Self(self.0.rounded_pill_border(true))
    }
}

impl<D: Dom> SetSpacing for Badge<D> {}

impl<D: Dom> From<Badge<D>> for GenericElement<D> {
    fn from(badge: Badge<D>) -> Self {
        badge.0.into()
    }
}

impl<D: Dom> From<Badge<D>> for Node<D> {
    fn from(badge: Badge<D>) -> Self {
        badge.0.into()
    }
}
