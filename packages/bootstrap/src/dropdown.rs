use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    dom::{DefaultDom, Dom},
    elements::{
        html::{self, div, li, ul, Div, Form, Hr, Span, Ul, A},
        AriaElement,
    },
    node::{
        element::{Element, GenericElement},
        Node,
    },
    prelude::ParentElement,
    value::SignalOrValue,
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value, ServerSend,
};

use crate::{button::Button, css};

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct Dropdown<D: Dom = DefaultDom>(Div<D>);

pub fn dropdown<D: Dom>(button: Button<D>, menu: impl Into<Menu<D>>) -> Dropdown<D> {
    Dropdown(
        div()
            .classes([css::DROPDOWN])
            .child(
                button
                    .class(css::DROPDOWN_TOGGLE)
                    .aria_expanded("false")
                    .attribute("data-bs-toggle", "dropdown"),
            )
            .child(menu.into().0),
    )
}

impl<D: Dom> From<Dropdown<D>> for GenericElement<D> {
    fn from(elem: Dropdown<D>) -> Self {
        elem.0.into()
    }
}

impl<D: Dom> From<Dropdown<D>> for Node<D> {
    fn from(elem: Dropdown<D>) -> Self {
        elem.0.into()
    }
}

#[derive(Value)]
pub struct Menu<D: Dom = DefaultDom>(Ul<D>);

pub fn dropdown_menu<D: Dom>() -> Menu<D> {
    Menu(ul().class(css::DROPDOWN_MENU))
}

impl<D: Dom> Menu<D> {
    pub fn child(self, child: impl SignalOrValue<Item = impl Into<MenuItem<D>>>) -> Self {
        Self(self.0.child(child.map(|child| child.into().0)))
    }

    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Into<MenuItem<D>>>> + 'static,
    ) -> Self {
        Self(
            self.0
                .optional_child(child.map(|child| child.map(|child| child.into().0))),
        )
    }

    pub fn children<Iter, M>(self, children: impl IntoIterator<Item = M, IntoIter = Iter>) -> Self
    where
        M: Into<MenuItem<D>>,
        Iter: Iterator<Item = M> + ServerSend,
    {
        Self(
            self.0
                .children(children.into_iter().map(|child| child.into().0)),
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<MenuItem<D>>> + ServerSend + 'static,
    ) -> Self {
        Self(self.0.children_signal(children.map(|child| child.into().0)))
    }
}

impl<D: Dom> From<Menu<D>> for Node<D> {
    fn from(elem: Menu<D>) -> Self {
        elem.0.into()
    }
}

pub struct MenuItem<D: Dom = DefaultDom>(Node<D>);

macro_rules! menu_items{
    ($($elem:path),* $(,)?) => {
        $(
            impl<D: Dom> From<$elem> for MenuItem<D> {
                fn from(item: $elem) -> Self {
                    Self(li().child(item.class(css::DROPDOWN_ITEM)).into())
                }
            }
        )*
    }
}

menu_items! {
    Hr<D>,
    html::Button<D>,
    Form<D>,
    A<D>,
    Span<D>,
}
