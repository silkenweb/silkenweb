use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    node::element::{ElementBuilder, Sig},
    prelude::{HtmlElement, ParentBuilder},
};

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

    pub fn background(self) -> Class {
        match self {
            Self::Primary => css::BG_PRIMARY,
            Self::Secondary => css::BG_SECONDARY,
            Self::Success => css::BG_SUCCESS,
            Self::Danger => css::BG_DANGER,
            Self::Warning => css::BG_WARNING,
            Self::Info => css::BG_INFO,
            Self::Light => css::BG_LIGHT,
            Self::Dark => css::BG_DARK,
        }
    }

    pub fn text(self) -> Class {
        match self {
            Self::Primary => css::TEXT_PRIMARY,
            Self::Secondary => css::TEXT_SECONDARY,
            Self::Success => css::TEXT_SUCCESS,
            Self::Danger => css::TEXT_DANGER,
            Self::Warning => css::TEXT_WARNING,
            Self::Info => css::TEXT_INFO,
            Self::Light => css::TEXT_LIGHT,
            Self::Dark => css::TEXT_DARK,
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

    pub fn button(self, outline: bool) -> Class {
        match (outline, self) {
            (false, Colour::Primary) => css::BTN_PRIMARY,
            (false, Colour::Secondary) => css::BTN_SECONDARY,
            (false, Colour::Success) => css::BTN_SUCCESS,
            (false, Colour::Danger) => css::BTN_DANGER,
            (false, Colour::Warning) => css::BTN_WARNING,
            (false, Colour::Info) => css::BTN_INFO,
            (false, Colour::Light) => css::BTN_LIGHT,
            (false, Colour::Dark) => css::BTN_DARK,
            (true, Colour::Primary) => css::BTN_OUTLINE_PRIMARY,
            (true, Colour::Secondary) => css::BTN_OUTLINE_SECONDARY,
            (true, Colour::Success) => css::BTN_OUTLINE_SUCCESS,
            (true, Colour::Danger) => css::BTN_OUTLINE_DANGER,
            (true, Colour::Warning) => css::BTN_OUTLINE_WARNING,
            (true, Colour::Info) => css::BTN_OUTLINE_INFO,
            (true, Colour::Light) => css::BTN_OUTLINE_LIGHT,
            (true, Colour::Dark) => css::BTN_OUTLINE_DARK,
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

    pub fn rounded_border(self) -> Class {
        match self {
            Side::Top => css::ROUNDED_TOP,
            Side::Bottom => css::ROUNDED_BOTTOM,
            Side::Start => css::ROUNDED_START,
            Side::End => css::ROUNDED_END,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
pub enum SideOrAxis {
    Side(Side),
    Axis(Axis),
}

impl SideOrAxis {
    fn margin(self) -> Class {
        match self {
            SideOrAxis::Side(side) => side.margin(),
            SideOrAxis::Axis(axis) => axis.margin(),
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

    pub fn border(self) -> Class {
        match self {
            Size::Size0 => css::BORDER_0,
            Size::Size1 => css::BORDER_1,
            Size::Size2 => css::BORDER_2,
            Size::Size3 => css::BORDER_3,
            Size::Size4 => css::BORDER_4,
            Size::Size5 => css::BORDER_5,
        }
    }

    pub fn rounded_border(self) -> Class {
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Shadow {
    None,
    Small,
    Medium,
    Large,
}

impl Shadow {
    pub fn class(self) -> Class {
        match self {
            Shadow::None => css::SHADOW_NONE,
            Shadow::Small => css::SHADOW_SM,
            Shadow::Medium => css::SHADOW,
            Shadow::Large => css::SHADOW_LG,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Overflow {
    Auto,
    Hidden,
    Visible,
    Scroll,
}

impl Overflow {
    pub fn class(self) -> Class {
        match self {
            Overflow::Auto => css::OVERFLOW_AUTO,
            Overflow::Hidden => css::OVERFLOW_HIDDEN,
            Overflow::Visible => css::OVERFLOW_VISIBLE,
            Overflow::Scroll => css::OVERFLOW_SCROLL,
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
        let (size, side) = self;

        match size {
            None => side.margin(),
            Some(size) => (size, side).margin(),
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
        let (size, axis) = self;

        match size {
            None => axis.margin(),
            Some(size) => (size, axis).margin(),
        }
    }
}

impl Margin for (Size, SideOrAxis) {
    fn margin(self) -> Class {
        let (size, side_or_axis) = self;

        match side_or_axis {
            SideOrAxis::Side(side) => (size, side).margin(),
            SideOrAxis::Axis(axis) => (size, axis).margin(),
        }
    }
}

impl Margin for (Option<Size>, Option<SideOrAxis>) {
    fn margin(self) -> Class {
        match self {
            (None, None) => css::M_AUTO,
            (None, Some(side_or_axis)) => side_or_axis.margin(),
            (Some(size), None) => size.margin(),
            (Some(size), Some(side_or_axis)) => (size, side_or_axis).margin(),
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

impl Padding for (Size, SideOrAxis) {
    fn padding(self) -> Class {
        let (size, side_or_axis) = self;

        match side_or_axis {
            SideOrAxis::Side(side) => (size, side).padding(),
            SideOrAxis::Axis(side) => (size, side).padding(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    pub fn class(self) -> Class {
        match self {
            FlexDirection::Row => css::FLEX_ROW,
            FlexDirection::RowReverse => css::FLEX_ROW_REVERSE,
            FlexDirection::Column => css::FLEX_COLUMN,
            FlexDirection::ColumnReverse => css::FLEX_COLUMN_REVERSE,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Align {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl Align {
    pub fn align_items(self) -> Class {
        match self {
            Align::Start => css::ALIGN_ITEMS_START,
            Align::End => css::ALIGN_ITEMS_END,
            Align::Center => css::ALIGN_ITEMS_CENTER,
            Align::Baseline => css::ALIGN_ITEMS_BASELINE,
            Align::Stretch => css::ALIGN_ITEMS_STRETCH,
        }
    }

    pub fn align_self(self) -> Class {
        match self {
            Align::Start => css::ALIGN_SELF_START,
            Align::End => css::ALIGN_SELF_END,
            Align::Center => css::ALIGN_SELF_CENTER,
            Align::Baseline => css::ALIGN_SELF_BASELINE,
            Align::Stretch => css::ALIGN_SELF_STRETCH,
        }
    }
}

pub trait SetSpacing: ElementBuilder {
    /// Set the margin size
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin(self, size: Option<Size>) -> Self {
        self.class(size.margin())
    }

    /// Set the margin on `side`
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin_on(self, size: Option<Size>, side: Side) -> Self {
        self.class((size, side).margin())
    }

    /// Set the margin on `axis`
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin_on_axis(self, size: Option<Size>, axis: Axis) -> Self {
        self.class((size, axis).margin())
    }

    fn margin_signal(
        self,
        margin: impl Signal<Item = (Option<Size>, Option<SideOrAxis>)> + 'static,
    ) -> Self {
        self.class(Sig(margin.map(|m| m.margin())))
    }

    fn padding(self, size: Size) -> Self {
        self.class(size.padding())
    }

    fn padding_on(self, size: Size, side: Side) -> Self {
        self.class((size, side).padding())
    }

    fn padding_on_axis(self, size: Size, axis: Axis) -> Self {
        self.class((size, axis).padding())
    }

    fn padding_signal(self, size: Size, side_or_axis: SideOrAxis) -> Self {
        self.class((size, side_or_axis).padding())
    }
}

pub trait SetBorder: ElementBuilder {
    /// Use a border
    fn border(self) -> Self {
        self.class(css::BORDER)
    }

    fn border_signal(self, active: impl Signal<Item = bool> + 'static) -> Self {
        self.classes(Sig(active.map(|active| active.then_some(css::BORDER))))
    }

    fn border_colour(self, colour: Colour) -> Self {
        self.class(colour.border())
    }

    fn border_colour_signal(self, colour: impl Signal<Item = Colour> + 'static) -> Self {
        self.class(Sig(colour.map(|colour| colour.border())))
    }

    fn border_width(self, size: Size) -> Self {
        self.class(size.border())
    }

    fn border_width_signal(self, size: impl Signal<Item = Size> + 'static) -> Self {
        self.class(Sig(size.map(|size| size.border())))
    }

    fn rounded_border(self) -> Self {
        self.class(css::ROUNDED)
    }

    fn rounded_border_signal(self, active: impl Signal<Item = bool> + 'static) -> Self {
        self.classes(Sig(active.map(|active| active.then_some(css::ROUNDED))))
    }

    fn rounded_border_of_size(self, size: Size) -> Self {
        self.class(size.rounded_border())
    }

    fn rounded_border_of_size_signal(self, size: impl Signal<Item = Size> + 'static) -> Self {
        self.class(Sig(size.map(Size::rounded_border)))
    }

    fn rounded_pill_border(self) -> Self {
        self.class(css::ROUNDED_PILL)
    }

    fn rounded_pill_border_signal(self, active: impl Signal<Item = bool> + 'static) -> Self {
        self.classes(Sig(active.map(|active| active.then_some(css::ROUNDED_PILL))))
    }

    fn rounded_circular_border(self) -> Self {
        self.class(css::ROUNDED_CIRCLE)
    }

    // TODO: Get rid of `_signal` variants
    fn rounded_circular_border_signal(self, active: impl Signal<Item = bool> + 'static) -> Self {
        self.classes(Sig(
            active.map(|active| active.then_some(css::ROUNDED_CIRCLE))
        ))
    }

    fn rounded_border_on(self, side: Side) -> Self {
        self.class(side.rounded_border())
    }

    fn rounded_border_on_signal(self, side: impl Signal<Item = Side> + 'static) -> Self {
        self.class(Sig(side.map(|side| side.rounded_border())))
    }

    fn border_opacity(self, opacity: Opacity) -> Self {
        self.class(opacity.border())
    }

    fn border_opacity_signal(self, opacity: impl Signal<Item = Opacity> + 'static) -> Self {
        self.class(Sig(opacity.map(|opacity| opacity.border())))
    }

    /// Set a medium drop shadow
    fn shadow(self) -> Self {
        self.shadow_of_size(Shadow::Medium)
    }

    fn shadow_of_size(self, shadow: Shadow) -> Self {
        self.class(shadow.class())
    }

    fn shadow_of_size_signal(self, shadow: impl Signal<Item = Shadow> + 'static) -> Self {
        self.class(Sig(shadow.map(|shadow| shadow.class())))
    }
}

pub trait SetOverflow: ElementBuilder {
    fn overflow(self, overflow: Overflow) -> Self {
        self.class(overflow.class())
    }

    fn overflow_signal(self, overflow: impl Signal<Item = Overflow> + 'static) -> Self {
        self.class(Sig(overflow.map(|overflow| overflow.class())))
    }
}

pub trait SetColour: ElementBuilder {
    fn background_colour(self, colour: Colour) -> Self {
        self.class(colour.background())
    }

    fn background_colour_signal(self, colour: impl Signal<Item = Colour> + 'static) -> Self {
        self.class(Sig(colour.map(|colour| colour.background())))
    }

    fn text_colour(self, colour: Colour) -> Self {
        self.class(colour.text())
    }

    fn text_colour_signal(self, colour: impl Signal<Item = Colour> + 'static) -> Self {
        self.class(Sig(colour.map(|colour| colour.text())))
    }
}

pub trait SetFlex: ElementBuilder {
    /// Add `d-flex` and `flex-column` classes
    fn flex_column(self) -> Self {
        self.flex(FlexDirection::Column)
    }

    /// Add `d-flex` and `flex-row` classes
    fn flex_row(self) -> Self {
        self.flex(FlexDirection::Row)
    }

    /// Add `display: flex` and `flex-direction: <direction>` classes
    fn flex(self, direction: FlexDirection) -> Self {
        self.classes([css::D_FLEX, direction.class()])
    }

    /// Add `display: flex` and `flex-direction: <direction>` classes
    fn flex_signal(self, direction: impl Signal<Item = FlexDirection> + 'static) -> Self {
        self.class(css::D_FLEX)
            .class(Sig(direction.map(|d| d.class())))
    }

    fn align_items(self, align: Align) -> Self {
        self.class(align.align_items())
    }

    fn align_items_signal(self, align: impl Signal<Item = Align> + 'static) -> Self {
        self.class(Sig(align.map(|align| align.align_items())))
    }
}

pub trait SetAlign: ElementBuilder {
    fn align_self(self, align: Align) -> Self {
        self.class(align.align_self())
    }

    fn align_self_signal(self, align: impl Signal<Item = Align> + 'static) -> Self {
        self.class(Sig(align.map(|align| align.align_self())))
    }
}

impl<T: HtmlElement> SetSpacing for T {}
impl<T: HtmlElement> SetBorder for T {}
impl<T: ParentBuilder> SetOverflow for T {}
impl<T: HtmlElement> SetColour for T {}
impl<T: HtmlElement> SetAlign for T {}
impl<T: ParentBuilder> SetFlex for T {}
