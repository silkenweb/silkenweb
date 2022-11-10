use silkenweb::{
    elements::html,
    node::element::ElementBuilder,
    prelude::{HtmlElement, ParentBuilder},
    value::SignalOrValue,
    Value,
};

use crate::{css, Class};

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

    pub fn link(self) -> Class {
        match self {
            Self::Primary => css::LINK_PRIMARY,
            Self::Secondary => css::LINK_SECONDARY,
            Self::Success => css::LINK_SUCCESS,
            Self::Danger => css::LINK_DANGER,
            Self::Warning => css::LINK_WARNING,
            Self::Info => css::LINK_INFO,
            Self::Light => css::LINK_LIGHT,
            Self::Dark => css::LINK_DARK,
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

    pub fn border(self) -> Class {
        match self {
            Side::Top => css::BORDER_TOP,
            Side::Bottom => css::BORDER_BOTTOM,
            Side::Start => css::BORDER_START,
            Side::End => css::BORDER_END,
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

    pub fn gap(self) -> Class {
        match self {
            Size::Size0 => css::GAP_0,
            Size::Size1 => css::GAP_1,
            Size::Size2 => css::GAP_2,
            Size::Size3 => css::GAP_3,
            Size::Size4 => css::GAP_4,
            Size::Size5 => css::GAP_5,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    fn class(self) -> Class {
        match self {
            FlexDirection::Row => css::FLEX_ROW,
            FlexDirection::RowReverse => css::FLEX_ROW_REVERSE,
            FlexDirection::Column => css::FLEX_COLUMN,
            FlexDirection::ColumnReverse => css::FLEX_COLUMN_REVERSE,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
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

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub fn class(self) -> Class {
        match self {
            Position::Static => css::POSITION_STATIC,
            Position::Relative => css::POSITION_RELATIVE,
            Position::Absolute => css::POSITION_ABSOLUTE,
            Position::Fixed => css::POSITION_FIXED,
            Position::Sticky => css::POSITION_STICKY,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Value)]
pub enum Display {
    None,
    Inline,
    InlineBlock,
    Block,
    Grid,
    Table,
    TableCell,
    TableRow,
    Flex,
    InlineFlex,
}

impl Display {
    pub fn class(self) -> Class {
        match self {
            Display::None => "d-none",
            Display::Inline => "d-inline",
            Display::InlineBlock => "d-inline-block",
            Display::Block => "d-block",
            Display::Grid => "d-grid",
            Display::Table => "d-table",
            Display::TableCell => "d-table-cell",
            Display::TableRow => "d-table-row",
            Display::Flex => "d-flex",
            Display::InlineFlex => "d-inline-flex",
        }
    }
}

pub trait SetSpacing: ElementBuilder {
    /// Set the margin size
    ///
    /// A `size` of `None` will set the margin to `auto`
    fn margin(self, size: impl SignalOrValue<Item = Option<Size>>) -> Self {
        self.class(size.map(Option::<Size>::margin))
    }

    fn margin_on(
        self,
        margin: impl SignalOrValue<Item = (Option<Size>, Option<SideOrAxis>)>,
    ) -> Self {
        self.class(margin.map(|m| m.margin()))
    }

    /// Set the margin on `side`
    ///
    /// A `Size` of `None` will set the margin to `auto`
    fn margin_on_side(self, margin: impl SignalOrValue<Item = (Option<Size>, Side)>) -> Self {
        self.class(margin.map(|m| m.margin()))
    }

    /// Set the margin on `axis`
    ///
    /// A `Size` of `None` will set the margin to `auto`
    fn margin_on_axis(self, margin: impl SignalOrValue<Item = (Option<Size>, Axis)>) -> Self {
        self.class(margin.map(|m| m.margin()))
    }

    fn padding(self, size: impl SignalOrValue<Item = Size>) -> Self {
        self.class(size.map(Size::padding))
    }

    fn padding_on(self, padding: impl SignalOrValue<Item = (Size, SideOrAxis)>) -> Self {
        self.class(padding.map(|p| p.padding()))
    }

    fn padding_on_side(self, padding: impl SignalOrValue<Item = (Size, Side)>) -> Self {
        self.class(padding.map(|p| p.padding()))
    }

    fn padding_on_axis(self, padding: impl SignalOrValue<Item = (Size, Axis)>) -> Self {
        self.class(padding.map(|p| p.padding()))
    }
}

pub trait SetBorder: ElementBuilder {
    /// Use a border
    fn border(self, active: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(active.map(|active| active.then_some(css::BORDER)))
    }

    fn border_on(self, side: impl SignalOrValue<Item = Side>) -> Self {
        self.class(side.map(|side| side.border()))
    }

    fn border_colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(|colour| colour.border()))
    }

    fn border_width(self, size: impl SignalOrValue<Item = Size>) -> Self {
        self.class(size.map(|size| size.border()))
    }

    fn rounded_border(self, active: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(active.map(|active| active.then_some(css::ROUNDED)))
    }

    fn rounded_border_on(self, side: impl SignalOrValue<Item = Side>) -> Self {
        self.class(side.map(|side| side.rounded_border()))
    }

    fn rounded_border_of_size(self, size: impl SignalOrValue<Item = Size>) -> Self {
        self.class(size.map(Size::rounded_border))
    }

    fn rounded_pill_border(self, active: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(active.map(|active| active.then_some(css::ROUNDED_PILL)))
    }

    fn rounded_circular_border(self, active: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(active.map(|active| active.then_some(css::ROUNDED_CIRCLE)))
    }

    fn border_opacity(self, opacity: impl SignalOrValue<Item = Opacity>) -> Self {
        self.class(opacity.map(|opacity| opacity.border()))
    }

    fn shadow(self, shadow: impl SignalOrValue<Item = Shadow>) -> Self {
        self.class(shadow.map(|shadow| shadow.class()))
    }
}

pub trait SetOverflow: ElementBuilder {
    fn overflow(self, overflow: impl SignalOrValue<Item = Overflow>) -> Self {
        self.class(overflow.map(|overflow| overflow.class()))
    }
}

pub trait SetColour: ElementBuilder {
    fn background_colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(|colour| colour.background()))
    }

    fn text_colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(|colour| colour.text()))
    }

    fn link_colour(self, colour: impl SignalOrValue<Item = Colour>) -> Self {
        self.class(colour.map(|colour| colour.link()))
    }
}

pub trait SetDisplay: ElementBuilder {
    fn display(self, display: impl SignalOrValue<Item = Display>) -> Self {
        self.class(display.map(Display::class))
    }

    /// Add `d-flex` and `flex-column` classes
    fn flex_column(self) -> Self {
        self.flex(FlexDirection::Column)
    }

    /// Add `d-flex` and `flex-row` classes
    fn flex_row(self) -> Self {
        self.flex(FlexDirection::Row)
    }

    /// Add `display: flex` and `flex-direction: <direction>` classes
    fn flex(self, direction: impl SignalOrValue<Item = FlexDirection>) -> Self {
        self.class(css::D_FLEX)
            .class(direction.map(FlexDirection::class))
    }

    fn align_items(self, align: impl SignalOrValue<Item = Align>) -> Self {
        self.class(align.map(|align| align.align_items()))
    }
}

pub trait SetAlign: ElementBuilder {
    fn align_self(self, align: impl SignalOrValue<Item = Align>) -> Self {
        self.class(align.map(|align| align.align_self()))
    }
}

pub trait SetGap: ElementBuilder {
    fn gap(self, size: impl SignalOrValue<Item = Size>) -> Self {
        self.class(size.map(Size::gap))
    }
}

pub trait SetPosition: ElementBuilder {
    fn position(self, position: impl SignalOrValue<Item = Position>) -> Self {
        self.class(position.map(Position::class))
    }
}

pub trait Active: ElementBuilder {
    fn active(self, is_active: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(is_active.map(|flag| flag.then_some(css::ACTIVE)))
    }
}

pub trait Disabled: ElementBuilder {
    fn disabled(self, is_disabled: impl SignalOrValue<Item = bool>) -> Self {
        self.classes(is_disabled.map(|flag| flag.then_some(css::DISABLED)))
    }
}

impl<T: HtmlElement> SetSpacing for T {}
impl<T: HtmlElement> SetBorder for T {}
impl<T: ParentBuilder> SetOverflow for T {}
impl<T: HtmlElement> SetColour for T {}
impl<T: HtmlElement> SetAlign for T {}
impl<T: ParentBuilder> SetDisplay for T {}
impl<T: ParentBuilder> SetGap for T {}
impl<T: HtmlElement> SetPosition for T {}

impl Active for html::ABuilder {}
impl Active for html::ButtonBuilder {}

impl Disabled for html::ABuilder {}
