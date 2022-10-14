use silkenweb::node::element::ElementBuilder;

use crate::{css, Class};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Colour {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Colour {
    pub fn text_background(self) -> Class {
        match self {
            Self::Primary => css::TEXT_BG_PRIMARY,
            Self::Secondary => css::TEXT_BG_SECONDARY,
            Self::Success => css::TEXT_BG_SUCCESS,
            Self::Danger => css::TEXT_BG_DANGER,
            Self::Warning => css::TEXT_BG_WARNING,
            Self::Info => css::TEXT_BG_INFO,
            Self::Light => css::TEXT_BG_LIGHT,
            Self::Dark => css::TEXT_BG_DARK,
        }
    }

    pub fn border(self) -> Class {
        match self {
            Colour::Primary => css::BORDER_PRIMARY,
            Colour::Secondary => css::BORDER_SECONDARY,
            Colour::Success => css::BORDER_SUCCESS,
            Colour::Danger => css::BORDER_DANGER,
            Colour::Warning => css::BORDER_WARNING,
            Colour::Info => css::BORDER_INFO,
            Colour::Light => css::BORDER_LIGHT,
            Colour::Dark => css::BORDER_DARK,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Start,
    End,
}

impl Side {
    pub fn margin(self) -> Class {
        match self {
            Side::Top => css::MT_AUTO,
            Side::Bottom => css::MB_AUTO,
            Side::Start => css::MS_AUTO,
            Side::End => css::ME_AUTO,
        }
    }

    pub fn rounded(self) -> Class {
        match self {
            Side::Top => css::ROUNDED_TOP,
            Side::Bottom => css::ROUNDED_BOTTOM,
            Side::Start => css::ROUNDED_START,
            Side::End => css::ROUNDED_END,
        }
    }
}

pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn margin(self) -> Class {
        match self {
            Axis::X => css::MX_AUTO,
            Axis::Y => css::MY_AUTO,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Size {
    Size0,
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
}

impl Size {
    pub fn margin(self) -> Class {
        match self {
            Size::Size0 => css::M_0,
            Size::Size1 => css::M_1,
            Size::Size2 => css::M_2,
            Size::Size3 => css::M_3,
            Size::Size4 => css::M_4,
            Size::Size5 => css::M_5,
        }
    }

    pub fn padding(self) -> Class {
        match self {
            Size::Size0 => css::P_0,
            Size::Size1 => css::P_1,
            Size::Size2 => css::P_2,
            Size::Size3 => css::P_3,
            Size::Size4 => css::P_4,
            Size::Size5 => css::P_5,
        }
    }

    pub fn border_width(self) -> Class {
        match self {
            Size::Size0 => css::BORDER_0,
            Size::Size1 => css::BORDER_1,
            Size::Size2 => css::BORDER_2,
            Size::Size3 => css::BORDER_3,
            Size::Size4 => css::BORDER_4,
            Size::Size5 => css::BORDER_5,
        }
    }

    pub fn rounded(self) -> Class {
        match self {
            Size::Size0 => css::ROUNDED_0,
            Size::Size1 => css::ROUNDED_1,
            Size::Size2 => css::ROUNDED_2,
            Size::Size3 => css::ROUNDED_3,
            Size::Size4 => css::ROUNDED_4,
            Size::Size5 => css::ROUNDED_5,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Opacity {
    Opacity10,
    Opacity25,
    Opacity50,
    Opacity75,
    Opacity100,
}

impl Opacity {
    pub fn border(self) -> Class {
        match self {
            Opacity::Opacity10 => css::BORDER_OPACITY_10,
            Opacity::Opacity25 => css::BORDER_OPACITY_25,
            Opacity::Opacity50 => css::BORDER_OPACITY_50,
            Opacity::Opacity75 => css::BORDER_OPACITY_75,
            Opacity::Opacity100 => css::BORDER_OPACITY_100,
        }
    }
}

pub trait Margin {
    fn margin(self) -> Class;
}

impl Margin for Option<Size> {
    fn margin(self) -> Class {
        match self {
            None => css::M_AUTO,
            Some(size) => size.margin(),
        }
    }
}

impl Margin for (Size, Side) {
    fn margin(self) -> Class {
        match self {
            (Size::Size0, Side::Top) => css::MT_0,
            (Size::Size0, Side::Bottom) => css::MB_0,
            (Size::Size0, Side::Start) => css::MS_0,
            (Size::Size0, Side::End) => css::ME_0,
            (Size::Size1, Side::Top) => css::MT_1,
            (Size::Size1, Side::Bottom) => css::MB_1,
            (Size::Size1, Side::Start) => css::MS_1,
            (Size::Size1, Side::End) => css::ME_1,
            (Size::Size2, Side::Top) => css::MT_2,
            (Size::Size2, Side::Bottom) => css::MB_2,
            (Size::Size2, Side::Start) => css::MS_2,
            (Size::Size2, Side::End) => css::ME_2,
            (Size::Size3, Side::Top) => css::MT_3,
            (Size::Size3, Side::Bottom) => css::MB_3,
            (Size::Size3, Side::Start) => css::MS_3,
            (Size::Size3, Side::End) => css::ME_3,
            (Size::Size4, Side::Top) => css::MT_4,
            (Size::Size4, Side::Bottom) => css::MB_4,
            (Size::Size4, Side::Start) => css::MS_4,
            (Size::Size4, Side::End) => css::ME_4,
            (Size::Size5, Side::Top) => css::MT_5,
            (Size::Size5, Side::Bottom) => css::MB_5,
            (Size::Size5, Side::Start) => css::MS_5,
            (Size::Size5, Side::End) => css::ME_5,
        }
    }
}

impl Margin for (Option<Size>, Side) {
    fn margin(self) -> Class {
        match self {
            (None, side) => side.margin(),
            (Some(size), side) => (size, side).margin(),
        }
    }
}

impl Margin for (Size, Axis) {
    fn margin(self) -> Class {
        match self {
            (Size::Size0, Axis::X) => css::MX_0,
            (Size::Size0, Axis::Y) => css::MY_0,
            (Size::Size1, Axis::X) => css::MX_1,
            (Size::Size1, Axis::Y) => css::MY_1,
            (Size::Size2, Axis::X) => css::MX_2,
            (Size::Size2, Axis::Y) => css::MY_2,
            (Size::Size3, Axis::X) => css::MX_3,
            (Size::Size3, Axis::Y) => css::MY_3,
            (Size::Size4, Axis::X) => css::MX_4,
            (Size::Size4, Axis::Y) => css::MY_4,
            (Size::Size5, Axis::X) => css::MX_5,
            (Size::Size5, Axis::Y) => css::MY_5,
        }
    }
}

impl Margin for (Option<Size>, Axis) {
    fn margin(self) -> Class {
        match self {
            (None, axis) => axis.margin(),
            (Some(size), axis) => (size, axis).margin(),
        }
    }
}

pub trait Padding {
    fn padding(self) -> Class;
}

impl Padding for (Size, Side) {
    fn padding(self) -> Class {
        match self {
            (Size::Size0, Side::Top) => css::PT_0,
            (Size::Size0, Side::Bottom) => css::PB_0,
            (Size::Size0, Side::Start) => css::PS_0,
            (Size::Size0, Side::End) => css::PE_0,
            (Size::Size1, Side::Top) => css::PT_1,
            (Size::Size1, Side::Bottom) => css::PB_1,
            (Size::Size1, Side::Start) => css::PS_1,
            (Size::Size1, Side::End) => css::PE_1,
            (Size::Size2, Side::Top) => css::PT_2,
            (Size::Size2, Side::Bottom) => css::PB_2,
            (Size::Size2, Side::Start) => css::PS_2,
            (Size::Size2, Side::End) => css::PE_2,
            (Size::Size3, Side::Top) => css::PT_3,
            (Size::Size3, Side::Bottom) => css::PB_3,
            (Size::Size3, Side::Start) => css::PS_3,
            (Size::Size3, Side::End) => css::PE_3,
            (Size::Size4, Side::Top) => css::PT_4,
            (Size::Size4, Side::Bottom) => css::PB_4,
            (Size::Size4, Side::Start) => css::PS_4,
            (Size::Size4, Side::End) => css::PE_4,
            (Size::Size5, Side::Top) => css::PT_5,
            (Size::Size5, Side::Bottom) => css::PB_5,
            (Size::Size5, Side::Start) => css::PS_5,
            (Size::Size5, Side::End) => css::PE_5,
        }
    }
}

impl Padding for (Size, Axis) {
    fn padding(self) -> Class {
        match self {
            (Size::Size0, Axis::X) => css::PX_0,
            (Size::Size0, Axis::Y) => css::PY_0,
            (Size::Size1, Axis::X) => css::PX_1,
            (Size::Size1, Axis::Y) => css::PY_1,
            (Size::Size2, Axis::X) => css::PX_2,
            (Size::Size2, Axis::Y) => css::PY_2,
            (Size::Size3, Axis::X) => css::PX_3,
            (Size::Size3, Axis::Y) => css::PY_3,
            (Size::Size4, Axis::X) => css::PX_4,
            (Size::Size4, Axis::Y) => css::PY_4,
            (Size::Size5, Axis::X) => css::PX_5,
            (Size::Size5, Axis::Y) => css::PY_5,
        }
    }
}

pub trait Spacing: ElementBuilder {
    // TODO: signal equivalent functions

    /// Set the margin size
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin(self, size: Option<Size>) -> Self {
        self.class([size.margin()])
    }

    /// Set the margin on `side`
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin_on(self, size: Option<Size>, side: Side) -> Self {
        self.class([(size, side).margin()])
    }

    /// Set the margin on `axis`
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin_on_axis(self, size: Option<Size>, axis: Axis) -> Self {
        self.class([(size, axis).margin()])
    }

    /// Set the padding
    fn padding(self, size: Size) -> Self {
        self.class([size.padding()])
    }

    /// Set the padding on `side`
    fn padding_on(self, size: Size, side: Side) -> Self {
        self.class([(size, side).padding()])
    }

    /// Set the padding on `axis`
    fn padding_on_axis(self, size: Size, axis: Axis) -> Self {
        self.class([(size, axis).padding()])
    }

    /// Use a border
    fn border(self) -> Self {
        self.class([css::BORDER])
    }

    fn border_colour(self, colour: Colour) -> Self {
        self.class([colour.border()])
    }

    /// Set the border width
    fn border_width(self, size: Size) -> Self {
        self.class([size.border_width()])
    }

    /// Set a border with rounded corners
    fn border_rounded(self) -> Self {
        self.class([css::ROUNDED])
    }

    /// Set a border with rounded edges, with the radius of `size`
    fn border_rounded_size(self, size: Size) -> Self {
        self.class([size.rounded()])
    }

    /// Set a pill shaped border
    fn border_pill(self) -> Self {
        self.class([css::ROUNDED_PILL])
    }

    /// Set a circular/elliptical border
    fn border_circle(self) -> Self {
        self.class([css::ROUNDED_CIRCLE])
    }

    /// Set a border with rounded edges on a particular `side`
    fn border_rounded_on_side(self, side: Side) -> Self {
        self.class([side.rounded()])
    }

    fn border_opacity(self, opacity: Opacity) -> Self {
        self.class([opacity.border()])
    }
}

impl<T: ElementBuilder> Spacing for T {}
