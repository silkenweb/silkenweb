use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html::{span, Span, SpanBuilder},
    extend_html_element,
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};

use crate::{css, utility::Colour};

#[derive(ElementBuilder)]
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

extend_html_element!(BadgeBuilder {0});

pub struct Badge(Span);

impl From<Badge> for Node {
    fn from(badge: Badge) -> Self {
        badge.0.into()
    }
}
