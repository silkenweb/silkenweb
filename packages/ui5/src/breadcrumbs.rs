use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    elements::CustomEvent,
    node::{element::{ElementBuilder, GenericElement}, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

use self::element::{ui5_breadcrumbs, ui5_breadcrumbs_item, Ui5Breadcrumbs, Ui5BreadcrumbsItem};
use crate::{macros::attributes1, SELECTED_ID};

#[derive(Copy, Clone, Display, Eq, PartialEq)]
pub enum BreadcrumbsDesign {
    Standard,
    NoCurrentPage,
}

impl Attribute for BreadcrumbsDesign {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<BreadcrumbsDesign> for BreadcrumbsDesign {}

#[derive(Copy, Clone, Display, Eq, PartialEq)]
pub enum BreadcrumbsSeparatorStyle {
    Slash,
    BackSlash,
    DoubleBackSlash,
    DoubleGreaterThan,
    DoubleSlash,
    GreaterThan,
}

impl Attribute for BreadcrumbsSeparatorStyle {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<BreadcrumbsSeparatorStyle> for BreadcrumbsSeparatorStyle {}

#[derive(Copy, Clone, Display, Eq, PartialEq)]
pub enum BreadcrumbsTarget {
    SelfTarget,
    Top,
    Blank,
    Parent,
    Search,
}

impl Attribute for BreadcrumbsTarget {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(
            match &self {
                BreadcrumbsTarget::SelfTarget => "_self",
                BreadcrumbsTarget::Top => "_top",
                BreadcrumbsTarget::Blank => "_blank",
                BreadcrumbsTarget::Parent => "_parent",
                BreadcrumbsTarget::Search => "_search",
            }
            .into(),
        )
    }
}

impl AsAttribute<BreadcrumbsTarget> for BreadcrumbsTarget {}

mod element {
    use silkenweb::{custom_html_element, elements::CustomEvent, parent_element};

    use super::{BreadcrumbsDesign, BreadcrumbsSeparatorStyle, BreadcrumbsTarget, ItemClickDetail};

    custom_html_element!(
        ui5_breadcrumbs = { dom_type: web_sys::HtmlElement;
            attributes {
                design: BreadcrumbsDesign,
                separator_style: BreadcrumbsSeparatorStyle,
            };

            custom_events {
                item_click: CustomEvent<ItemClickDetail>
            };
        }
    );

    parent_element!(ui5_breadcrumbs);

    custom_html_element!(
        ui5_breadcrumbs_item = {
            dom_type: web_sys::HtmlElement;
            attributes {
                accessible_name: String,
                href: String,
                target: BreadcrumbsTarget,
            };
        }
    );

    parent_element!(ui5_breadcrumbs_item);
}

pub fn breadcrumbs<T>() -> Breadcrumbs<T> {
    Breadcrumbs(ui5_breadcrumbs(), PhantomData)
}

#[derive(ElementBuilder)]
pub struct Breadcrumbs<Id>(Ui5Breadcrumbs, PhantomData<Id>);

impl<Id> Breadcrumbs<Id>
where
    Id: Display + FromStr,
    Id::Err: Debug,
{
    attributes1! {
        1,
        design: BreadcrumbsDesign,
        separator_style: BreadcrumbsSeparatorStyle
    }

    pub fn children(
        self,
        children: impl IntoIterator<Item = (Id, BreadcrumbsItem)>,
    ) -> Self {
        Self(
            self.0.children(
                children
                    .into_iter()
                    .map(|(id, item)| item.attribute(SELECTED_ID, id.to_string())),
            ),
            PhantomData,
        )
    }

    pub fn children_signal(
        self,
        children: impl SignalVec<Item = (Id, BreadcrumbsItem)> + 'static,
    ) -> Self {
        Self(
            self.0.children_signal(
                children.map(|(id, item)| item.attribute(SELECTED_ID, id.to_string())),
            ),
            PhantomData,
        )
    }

    pub fn on_item_click(
        self,
        mut f: impl FnMut(CustomEvent<ItemClickDetail>, Id) + 'static,
    ) -> Self {
        Self(
            self.0.on_item_click(move |event, _target| {
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

impl<Id> HtmlElement for Breadcrumbs<Id> {}

impl<Id> HtmlElementEvents for Breadcrumbs<Id> {}

impl<Id> ElementEvents for Breadcrumbs<Id> {}

impl<T> From<Breadcrumbs<T>> for GenericElement {
    fn from(builder: Breadcrumbs<T>) -> Self {
        builder.0.into()
    }
}

impl<T> From<Breadcrumbs<T>> for Node {
    fn from(builder: Breadcrumbs<T>) -> Self {
        builder.0.into()
    }
}

pub fn breadcrumbs_item() -> BreadcrumbsItem {
    ui5_breadcrumbs_item()
}

pub type BreadcrumbsItem = Ui5BreadcrumbsItem;

#[wasm_bindgen]
extern "C" {
    pub type ItemClickDetail;

    #[wasm_bindgen(method, getter)]
    pub fn item(this: &ItemClickDetail) -> web_sys::HtmlElement;
}
