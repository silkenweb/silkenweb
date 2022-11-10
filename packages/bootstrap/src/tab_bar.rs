use derive_more::Into;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::html::{self, nav, ABuilder, NavBuilder},
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::ParentBuilder,
    value::SignalOrValue,
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, utility::SetFlex};

pub fn tab_bar() -> TabBarBuilder {
    TabBarBuilder(nav().class(css::NAV))
}

#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
#[element_target(TabBar)]
pub struct TabBarBuilder(NavBuilder);

impl TabBarBuilder {
    pub fn style(self, style: impl SignalOrValue<Item = Style>) -> Self {
        TabBarBuilder(self.0.classes(style.map(|s| match s {
            Style::Plain => None,
            Style::Tabs => Some(css::NAV_TABS),
            Style::Pills => Some(css::NAV_PILLS),
        })))
    }

    pub fn fill(self, fill: impl SignalOrValue<Item = Fill>) -> Self {
        TabBarBuilder(self.0.classes(fill.map(|fl| match fl {
            Fill::Compact => None,
            Fill::Stretch => Some(css::NAV_FILL),
            Fill::Justified => Some(css::NAV_JUSTIFIED),
        })))
    }

    pub fn child(self, child: impl SignalOrValue<Item = impl Into<TabBarItem>>) -> Self {
        Self(self.0.child(child.map(|child| child.into().0)))
    }

    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Into<TabBarItem>>> + 'static,
    ) -> Self {
        Self(
            self.0
                .optional_child(child.map(|child| child.map(|child| child.into().0))),
        )
    }

    pub fn children(self, children: impl IntoIterator<Item = impl Into<TabBarItem>>) -> Self {
        Self(
            self.0
                .children(children.into_iter().map(|child| child.into().0)),
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<TabBarItem>> + 'static,
    ) -> TabBar {
        TabBar(
            self.0
                .children_signal(children.map(|child| child.into().0))
                .into(),
        )
    }
}

impl SetFlex for TabBarBuilder {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Style {
    Plain,
    Tabs,
    Pills,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Fill {
    Compact,
    Stretch,
    Justified,
}

pub struct TabBarItem(Node);

impl From<ABuilder> for TabBarItem {
    fn from(elem: ABuilder) -> Self {
        Self(elem.class(css::NAV_LINK).into())
    }
}

impl From<html::ButtonBuilder> for TabBarItem {
    fn from(elem: html::ButtonBuilder) -> Self {
        Self(elem.class(css::NAV_LINK).into())
    }
}

#[derive(Into, Value)]
pub struct TabBar(Element);

impl From<TabBar> for Node {
    fn from(elem: TabBar) -> Self {
        elem.0.into()
    }
}
