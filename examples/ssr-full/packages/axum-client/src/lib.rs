use ssr_full_app::hydrate_app;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn js_main() {
    hydrate_app()
}
