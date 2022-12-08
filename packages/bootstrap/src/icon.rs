use derive_more::Into;
use silkenweb::{
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
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons-1.9.1/bootstrap-icons.css");
}

pub fn icon(icon: impl SignalOrValue<Item = IconType>) -> Icon {
    Icon(i().class(icon.map(IconType::class)))
}

#[derive(Value, Element, ElementEvents, HtmlElementEvents, Into)]
pub struct Icon(I);

impl Icon {
    pub fn colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(Colour::text))
    }
}

impl SetSpacing for Icon {}

impl From<Icon> for GenericElement {
    fn from(icon: Icon) -> Self {
        icon.0.into()
    }
}

impl From<Icon> for Node {
    fn from(icon: Icon) -> Self {
        icon.0.into()
    }
}

define_icons!("bootstrap-icons-1.9.1/bootstrap-icons.css");
