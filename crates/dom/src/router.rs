use silkenweb_reactive::signal::{ReadSignal, Signal};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::Url;

use crate::window;

/// A signal that will vary according to the current browser URL.
pub fn url() -> ReadSignal<Url> {
    URL.with(Signal::read)
}

/// Set the path portion of the URL.
///
/// The path is the part of the URL after the scheme, host and port. For
/// example, the path of <http://example.com/this/is/the/path> is "/this/is/the/path".
///
/// [`set_url_path`] will:
/// - Set the browser URL
/// - Push it onto the history stack so the forward and back buttons work
/// - Set the [`url()`] signal
pub fn set_url_path(path: impl 'static + AsRef<str>) {
    URL.with(move |url| {
        url.write().mutate(move |url| {
            url.set_pathname(path.as_ref());
            window()
                .history()
                .unwrap()
                .push_state_with_url(&JsValue::null(), "", Some(&url.href()))
                .unwrap();
        })
    })
}

fn new_url_signal() -> Signal<Url> {
    let window = window();
    let url = Url::new(
        &window
            .location()
            .href()
            .expect("Must be able to get window 'href'"),
    )
    .expect("URL must be valid");

    ON_POPSTATE
        .with(|on_popstate| window.set_onpopstate(Some(on_popstate.as_ref().unchecked_ref())));

    Signal::new(url)
}

thread_local! {
    static ON_POPSTATE: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_event: JsValue| {
            URL.with(|url| url.write().set(
                Url::new(&window().location().href().expect("HRef must exist")).expect("URL must be valid")
            ));
        }));
    static URL: Signal<Url> = new_url_signal();
}
