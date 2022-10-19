use derive_more::Into;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::{
        html::{
            self, div, li, ul, ABuilder, DivBuilder, FormBuilder, HrBuilder, SpanBuilder, UlBuilder,
        },
        AriaElement,
    },
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    prelude::ParentBuilder,
    value::SignalOrValue,
    AriaElement, ElementBuilder, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};

use crate::{button::ButtonBuilder, css};

#[derive(Value, ElementBuilder, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
#[element_target(Dropdown)]
pub struct DropdownBuilder(DivBuilder);

pub fn dropdown(button: ButtonBuilder, menu: impl Into<Menu>) -> DropdownBuilder {
    DropdownBuilder(
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

impl From<DropdownBuilder> for Element {
    fn from(builder: DropdownBuilder) -> Self {
        builder.0.into()
    }
}

impl From<DropdownBuilder> for Node {
    fn from(builder: DropdownBuilder) -> Self {
        builder.0.into()
    }
}

#[derive(Into, Value)]
pub struct Dropdown(Element);

impl From<Dropdown> for Node {
    fn from(elem: Dropdown) -> Self {
        elem.0.into()
    }
}

#[derive(Value)]
pub struct MenuBuilder(UlBuilder);

pub fn dropdown_menu() -> MenuBuilder {
    MenuBuilder(ul().class(css::DROPDOWN_MENU))
}

impl MenuBuilder {
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
    ) -> Menu {
        Menu(
            self.0
                .children_signal(children.map(|child| child.into().0))
                .into(),
        )
    }
}

#[derive(Value)]
pub struct Menu(Node);

impl From<MenuBuilder> for Menu {
    fn from(builder: MenuBuilder) -> Self {
        Menu(builder.0.into())
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
    HrBuilder,
    html::ButtonBuilder,
    FormBuilder,
    ABuilder,
    SpanBuilder,
}
