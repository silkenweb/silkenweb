use std::borrow::Cow;

use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    html_element,
    node::element::{ElementBuilder, ParentBuilder},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};

html_element!(
    ui5-calendar<web_sys::HtmlElement> {
        attributes {
            hide-week-numbers: bool,
            selection-mode: SelectionMode,
            format-pattern: String,
            max-date: String,
            min-date: String,
            primary-calendar-type: PrimaryCalendarType,
        }

        custom_events {
            selected-dates-change: SelectedDatesChange
        }
    }
);

html_element!(
    ui5-date<web_sys::HtmlElement> {
        attributes {
            value: String,
        }
    }
);

impl Ui5CalendarBuilder {
    pub fn selected_date(self, date: String) -> Self {
        Self {
            builder: self.builder.child(ui5_date().value(&date).build()),
        }
    }
}

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
pub enum PrimaryCalendarType {
    Gregorian,
    Buddhist,
    Islamic,
    Japanese,
    Persian,
}

impl Attribute for PrimaryCalendarType {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.to_string()))
    }
}

pub struct SelectedDatesChange {
    event: web_sys::CustomEvent,
}

impl SelectedDatesChange {
    pub fn event(&self) -> &web_sys::CustomEvent {
        &self.event
    }

    pub fn selected_dates(&self) -> impl Iterator<Item = String> {
        self.event
            .detail()
            .unchecked_into::<Values>()
            .values()
            .into_vec()
            .into_iter()
            .map(|obj| obj.as_string().unwrap_throw())
    }
}

impl From<web_sys::CustomEvent> for SelectedDatesChange {
    fn from(event: web_sys::CustomEvent) -> Self {
        Self { event }
    }
}

#[wasm_bindgen]
extern "C" {
    type Values;

    #[wasm_bindgen(structural, method, getter)]
    fn values(this: &Values) -> Box<[JsValue]>;
}
