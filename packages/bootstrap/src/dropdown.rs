use derive_more::Into;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use silkenweb::{
    elements::html::{
        li, ul, ABuilder, ButtonBuilder, FormBuilder, HrBuilder, SpanBuilder, UlBuilder,
    },
    node::{element::ElementBuilder, Node},
    prelude::ParentBuilder,
    value::SignalOrValue,
};

use crate::css;

pub struct MenuBuilder(UlBuilder);

pub fn menu() -> MenuBuilder {
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

// TODO: Once we've written the dropdown container, we won't need this.
impl From<MenuBuilder> for Node {
    fn from(builder: MenuBuilder) -> Self {
        builder.0.into()
    }
}

#[derive(Into)] // TODO: Once we've written the dropdown container, we won't need to derive Into
pub struct Menu(Node);

pub struct MenuItem(Node);

macro_rules! menu_items{
    ($($elem:ty),* $(,)?) => {
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
    // TODO: Wrap button
    ButtonBuilder,
    FormBuilder,
    ABuilder,
    SpanBuilder,
}
