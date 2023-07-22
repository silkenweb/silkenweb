mod app;
mod drive;
mod state;
#[cfg(test)]
mod test_utils;

#[cfg_browser(true)]
use app::hydrate_app;
#[cfg_browser(true)]
use wasm_bindgen::prelude::wasm_bindgen;

pub use app::app;
use drive::signal_drive_vector;
use silkenweb::cfg_browser;

#[cfg_browser(true)]
#[wasm_bindgen]
pub fn js_main() {
    hydrate_app()
}
