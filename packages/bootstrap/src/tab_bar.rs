use std::marker::PhantomData;

use derive_more::Into;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::html::{self, li, nav, ABuilder, Nav},
    node::{
        element::{Element, ElementBuilder, ElementBuilderBase},
        Node,
    },
    prelude::ParentBuilder,
    value::{SignalOrValue, Value},
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, dropdown::Menu, utility::SetFlex, List};

pub fn tab_bar() -> TabBarBuilder<Nav> {
    tab_bar_on()
}

pub fn tab_bar_on<Base>() -> TabBarBuilder<Base> {
    TabBarBuilder(nav().class(css::NAV).into(), PhantomData)
}

#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
#[element_target(TabBar)]
pub struct TabBarBuilder<Base = Nav>(ElementBuilderBase, PhantomData<Base>);

impl<Base> TabBarBuilder<Base> {
    pub fn style(self, style: impl SignalOrValue<Item = Style>) -> Self {
        TabBarBuilder(
            self.0.classes(style.map(|s| match s {
                Style::Plain => None,
                Style::Tabs => Some(css::NAV_TABS),
                Style::Pills => Some(css::NAV_PILLS),
            })),
            PhantomData,
        )
    }

    pub fn fill(self, fill: impl SignalOrValue<Item = Fill>) -> Self {
        TabBarBuilder(
            self.0.classes(fill.map(|fl| match fl {
                Fill::Compact => None,
                Fill::Stretch => Some(css::NAV_FILL),
                Fill::Justified => Some(css::NAV_JUSTIFIED),
            })),
            PhantomData,
        )
    }

    pub fn child(self, child: impl SignalOrValue<Item = impl Into<TabBarItem<Base>>>) -> Self {
        Self(self.0.child(child.map(|child| child.into().0)), PhantomData)
    }

    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Into<TabBarItem<Base>>>> + 'static,
    ) -> Self {
        Self(
            self.0
                .optional_child(child.map(|child| child.map(|child| child.into().0))),
            PhantomData,
        )
    }

    pub fn children(self, children: impl IntoIterator<Item = impl Into<TabBarItem<Base>>>) -> Self {
        Self(
            self.0
                .children(children.into_iter().map(|child| child.into().0)),
            PhantomData,
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<TabBarItem<Base>>> + 'static,
    ) -> TabBar {
        TabBar(self.0.children_signal(children.map(|child| child.into().0)))
    }
}

impl<Base> SetFlex for TabBarBuilder<Base> {}

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

pub struct TabBarItem<Base = Nav>(Node, PhantomData<Base>);

impl<A: Actionable> From<A> for TabBarItem<Nav> {
    fn from(elem: A) -> Self {
        Self(elem.class(css::NAV_LINK).into(), PhantomData)
    }
}

impl<A: Actionable, L: List> From<A> for TabBarItem<L> {
    fn from(elem: A) -> Self {
        Self(
            li().class(css::NAV_ITEM)
                .child(elem.class(css::NAV_LINK))
                .into(),
            PhantomData,
        )
    }
}

impl<L: List> TabBarItem<L> {
    pub fn dropdown(item: TabBarItem<Nav>, menu: impl Into<Menu>) -> Self {
        Self(
            li().class(css::NAV_ITEM)
                .child(item.0)
                .child(menu.into())
                .into(),
            PhantomData,
        )
    }
}

pub trait Actionable: ElementBuilder + ParentBuilder + Into<Node> + Value + 'static {}
impl Actionable for ABuilder {}
impl Actionable for html::ButtonBuilder {}

#[derive(Into, Value)]
pub struct TabBar(Element);

impl From<TabBar> for Node {
    fn from(elem: TabBar) -> Self {
        elem.0.into()
    }
}
