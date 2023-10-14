use ssr_example_app::hydrate_app;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn js_main() {
    hydrate_app()
}
