use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html::{span, SpanBuilder},
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
pub struct Badge(SpanBuilder);

impl Badge {
    pub fn new(text: &str, background_colour: Colour) -> Self {
        Self(
            span()
                .class([css::BADGE, background_colour.text_background()])
                .text(text),
        )
    }

    pub fn new_signal(
        text: impl Signal<Item = impl Into<String>> + 'static,
        background_colour: impl Signal<Item = Colour> + 'static,
    ) -> Self {
        Self(
            span()
                // TODO: `class [_signal]` should be renamed to `classes [_signal]` and `class
                // [_signal]` should take a single class
                .class([css::BADGE])
                .class_signal(background_colour.map(|colour| [colour.text_background()]))
                .text_signal(text),
        )
    }

    pub fn rounded_pill(self) -> Self {
        Self(self.0.class([css::ROUNDED_PILL]))
    }
}

extend_html_element!(Badge {0});
