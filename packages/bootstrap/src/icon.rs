use derive_more::Into;
use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    elements::html::{i, IBuilder},
    node::{element::ElementBuilder, Node},
    prelude::{ElementEvents, HtmlElementEvents},
    ElementBuilder,
};

use crate::{utility::SetSpacing, Class};

pub mod css {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons-1.9.1/bootstrap-icons.css");
}

#[derive(ElementBuilder, Into)]
pub struct IconBuilder(IBuilder);

pub fn icon(icon: IconType) -> IconBuilder {
    IconBuilder(i().class([icon.class()]))
}

pub fn icon_signal(icon: impl Signal<Item = IconType> + 'static) -> IconBuilder {
    IconBuilder(i().class_signal(icon.map(|icon| [icon.class()])))
}

impl ElementEvents for IconBuilder {}
impl HtmlElementEvents for IconBuilder {}
impl SetSpacing for IconBuilder {}

impl From<IconBuilder> for Node {
    fn from(badge: IconBuilder) -> Self {
        badge.0.into()
    }
}

#[derive(Into)]
pub struct Icon(Node);

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
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IconType {
    Circle,
    PlayCircleFill,
    CheckCircleFill,
    ExclamationTriangleFill,
}

impl IconType {
    pub fn class(self) -> Class {
        match self {
            Self::Circle => css::BI_CIRCLE,
            Self::PlayCircleFill => css::BI_PLAY_CIRCLE_FILL,
            Self::CheckCircleFill => css::BI_CHECK_CIRCLE_FILL,
            Self::ExclamationTriangleFill => css::BI_EXCLAMATION_TRIANGLE_FILL,
        }
    }
}
