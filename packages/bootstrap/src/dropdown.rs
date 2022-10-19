use derive_more::Into;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::{
        html::{
            self, div, li, ul, ABuilder, DivBuilder, FormBuilder, HrBuilder, SpanBuilder, UlBuilder,
        },
        AriaElement,
    },
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    value::{SignalOrValue, Value},
    ElementBuilder,
};

use crate::{button::ButtonBuilder, css};

#[derive(ElementBuilder)]
#[element_target(Dropdown)]
pub struct DropdownBuilder(DivBuilder);

// TODO: Proc macro derives for all of these
impl Value for DropdownBuilder {}
impl HtmlElement for DropdownBuilder {}
impl HtmlElementEvents for DropdownBuilder {}
impl ElementEvents for DropdownBuilder {}
impl AriaElement for DropdownBuilder {}

pub fn dropdown(button: ButtonBuilder, menu: impl Into<Menu>) -> DropdownBuilder {
    // TODO: BTN_GROUP vs DROPDOWN classes: what's the difference?
    // DROPDOWN just applies position: relative, btn_group applies border radius,
    // display and alignment. We only want to apply button group when we're a
    // child of button group (button group show do this).
    DropdownBuilder(
        div()
            .classes([css::DROPDOWN])
            .child(
                button
                    .class(css::DROPDOWN_TOGGLE)
                    .aria_expanded("false")
                    .attribute("data-bs-toggle", "dropdown"),
            )
            .child(menu.into()),
    )
}

impl From<DropdownBuilder> for Node {
    fn from(builder: DropdownBuilder) -> Self {
        builder.0.into()
    }
}

#[derive(Into)]
pub struct Dropdown(Node);

impl Value for Dropdown {}

pub struct MenuBuilder(UlBuilder);

pub fn dropdown_menu() -> MenuBuilder {
    MenuBuilder(ul().class(css::DROPDOWN_MENU))
}

impl MenuBuilder {
    pub fn child(self, child: impl SignalOrValue<Item = MenuItem>) -> Self {
        Self(self.0.child(child.map(|child| child.0)))
    }

    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<MenuItem>> + 'static,
    ) -> Self {
        Self(
            self.0
                .optional_child(child.map(|child| child.map(|child| child.0))),
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

impl Value for MenuBuilder {}

// TODO: Once we've written the dropdown container, we won't need this.
impl From<MenuBuilder> for Node {
    fn from(builder: MenuBuilder) -> Self {
        builder.0.into()
    }
}

#[derive(Into)] // TODO: Once we've written the dropdown container, we won't need to derive Into
pub struct Menu(Node);

impl From<MenuBuilder> for Menu {
    fn from(builder: MenuBuilder) -> Self {
        Menu(builder.0.into())
    }
}

impl Value for Menu {}

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
    // TODO: Wrap button (and other items?)
    html::ButtonBuilder,
    FormBuilder,
    ABuilder,
    SpanBuilder,
}
