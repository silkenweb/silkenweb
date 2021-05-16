use silkenweb::{html_element, Builder};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys as dom;

html_element!(
    ui5-calendar<dom::HtmlElement> {
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
    ui5-date<dom::HtmlElement> {
        attributes {
            value: String,
        }
    }
);

impl Ui5CalendarBuilder {
    pub fn selected_date(self, date: String) -> Self {
        Self {
            builder: self.builder.child(ui5_date().value(date).into_element()),
        }
    }
}

// TODO: Currently we need to implement StaticAttribute for these. Need a way to
// set any Display attr.
pub enum SelectionMode {
    Single,
    Range,
    Multiple,
}

pub enum PrimaryCalendarType {
    Gregorian,
    Buddhist,
    Islamic,
    Japanese,
    Persian,
}

pub struct SelectedDatesChange {
    event: dom::CustomEvent,
}

impl SelectedDatesChange {
    pub fn event(&self) -> &dom::CustomEvent {
        &self.event
    }

    pub fn selected_dates(&self) -> impl Iterator<Item = String> {
        self.event
            .detail()
            .unchecked_into::<Values>()
            .values()
            .into_vec()
            .into_iter()
            .map(|obj| obj.as_string().unwrap())
    }
}

impl From<dom::CustomEvent> for SelectedDatesChange {
    fn from(event: dom::CustomEvent) -> Self {
        Self { event }
    }
}

#[wasm_bindgen]
extern "C" {
    type Values;

    #[wasm_bindgen(structural, method, getter)]
    fn values(this: &Values) -> Box<[JsValue]>;
}
