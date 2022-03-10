use std::borrow::Cow;

use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    elements::html::Img,
    node::element::{ElementBuilder, ParentBuilder},
    ElementBuilder,
};

use crate::icon::Icon;

use self::elements::ui5_avatar;

pub enum ColorScheme {
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Accent7,
    Accent8,
    Accent9,
    Accent10,
}

impl Attribute for ColorScheme {
    fn text(&self) -> Option<Cow<str>> {
        Some(
            match self {
                ColorScheme::Accent1 => "Accent1",
                ColorScheme::Accent2 => "Accent2",
                ColorScheme::Accent3 => "Accent3",
                ColorScheme::Accent4 => "Accent4",
                ColorScheme::Accent5 => "Accent5",
                ColorScheme::Accent6 => "Accent6",
                ColorScheme::Accent7 => "Accent7",
                ColorScheme::Accent8 => "Accent8",
                ColorScheme::Accent9 => "Accent9",
                ColorScheme::Accent10 => "Accent10",
            }
            .into(),
        )
    }
}

impl AsAttribute<ColorScheme> for ColorScheme {}

pub enum Shape {
    Circle,
    Square,
}

impl Attribute for Shape {
    fn text(&self) -> Option<Cow<str>> {
        Some(
            match self {
                Shape::Circle => "Circle",
                Shape::Square => "Square",
            }
            .into(),
        )
    }
}

impl AsAttribute<Shape> for Shape {}

pub enum Size {
    Xs,
    S,
    M,
    L,
    Xl,
}

impl Attribute for Size {
    fn text(&self) -> Option<Cow<str>> {
        Some(
            match self {
                Size::Xs => "Xs",
                Size::S => "S",
                Size::M => "M",
                Size::L => "L",
                Size::Xl => "Xl",
            }
            .into(),
        )
    }
}

impl AsAttribute<Size> for Size {}

mod elements {
    use silkenweb::{html_element, parent_element};

    use super::{ColorScheme, Shape, Size};
    use crate::icon::Icon;

    html_element!(
        ui5-avatar<web_sys::HtmlElement> {
            attributes {
                accessible-name: String,
                color-scheme: ColorScheme,
                icon: Icon,
                initials: String,
                interactive: bool,
                shape: Shape,
                size: Size,
            }
        }
    );

    parent_element!(ui5 - avatar);
}

pub type Avatar = elements::Ui5Avatar;

pub fn avatar() -> AvatarBuilder {
    AvatarBuilder(ui5_avatar())
}

#[derive(ElementBuilder)]
pub struct AvatarBuilder(elements::Ui5AvatarBuilder);

impl AvatarBuilder {
    pub fn accessible_name(self, value: &str) -> Self {
        Self(self.0.accessible_name(value))
    }

    pub fn accessible_name_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.accessible_name_signal(value))
    }

    pub fn color_scheme(self, value: ColorScheme) -> Self {
        Self(self.0.color_scheme(value))
    }

    pub fn color_scheme_signal(self, value: impl Signal<Item = ColorScheme> + 'static) -> Self {
        Self(self.0.color_scheme_signal(value))
    }

    pub fn icon(self, value: Icon) -> Self {
        Self(self.0.icon(value))
    }

    pub fn icon_signal(self, value: impl Signal<Item = Icon> + 'static) -> Self {
        Self(self.0.icon_signal(value))
    }

    pub fn initials(self, value: &str) -> Self {
        Self(self.0.initials(value))
    }

    pub fn initials_signal(self, value: impl Signal<Item = String> + 'static) -> Self {
        Self(self.0.initials_signal(value))
    }

    pub fn interactive(self, value: bool) -> Self {
        Self(self.0.interactive(value))
    }

    pub fn interactive_signal(self, value: impl Signal<Item = bool> + 'static) -> Self {
        Self(self.0.interactive_signal(value))
    }

    pub fn shape(self, value: Shape) -> Self {
        Self(self.0.shape(value))
    }

    pub fn shape_signal(self, value: impl Signal<Item = Shape> + 'static) -> Self {
        Self(self.0.shape_signal(value))
    }

    pub fn size(self, value: Size) -> Self {
        Self(self.0.size(value))
    }

    pub fn size_signal(self, value: impl Signal<Item = Size> + 'static) -> Self {
        Self(self.0.size_signal(value))
    }

    pub fn image(self, image: impl Into<Img>) -> Avatar {
        self.0.child(image.into()).build()
    }

    pub fn image_signal(self, image: impl Signal<Item = impl Into<Img>> + 'static) -> Avatar {
        self.0.child_signal(image.map(|img| img.into()))
    }
}
