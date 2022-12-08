use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
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
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{button::Button, css};

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
#[element_target(Dropdown)]
pub struct Dropdown(Div);

pub fn dropdown(button: Button, menu: impl Into<Menu>) -> Dropdown {
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

impl From<Dropdown> for GenericElement {
    fn from(elem: Dropdown) -> Self {
        elem.0.into()
    }
}

impl From<Dropdown> for Node {
    fn from(elem: Dropdown) -> Self {
        elem.0.into()
    }
}

#[derive(Value)]
pub struct Menu(Ul);

pub fn dropdown_menu() -> Menu {
    Menu(ul().class(css::DROPDOWN_MENU))
}

impl Menu {
    pub fn child(self, child: impl SignalOrValue<Item = impl Into<MenuItem>>) -> Self {
        Self(self.0.child(child.map(|child| child.into().0)))
    }

    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Into<MenuItem>>> + 'static,
    ) -> Self {
        Self(
            self.0
                .optional_child(child.map(|child| child.map(|child| child.into().0))),
        )
    }

    pub fn children(self, children: impl IntoIterator<Item = impl Into<MenuItem>>) -> Self {
        Self(
            self.0
                .children(children.into_iter().map(|child| child.into().0)),
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<MenuItem>> + 'static,
    ) -> Self {
        Self(
            self.0
                .children_signal(children.map(|child| child.into().0))
        )
    }
}

impl From<Menu> for Node {
    fn from(elem: Menu) -> Self {
        elem.0.into()
    }
}

pub struct MenuItem(Node);

macro_rules! menu_items{
    ($($elem:path),* $(,)?) => {
        $(
            impl From<$elem> for MenuItem {
                fn from(item: $elem) -> Self {
                    Self(li().child(item.class(css::DROPDOWN_ITEM)).into())
                }
            }
        )*
    }
}

menu_items! {
    Hr,
    html::Button,
    Form,
    A,
    Span,
}
