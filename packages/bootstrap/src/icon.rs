use derive_more::Into;
use silkenweb::{
    elements::html::{i, IBuilder},
    node::{
        element::{Element, ElementBuilder},
        Node,
    },
    value::SignalOrValue,
    ElementBuilder, ElementEvents, HtmlElementEvents, Value,
};
use silkenweb_bootstrap_macros::define_icons;

use crate::{
    utility::{Colour, SetSpacing},
    Class,
};

pub mod css {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons-1.9.1/bootstrap-icons.css");
}

pub fn icon(icon: impl SignalOrValue<Item = IconType>) -> IconBuilder {
    IconBuilder(i().class(icon.map(IconType::class)))
}

#[derive(Value, ElementBuilder, ElementEvents, HtmlElementEvents, Into)]
pub struct IconBuilder(IBuilder);

impl IconBuilder {
    pub fn colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(Colour::text))
    }
}

impl SetSpacing for IconBuilder {}

impl From<IconBuilder> for Element {
    fn from(icon: IconBuilder) -> Self {
        icon.0.into()
    }
}

impl From<IconBuilder> for Node {
    fn from(icon: IconBuilder) -> Self {
        icon.0.into()
    }
}

#[derive(Into, Value)]
pub struct Icon(Element);

impl From<Icon> for Node {
    fn from(elem: Icon) -> Self {
        elem.0.into()
    }
}

impl From<IconBuilder> for Icon {
    fn from(icon: IconBuilder) -> Self {
        Self(icon.0.into())
    }
}

define_icons!("bootstrap-icons-1.9.1/bootstrap-icons.css");
