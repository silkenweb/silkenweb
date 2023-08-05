mod app;
mod drive;
mod state;
#[cfg(test)]
mod test_utils;

pub use app::app;
#[cfg_browser(true)]
use app::hydrate_app;
use futures_util::Future;
use silkenweb::cfg_browser;
#[cfg_browser(true)]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_browser(true)]
#[wasm_bindgen]
pub fn js_main() {
    hydrate_app()
}

#[cfg(not(test))]
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    silkenweb::task::spawn_local(future)
}

#[cfg(test)]
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    tokio::task::spawn_local(future);
}
