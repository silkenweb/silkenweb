use derive_more::Into;
use silkenweb::{
    elements::html::{i, IBuilder},
    node::{element::ElementBuilder, Node},
    value::SignalOrValue,
    ElementBuilder, ElementEvents, HtmlElementEvents, Value,
};

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

impl From<IconBuilder> for Node {
    fn from(icon: IconBuilder) -> Self {
        icon.0.into()
    }
}

#[derive(Into, Value)]
pub struct Icon(Node);

impl From<IconBuilder> for Icon {
    fn from(icon: IconBuilder) -> Self {
        icon.0.into()
    }
}

impl From<IBuilder> for Icon {
    fn from(builder: IBuilder) -> Self {
        Self(builder.into())
    }
}

// TODO: Generate all of this from a macro and the css file
impl Icon {
    pub fn circle() -> IconBuilder {
        icon(IconType::Circle)
    }

    pub fn play_circle_fill() -> IconBuilder {
        icon(IconType::PlayCircleFill)
    }

    pub fn check_circle_fill() -> IconBuilder {
        icon(IconType::CheckCircleFill)
    }

    pub fn exclamation_triangle_fill() -> IconBuilder {
        icon(IconType::ExclamationTriangleFill)
    }

    pub fn zoom_out() -> IconBuilder {
        icon(IconType::ZoomOut)
    }

    pub fn zoom_in() -> IconBuilder {
        icon(IconType::ZoomIn)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum IconType {
    Circle,
    PlayCircleFill,
    CheckCircleFill,
    ExclamationTriangleFill,
    ZoomOut,
    ZoomIn,
}

impl IconType {
    pub fn class(self) -> Class {
        match self {
            Self::Circle => css::BI_CIRCLE,
            Self::PlayCircleFill => css::BI_PLAY_CIRCLE_FILL,
            Self::CheckCircleFill => css::BI_CHECK_CIRCLE_FILL,
            Self::ExclamationTriangleFill => css::BI_EXCLAMATION_TRIANGLE_FILL,
            Self::ZoomOut => css::BI_ZOOM_OUT,
            Self::ZoomIn => css::BI_ZOOM_IN,
        }
    }
}
