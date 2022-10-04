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
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents, ParentBuilder},
    ElementBuilder,
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

use self::element::{
    ui5_breadcrumbs, ui5_breadcrumbs_item, Ui5Breadcrumbs, Ui5BreadcrumbsBuilder,
    Ui5BreadcrumbsItem, Ui5BreadcrumbsItemBuilder,
};
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
    use silkenweb::{elements::CustomEvent, html_element, parent_element};

    use super::{BreadcrumbsDesign, BreadcrumbsSeparatorStyle, BreadcrumbsTarget, ItemClickDetail};

    html_element!(
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

    html_element!(
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

pub type Breadcrumbs = Ui5Breadcrumbs;

pub fn breadcrumbs<T>() -> BreadcrumbsBuilder<T> {
    BreadcrumbsBuilder(ui5_breadcrumbs(), PhantomData)
}

#[derive(ElementBuilder)]
pub struct BreadcrumbsBuilder<Id>(Ui5BreadcrumbsBuilder, PhantomData<Id>);

impl<Id> BreadcrumbsBuilder<Id>
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
        children: impl IntoIterator<Item = (Id, BreadcrumbsItemBuilder)>,
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
        children: impl SignalVec<Item = (Id, BreadcrumbsItemBuilder)> + 'static,
    ) -> Ui5Breadcrumbs {
        self.0
            .children_signal(children.map(|(id, item)| item.attribute(SELECTED_ID, id.to_string())))
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

impl<Id> HtmlElement for BreadcrumbsBuilder<Id> {}

impl<Id> HtmlElementEvents for BreadcrumbsBuilder<Id> {}

impl<Id> ElementEvents for BreadcrumbsBuilder<Id> {}

impl<T> From<BreadcrumbsBuilder<T>> for Node {
    fn from(builder: BreadcrumbsBuilder<T>) -> Self {
        builder.0.into()
    }
}

pub fn breadcrumbs_item() -> BreadcrumbsItemBuilder {
    ui5_breadcrumbs_item()
}

pub type BreadcrumbsItemBuilder = Ui5BreadcrumbsItemBuilder;
pub type BreadcrumbsItem = Ui5BreadcrumbsItem;

#[wasm_bindgen]
extern "C" {
    pub type ItemClickDetail;

    #[wasm_bindgen(method, getter)]
    pub fn item(this: &ItemClickDetail) -> web_sys::HtmlElement;
}
