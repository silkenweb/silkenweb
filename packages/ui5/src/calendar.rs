use std::borrow::Cow;

use chrono::{NaiveDate, NaiveDateTime};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::element::ParentBuilder,
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    ElementBuilder,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};

mod elements {
    use silkenweb::{html_element, parent_element};

    use super::{CalendarType, SelectedDatesChange, SelectionMode};

    html_element!(
        ui5-calendar<web_sys::HtmlElement> {
            attributes {
                hide-week-numbers: bool,
                selection-mode: SelectionMode,
                format-pattern: String,
                max-date: String,
                min-date: String,
                primary-calendar-type: CalendarType,
                secondary-calendar-type: CalendarType,
            }

            custom_events {
                selected-dates-change: SelectedDatesChange
            }
        }
    );

    parent_element!(ui5 - calendar);

    html_element!(
        ui5-date<web_sys::HtmlElement> {
            attributes {
                value: String,
            }
        }
    );
}

pub use elements::Ui5Calendar as Calendar;

use self::elements::{ui5_calendar, ui5_date, Ui5CalendarBuilder};
use crate::macros::attributes0;

pub fn calendar() -> CalendarBuilder {
    CalendarBuilder(ui5_calendar())
}

#[derive(ElementBuilder)]
pub struct CalendarBuilder(Ui5CalendarBuilder);

impl CalendarBuilder {
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

    pub fn selected_dates_signal(self, dates: impl SignalVec<Item = String> + 'static) -> Calendar {
        self.0
            .children_signal(dates.map(|date| ui5_date().value(date)))
    }

    pub fn on_selected_dates_change(
        self,
        f: impl FnMut(SelectedDatesChange, web_sys::HtmlElement) + 'static,
    ) -> Self {
        Self(self.0.on_selected_dates_change(f))
    }
}

impl HtmlElement for CalendarBuilder {}

impl HtmlElementEvents for CalendarBuilder {}

impl ElementEvents for CalendarBuilder {}

#[derive(Display, Copy, Clone)]
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
