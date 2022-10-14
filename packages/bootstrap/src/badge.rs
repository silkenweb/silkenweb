use futures_signals::signal::Signal;
use silkenweb::{
    elements::html::{span, SpanBuilder},
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use crate::{colour::Colour, css};

#[derive(ElementBuilder)]
pub struct Badge(SpanBuilder);

impl Badge {
    pub fn new(text: &str, background_colour: Colour) -> Self {
        Self(Self::base_element(background_colour).text(text))
    }

    pub fn new_signal(
        text: impl Signal<Item = impl Into<String>> + 'static,
        background_colour: Colour,
    ) -> Self {
        Self(Self::base_element(background_colour).text_signal(text))
    }

    pub fn rounded_pill(self) -> Self {
        Self(self.0.class([css::ROUNDED_PILL]))
    }

    fn base_element(background_colour: Colour) -> SpanBuilder {
        span().class([css::BADGE, background_colour.text_background_class()])
    }
}

impl HtmlElement for Badge {}
impl HtmlElementEvents for Badge {}
impl ElementEvents for Badge {}

// TODO: Should this be part of derive(ElementBuilder)?
impl From<Badge> for Element {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}

impl From<Badge> for Node {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}

impl From<Badge> for SpanBuilder {
    fn from(badge: Badge) -> Self {
        badge.0
    }
}
