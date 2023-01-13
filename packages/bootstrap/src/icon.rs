use derive_more::Into;
use silkenweb::{
    dom::{DefaultDom, Dom},
    elements::html::{i, I},
    node::{
        element::{Element, GenericElement},
        Node,
    },
    value::SignalOrValue,
    Element, ElementEvents, HtmlElementEvents, Value,
};
use silkenweb_bootstrap_macros::define_icons;

use crate::{
    utility::{Colour, SetSpacing},
    Class,
};

pub mod css {
    silkenweb::css!(visibility = pub, path = "bootstrap-icons-1.9.1/bootstrap-icons.css");
}

pub fn icon(icon: impl SignalOrValue<Item = IconType>) -> Icon {
    Icon(i().class(icon.map(IconType::class)))
}

#[derive(Value, Element, ElementEvents, HtmlElementEvents, Into)]
pub struct Icon<D: Dom = DefaultDom>(I<D>);

impl<D: Dom> Icon<D> {
    pub fn colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(Colour::text))
    }
}

impl<D: Dom> SetSpacing for Icon<D> {}

impl<D: Dom> From<Icon<D>> for GenericElement<D> {
    fn from(icon: Icon<D>) -> Self {
        icon.0.into()
    }
}

impl<D: Dom> From<Icon<D>> for Node<D> {
    fn from(icon: Icon<D>) -> Self {
        icon.0.into()
    }
}

define_icons!("bootstrap-icons-1.9.1/bootstrap-icons.css");
