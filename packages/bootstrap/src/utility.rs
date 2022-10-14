use silkenweb::node::element::ElementBuilder;

use crate::css::{self, M_AUTO};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Start,
    End,
    X,
    Y,
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

pub trait Margin {
    fn margin(self) -> &'static str;
}

impl Margin for Size {
    fn margin(self) -> &'static str {
        match self {
            Size::Size0 => css::M_0,
            Size::Size1 => css::M_1,
            Size::Size2 => css::M_2,
            Size::Size3 => css::M_3,
            Size::Size4 => css::M_4,
            Size::Size5 => css::M_5,
        }
    }
}

impl Margin for Option<Size> {
    fn margin(self) -> &'static str {
        match self {
            None => M_AUTO,
            Some(size) => size.margin(),
        }
    }
}

impl Margin for (Size, Side) {
    fn margin(self) -> &'static str {
        match self {
            (Size::Size0, Side::Top) => css::MT_0,
            (Size::Size0, Side::Bottom) => css::MB_0,
            (Size::Size0, Side::Start) => css::MS_0,
            (Size::Size0, Side::End) => css::ME_0,
            (Size::Size0, Side::X) => css::MX_0,
            (Size::Size0, Side::Y) => css::MY_0,
            (Size::Size1, Side::Top) => css::MT_1,
            (Size::Size1, Side::Bottom) => css::MB_1,
            (Size::Size1, Side::Start) => css::MS_1,
            (Size::Size1, Side::End) => css::ME_1,
            (Size::Size1, Side::X) => css::MX_1,
            (Size::Size1, Side::Y) => css::MY_1,
            (Size::Size2, Side::Top) => css::MT_2,
            (Size::Size2, Side::Bottom) => css::MB_2,
            (Size::Size2, Side::Start) => css::MS_2,
            (Size::Size2, Side::End) => css::ME_2,
            (Size::Size2, Side::X) => css::MX_2,
            (Size::Size2, Side::Y) => css::MY_2,
            (Size::Size3, Side::Top) => css::MT_3,
            (Size::Size3, Side::Bottom) => css::MB_3,
            (Size::Size3, Side::Start) => css::MS_3,
            (Size::Size3, Side::End) => css::ME_3,
            (Size::Size3, Side::X) => css::MX_3,
            (Size::Size3, Side::Y) => css::MY_3,
            (Size::Size4, Side::Top) => css::MT_4,
            (Size::Size4, Side::Bottom) => css::MB_4,
            (Size::Size4, Side::Start) => css::MS_4,
            (Size::Size4, Side::End) => css::ME_4,
            (Size::Size4, Side::X) => css::MX_4,
            (Size::Size4, Side::Y) => css::MY_4,
            (Size::Size5, Side::Top) => css::MT_5,
            (Size::Size5, Side::Bottom) => css::MB_5,
            (Size::Size5, Side::Start) => css::MS_5,
            (Size::Size5, Side::End) => css::ME_5,
            (Size::Size5, Side::X) => css::MX_5,
            (Size::Size5, Side::Y) => css::MY_5,
        }
    }
}

impl Margin for (Option<Size>, Side) {
    fn margin(self) -> &'static str {
        match self {
            (None, side) => match side {
                Side::Top => css::MT_AUTO,
                Side::Bottom => css::MB_AUTO,
                Side::Start => css::MS_AUTO,
                Side::End => css::ME_AUTO,
                Side::X => css::MX_AUTO,
                Side::Y => css::MY_AUTO,
            },
            (Some(size), side) => (size, side).margin(),
        }
    }
}

pub trait Padding {
    fn padding(self) -> &'static str;
}

impl Padding for Size {
    fn padding(self) -> &'static str {
        match self {
            Size::Size0 => css::P_0,
            Size::Size1 => css::P_1,
            Size::Size2 => css::P_2,
            Size::Size3 => css::P_3,
            Size::Size4 => css::P_4,
            Size::Size5 => css::P_5,
        }
    }
}

impl Padding for (Size, Side) {
    fn padding(self) -> &'static str {
        match self {
            (Size::Size0, Side::Top) => css::PT_0,
            (Size::Size0, Side::Bottom) => css::PB_0,
            (Size::Size0, Side::Start) => css::PS_0,
            (Size::Size0, Side::End) => css::PE_0,
            (Size::Size0, Side::X) => css::PX_0,
            (Size::Size0, Side::Y) => css::PY_0,
            (Size::Size1, Side::Top) => css::PT_1,
            (Size::Size1, Side::Bottom) => css::PB_1,
            (Size::Size1, Side::Start) => css::PS_1,
            (Size::Size1, Side::End) => css::PE_1,
            (Size::Size1, Side::X) => css::PX_1,
            (Size::Size1, Side::Y) => css::PY_1,
            (Size::Size2, Side::Top) => css::PT_2,
            (Size::Size2, Side::Bottom) => css::PB_2,
            (Size::Size2, Side::Start) => css::PS_2,
            (Size::Size2, Side::End) => css::PE_2,
            (Size::Size2, Side::X) => css::PX_2,
            (Size::Size2, Side::Y) => css::PY_2,
            (Size::Size3, Side::Top) => css::PT_3,
            (Size::Size3, Side::Bottom) => css::PB_3,
            (Size::Size3, Side::Start) => css::PS_3,
            (Size::Size3, Side::End) => css::PE_3,
            (Size::Size3, Side::X) => css::PX_3,
            (Size::Size3, Side::Y) => css::PY_3,
            (Size::Size4, Side::Top) => css::PT_4,
            (Size::Size4, Side::Bottom) => css::PB_4,
            (Size::Size4, Side::Start) => css::PS_4,
            (Size::Size4, Side::End) => css::PE_4,
            (Size::Size4, Side::X) => css::PX_4,
            (Size::Size4, Side::Y) => css::PY_4,
            (Size::Size5, Side::Top) => css::PT_5,
            (Size::Size5, Side::Bottom) => css::PB_5,
            (Size::Size5, Side::Start) => css::PS_5,
            (Size::Size5, Side::End) => css::PE_5,
            (Size::Size5, Side::X) => css::PX_5,
            (Size::Size5, Side::Y) => css::PY_5,
        }
    }
}

pub trait Spacing: ElementBuilder {
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

    /// Set the padding
    fn padding(self, size: Size) -> Self {
        self.class([size.padding()])
    }

    /// Set the padding on `side`
    fn padding_on(self, size: Size, side: Side) -> Self {
        self.class([(size, side).padding()])
    }
}

impl<T: ElementBuilder> Spacing for T {}
