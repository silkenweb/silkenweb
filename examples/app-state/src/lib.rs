mod app;
mod state;

#[cfg_browser(true)]
use app::hydrate_app;
use silkenweb::cfg_browser;

#[cfg_browser(true)]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_browser(true)]
#[wasm_bindgen]
pub fn js_main() {
    hydrate_app()
}
