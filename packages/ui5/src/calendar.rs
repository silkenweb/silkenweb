use std::borrow::Cow;

use chrono::{NaiveDate, NaiveDateTime};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::element::{GenericElement, ParentElement},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};

mod elements {
    use silkenweb::{custom_html_element, parent_element};

    use super::{CalendarType, SelectedDatesChange, SelectionMode};

    custom_html_element!(
        ui5_calendar = {
            dom_type: web_sys::HtmlElement;
            attributes {
                hide_week_numbers: bool,
                selection_mode: SelectionMode,
                format_pattern: String,
                max_date: String,
                min_date: String,
                primary_calendar_type: CalendarType,
                secondary_calendar_type: CalendarType,
            };

            custom_events {
                selected_dates_change: SelectedDatesChange,
            };
        }
    );

    parent_element!(ui5_calendar);

    custom_html_element!(
        ui5_date = {
            dom_type: web_sys::HtmlElement;
            attributes { value: String };
        }
    );
}

use self::elements::{ui5_calendar, ui5_date, Ui5Calendar};
use crate::macros::attributes0;

pub fn calendar() -> Calendar {
    Calendar(ui5_calendar())
}

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct Calendar(Ui5Calendar);

impl Calendar {
    attributes0! {
        hide_week_numbers: bool,
        selection_mode: SelectionMode,
        format_pattern: String,
        max_date: String,
        min_date: String,
        primary_calendar_type: CalendarType,
        secondary_calendar_type: CalendarType,
    }

    pub fn selected_dates(self, dates: impl IntoIterator<Item = String>) -> Self {
        Self(
            self.0
                .children(dates.into_iter().map(|date| ui5_date().value(date))),
        )
    }

    pub fn selected_dates_signal(self, dates: impl SignalVec<Item = String> + 'static) -> Self {
        Self(
            self.0
                .children_signal(dates.map(|date| ui5_date().value(date))),
        )
    }

    pub fn on_selected_dates_change(
        self,
        f: impl FnMut(SelectedDatesChange, web_sys::HtmlElement) + 'static,
    ) -> Self {
        Self(self.0.on_selected_dates_change(f))
    }
}

impl From<Calendar> for GenericElement {
    fn from(elem: Calendar) -> Self {
        elem.0.into()
    }
}

#[derive(Display, Copy, Clone, Value)]
pub enum SelectionMode {
    Single,
    Range,
    Multiple,
}

impl Attribute for SelectionMode {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.to_string()))
    }
}

impl AsAttribute<SelectionMode> for SelectionMode {}

#[derive(Display, Copy, Clone)]
pub enum CalendarType {
    Gregorian,
    Buddhist,
    Islamic,
    Japanese,
    Persian,
}

impl Attribute for CalendarType {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.to_string()))
    }
}

impl AsAttribute<CalendarType> for CalendarType {}

pub struct SelectedDatesChange {
    event: web_sys::CustomEvent,
}

impl SelectedDatesChange {
    pub fn event(&self) -> &web_sys::CustomEvent {
        &self.event
    }

    pub fn values(&self) -> impl Iterator<Item = String> {
        self.selected_dates()
            .values()
            .into_vec()
            .into_iter()
            .map(|obj| obj.as_string().unwrap_throw())
    }

    /// UTC dates
    pub fn dates(&self) -> impl Iterator<Item = NaiveDate> {
        self.selected_dates()
            .dates()
            .into_vec()
            .into_iter()
            .map(|obj| {
                let seconds = obj.as_f64().unwrap_throw() as i64;
                NaiveDateTime::from_timestamp(seconds, 0).date()
            })
    }

    fn selected_dates(&self) -> SelectedDates {
        self.event.detail().unchecked_into::<SelectedDates>()
    }
}

impl From<web_sys::CustomEvent> for SelectedDatesChange {
    fn from(event: web_sys::CustomEvent) -> Self {
        Self { event }
    }
}

#[wasm_bindgen]
extern "C" {
    type SelectedDates;

    #[wasm_bindgen(method, getter)]
    fn values(this: &SelectedDates) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter)]
    fn dates(this: &SelectedDates) -> Box<[JsValue]>;
}
