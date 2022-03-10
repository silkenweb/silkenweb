use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use futures_signals::{signal::Signal, signal_vec::SignalVec};
use silkenweb::{
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};

use self::elements::{
    ui5_side_navigation, ui5_side_navigation_item, ui5_side_navigation_sub_item, Ui5SideNavigation,
    Ui5SideNavigationBuilder, Ui5SideNavigationItem, Ui5SideNavigationItemBuilder,
    Ui5SideNavigationSubItemBuilder,
};
use crate::icon::Icon;

mod elements {
    use silkenweb::{html_element, parent_element};

    use crate::icon::Icon;

    html_element!(
        ui5-side-navigation<web_sys::HtmlElement> {
            attributes {
                collapsed: bool,
            }

            custom_events {
                selection-change: web_sys::CustomEvent,
            }
        }
    );

    parent_element!(ui5 - side - navigation);

    html_element!(
        ui5-side-navigation-item<web_sys::HtmlElement> {
            attributes {
                expanded: bool,
                icon: Icon,
                selected: bool,
                text: String,
                whole-item-toggleable: bool
            }
        }
    );

    parent_element!(ui5 - side - navigation - item);

    html_element!(
        ui5-side-navigation-sub-item<web_sys::HtmlElement> {
            attributes {
                expanded: bool,
                icon: Icon,
                selected: bool,
                text: String,
            }
        }
    );
}

pub type SideNavigation = elements::Ui5SideNavigation;
pub type SideNavigationItem = elements::Ui5SideNavigationItem;
pub type SideNavigationSubItem = elements::Ui5SideNavigationSubItem;

pub fn side_navigation<Id>() -> SideNavigationBuilder<Id> {
    SideNavigationBuilder(ui5_side_navigation(), PhantomData)
}

#[derive(ElementBuilder)]
pub struct SideNavigationBuilder<Id>(Ui5SideNavigationBuilder, PhantomData<Id>);

impl<Id> SideNavigationBuilder<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn collapsed(self) -> Self {
        Self(self.0.collapsed(true), PhantomData)
    }

    pub fn collapsed_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.collapsed_signal(value), PhantomData)
    }

    // We don't include `child` and `child_signal` methods as they're not so useful
    // when the item type is specific.
    pub fn children(
        self,
        children: impl IntoIterator<Item = SideNavigationItemBuilder<Id>>,
    ) -> Self {
        Self(self.0.children(children), PhantomData)
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = SideNavigationItemBuilder<Id>> + 'static,
    ) -> Ui5SideNavigation {
        self.0.children_signal(children)
    }

    pub fn on_selection_change(self, mut f: impl FnMut(Id) + 'static) -> Self {
        Self(
            self.0.on_selection_change(move |event, _target| {
                f(event
                    .detail()
                    .unchecked_into::<Item>()
                    .item()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap_throw()
                    .get_attribute(SELECTED_ID)
                    .unwrap_throw()
                    .parse()
                    .unwrap_throw())
            }),
            PhantomData,
        )
    }
}

impl<Id> HtmlElement for SideNavigationBuilder<Id> {}

impl<Id> HtmlElementEvents for SideNavigationBuilder<Id> {}

impl<Id> ElementEvents for SideNavigationBuilder<Id> {}

impl<T> From<SideNavigationBuilder<T>> for Node {
    fn from(builder: SideNavigationBuilder<T>) -> Self {
        builder.0.into()
    }
}

pub fn item<Id: Display>(id: Id) -> SideNavigationItemBuilder<Id> {
    SideNavigationItemBuilder(
        ui5_side_navigation_item().attribute(SELECTED_ID, &id.to_string()),
        PhantomData,
    )
}

#[derive(ElementBuilder)]
pub struct SideNavigationItemBuilder<Id>(Ui5SideNavigationItemBuilder, PhantomData<Id>);

