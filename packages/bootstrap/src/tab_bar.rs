use std::marker::PhantomData;

use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::{
        html::{self, li, nav, ol, ul, Nav, Ol, Ul},
        AriaElement,
    },
    node::{
        element::{Element, GenericElement, ParentElement},
        ChildNode, Node,
    },
    value::SignalOrValue,
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{css, dropdown::Menu, List};

pub fn tab_bar() -> TabBar<Nav> {
    TabBar(nav().class(css::NAV).into(), PhantomData)
}

pub fn tab_bar_unordered() -> TabBar<Ul> {
    TabBar(ul().class(css::NAV).into(), PhantomData)
}

pub fn tab_bar_ordered() -> TabBar<Ol> {
    TabBar(ol().class(css::NAV).into(), PhantomData)
}

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct TabBar<Base = Nav>(#[element(target)] GenericElement, PhantomData<Base>);

impl<Base> TabBar<Base> {
    pub fn style(self, style: impl SignalOrValue<Item = Style>) -> Self {
        TabBar(
            self.0.classes(style.map(|s| match s {
                Style::Plain => None,
                Style::Tabs => Some(css::NAV_TABS),
                Style::Pills => Some(css::NAV_PILLS),
            })),
            PhantomData,
        )
    }

    pub fn fill(self, fill: impl SignalOrValue<Item = Fill>) -> Self {
        TabBar(
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

    pub fn children<T>(self, children: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<TabBarItem<Base>>,
    {
        Self(
            self.0
                .children(children.into_iter().map(|child| child.into().0)),
            PhantomData,
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<TabBarItem<Base>>> + 'static,
    ) -> Self {
        Self(
            self.0.children_signal(children.map(|child| child.into().0)),
            PhantomData,
        )
    }
}

impl<Base> From<TabBar<Base>> for Node {
    fn from(elem: TabBar<Base>) -> Self {
        elem.0.into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum Style {
    Plain,
    Tabs,
    Pills,
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum Fill {
    Compact,
    Stretch,
    Justified,
}

#[derive(Value)]
pub struct TabBarItem<Base = Nav>(Node, PhantomData<Base>);

impl<A: TabBarElement> From<A> for TabBarItem<Nav> {
    fn from(elem: A) -> Self {
        Self(elem.class(css::NAV_LINK).into(), PhantomData)
    }
}

impl<A: TabBarElement, L: List> From<A> for TabBarItem<L> {
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
    pub fn dropdown(item: impl TabBarElement, menu: impl Into<Menu>) -> Self {
        Self(
            li().class(css::NAV_ITEM)
                .child(
                    item.classes([css::NAV_LINK, css::DROPDOWN_TOGGLE])
                        .attribute("data-bs-toggle", "dropdown")
                        .role("button")
                        .aria_expanded("false")
                        .into(),
                )
                .child(menu.into())
                .into(),
            PhantomData,
        )
    }
}

pub trait TabBarElement: Element + AriaElement + ParentElement + ChildNode {}
impl TabBarElement for html::A {}
impl TabBarElement for html::Button {}
