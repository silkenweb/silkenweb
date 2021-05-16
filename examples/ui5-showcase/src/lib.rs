#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::option_if_let_else
)]
use chrono::NaiveDate;
use silkenweb::mount;
use silkenweb_ui5::chrono::ui5_calendar;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    mount(
        "app",
        ui5_calendar()
            .format_pattern("yyyy-MM-dd")
            .selected_date(format!("{}", NaiveDate::from_ymd(2000, 1, 1))),
    );

    Ok(())
}