impl<Id> SideNavigationItemBuilder<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn expanded(self) -> Self {
        Self(self.0.expanded(true), PhantomData)
    }

    pub fn expanded_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.expanded_signal(value), PhantomData)
    }

    pub fn icon(self, icon: Icon) -> Self {
        Self(self.0.icon(icon), PhantomData)
    }

    pub fn icon_signal(self, value: impl Signal<Item = Icon> + 'static) -> Self {
        Self(self.0.icon_signal(value), PhantomData)
    }

    pub fn selected(self) -> Self {
        Self(self.0.selected(true), PhantomData)
    }

    pub fn selected_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.selected_signal(value), PhantomData)
    }

    pub fn text(self, text: &str) -> Self {
        Self(self.0.text(text), PhantomData)
    }

    pub fn text_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.text_signal(value), PhantomData)
    }

    pub fn whole_item_toggleable(self) -> Self {
        Self(self.0.whole_item_toggleable(true), PhantomData)
    }

    pub fn whole_item_toggleable_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.whole_item_toggleable_signal(value), PhantomData)
    }

    // We don't include `child` and `child_signal` methods as they're not so useful
    // when the item type is specific.
    pub fn children(
        self,
        children: impl IntoIterator<Item = SideNavigationSubItemBuilder<Id>>,
    ) -> Self {
        Self(self.0.children(children), PhantomData)
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = SideNavigationSubItemBuilder<Id>> + 'static,
    ) -> Ui5SideNavigationItem {
        self.0.children_signal(children)
    }
}

impl<Id> HtmlElement for SideNavigationItemBuilder<Id> {}

impl<Id> HtmlElementEvents for SideNavigationItemBuilder<Id> {}

impl<Id> ElementEvents for SideNavigationItemBuilder<Id> {}

impl<T> From<SideNavigationItemBuilder<T>> for Node {
    fn from(builder: SideNavigationItemBuilder<T>) -> Self {
        builder.0.into()
    }
}

pub fn sub_item<Id: Display>(id: Id) -> SideNavigationSubItemBuilder<Id> {
    SideNavigationSubItemBuilder(
        ui5_side_navigation_sub_item().attribute(SELECTED_ID, &id.to_string()),
        PhantomData,
    )
}

#[derive(ElementBuilder)]
pub struct SideNavigationSubItemBuilder<Id>(Ui5SideNavigationSubItemBuilder, PhantomData<Id>);

impl<Id> SideNavigationSubItemBuilder<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn expanded(self) -> Self {
        Self(self.0.expanded(true), PhantomData)
    }

    pub fn expanded_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.expanded_signal(value), PhantomData)
    }

    pub fn icon(self, icon: Icon) -> Self {
        Self(self.0.icon(icon), PhantomData)
    }

    pub fn icon_signal(self, value: impl Signal<Item = Icon> + 'static) -> Self {
        Self(self.0.icon_signal(value), PhantomData)
    }

    pub fn selected(self) -> Self {
        Self(self.0.selected(true), PhantomData)
    }

    pub fn selected_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.selected_signal(value), PhantomData)
    }

    pub fn text(self, text: &str) -> Self {
        Self(self.0.text(text), PhantomData)
    }

    pub fn text_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.text_signal(value), PhantomData)
    }
}

impl<Id> HtmlElement for SideNavigationSubItemBuilder<Id> {}

impl<Id> HtmlElementEvents for SideNavigationSubItemBuilder<Id> {}

impl<Id> ElementEvents for SideNavigationSubItemBuilder<Id> {}

impl<T> From<SideNavigationSubItemBuilder<T>> for Node {
    fn from(builder: SideNavigationSubItemBuilder<T>) -> Self {
        builder.0.into()
    }
}

#[wasm_bindgen]
extern "C" {
    type Item;

    #[wasm_bindgen(structural, method, getter)]
    fn item(this: &Item) -> JsValue;
}

const SELECTED_ID: &str = "data-silkenweb-ui5-id";
