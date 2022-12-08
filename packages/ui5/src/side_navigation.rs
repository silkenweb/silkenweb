use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use futures_signals::signal_vec::SignalVec;
use silkenweb::{
    attribute::AsAttribute,
    elements::CustomEvent,
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentElement},
    value::{SignalOrValue, Value},
    ElementBuilder,
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

use self::elements::{
    ui5_side_navigation, ui5_side_navigation_item, ui5_side_navigation_sub_item, Ui5SideNavigation,
    Ui5SideNavigationItem, Ui5SideNavigationSubItem,
};
use crate::{icon::Icon, SELECTED_ID};

mod elements {
    use silkenweb::{custom_html_element, elements::CustomEvent, parent_element};

    use super::SelectionChangeDetail;
    use crate::icon::Icon;

    custom_html_element!(
        ui5_side_navigation = { dom_type: web_sys::HtmlElement;
            attributes {
                collapsed: bool,
            };

            custom_events {
                selection_change: CustomEvent<SelectionChangeDetail>,
            };
        }
    );

    parent_element!(ui5_side_navigation);

    custom_html_element!(
        ui5_side_navigation_item = {
            dom_type: web_sys::HtmlElement;
            attributes {
                expanded: bool,
                icon: Icon,
                selected: bool,
                text: String,
                whole_item_toggleable: bool,
            };
        }
    );

    parent_element!(ui5_side_navigation_item);

    custom_html_element!(
        ui5_side_navigation_sub_item = {
            dom_type: web_sys::HtmlElement;
            attributes {
                expanded: bool,
                icon: Icon,
                selected: bool,
                text: String,
            };
        }
    );
}

pub fn side_navigation<Id>() -> SideNavigation<Id> {
    SideNavigation(ui5_side_navigation(), PhantomData)
}

#[derive(ElementBuilder)]
pub struct SideNavigation<Id>(Ui5SideNavigation, PhantomData<Id>);

impl<Id> SideNavigation<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn collapsed(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.collapsed(value), PhantomData)
    }

    // We don't include a `child` method as they're not so useful when the item type
    // is specific.
    pub fn children(self, children: impl IntoIterator<Item = SideNavigationItem<Id>>) -> Self {
        Self(self.0.children(children), PhantomData)
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = SideNavigationItem<Id>> + 'static,
    ) -> Self {
        Self(self.0.children_signal(children), PhantomData)
    }

    pub fn on_selection_change(
        self,
        mut f: impl FnMut(CustomEvent<SelectionChangeDetail>, Id) + 'static,
    ) -> Self {
        Self(
            self.0.on_selection_change(move |event, _target| {
                let id = event
                    .detail()
                    .item()
                    .get_attribute(SELECTED_ID)
                    .unwrap_throw()
                    .parse()
                    .unwrap_throw();
                f(event, id)
            }),
            PhantomData,
        )
    }
}

impl<Id> Value for SideNavigation<Id> {}

impl<Id> HtmlElement for SideNavigation<Id> {}

impl<Id> HtmlElementEvents for SideNavigation<Id> {}

impl<Id> ElementEvents for SideNavigation<Id> {}

impl<T> From<SideNavigation<T>> for Node {
    fn from(builder: SideNavigation<T>) -> Self {
        builder.0.into()
    }
}

pub fn item<Id: Display>(id: Id) -> SideNavigationItem<Id> {
    SideNavigationItem(
        ui5_side_navigation_item().attribute(SELECTED_ID, &id.to_string()),
        PhantomData,
    )
}

#[derive(ElementBuilder)]
pub struct SideNavigationItem<Id>(Ui5SideNavigationItem, PhantomData<Id>);

impl<Id> SideNavigationItem<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn expanded(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.expanded(value), PhantomData)
    }

    pub fn icon(self, value: impl SignalOrValue<Item = Icon> + 'static) -> Self {
        Self(self.0.icon(value), PhantomData)
    }

    pub fn selected(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.selected(value), PhantomData)
    }

    pub fn whole_item_toggleable(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.whole_item_toggleable(value), PhantomData)
    }

    pub fn text(self, value: impl SignalOrValue<Item = impl AsAttribute<String>>) -> Self {
        Self(self.0.text(value), PhantomData)
    }

    // We don't include `child` and `child_signal` methods as they're not so useful
    // when the item type is specific.
    pub fn children(self, children: impl IntoIterator<Item = SideNavigationSubItem<Id>>) -> Self {
        Self(self.0.children(children), PhantomData)
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = SideNavigationSubItem<Id>> + 'static,
    ) -> Self {
        Self(self.0.children_signal(children), PhantomData)
    }
}

impl<Id> HtmlElement for SideNavigationItem<Id> {}

impl<Id> HtmlElementEvents for SideNavigationItem<Id> {}

impl<Id> ElementEvents for SideNavigationItem<Id> {}

impl<T> From<SideNavigationItem<T>> for Node {
    fn from(builder: SideNavigationItem<T>) -> Self {
        builder.0.into()
    }
}

pub fn sub_item<Id: Display>(id: Id) -> SideNavigationSubItem<Id> {
    SideNavigationSubItem(
        ui5_side_navigation_sub_item().attribute(SELECTED_ID, &id.to_string()),
        PhantomData,
    )
}

#[derive(ElementBuilder)]
pub struct SideNavigationSubItem<Id>(Ui5SideNavigationSubItem, PhantomData<Id>);

impl<Id> SideNavigationSubItem<Id>
where
    Id: FromStr,
    Id::Err: Debug,
{
    pub fn expanded(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.expanded(value), PhantomData)
    }

    pub fn icon(self, value: impl SignalOrValue<Item = Icon> + 'static) -> Self {
        Self(self.0.icon(value), PhantomData)
    }

    pub fn selected(self, value: impl SignalOrValue<Item = bool> + 'static) -> Self {
        Self(self.0.selected(value), PhantomData)
    }

    pub fn text(self, value: impl SignalOrValue<Item = String> + 'static) -> Self {
        Self(self.0.text(value), PhantomData)
    }
}

impl<Id> HtmlElement for SideNavigationSubItem<Id> {}

impl<Id> HtmlElementEvents for SideNavigationSubItem<Id> {}

impl<Id> ElementEvents for SideNavigationSubItem<Id> {}

impl<T> From<SideNavigationSubItem<T>> for Node {
    fn from(builder: SideNavigationSubItem<T>) -> Self {
        builder.0.into()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type SelectionChangeDetail;

    #[wasm_bindgen(method, getter)]
    pub fn item(this: &SelectionChangeDetail) -> web_sys::HtmlElement;
}
