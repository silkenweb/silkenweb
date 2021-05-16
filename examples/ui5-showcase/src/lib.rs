#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::option_if_let_else
)]
use silkenweb::mount;
use silkenweb_ui5::chrono::{ui5_calendar, SelectionMode};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    mount(
        "app",
        ui5_calendar()
            .format_pattern("yyyy-MM-dd")
            .selected_date("2000-01-01".to_string())
            .selection_mode(SelectionMode::Multiple)
            .on_selected_dates_change(|event, _target| {
                for d in event.selected_dates() {
                    web_log::println!("{}", d);
                }
            }),
    );

    Ok(())
}
