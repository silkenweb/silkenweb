//! URL based routing.
//!
//! Get the URL with [`url()`], and set it however you want to. For example:
//! - with an anchor element like `<a href="/some/link">Some link</a>`
//! - with [`set_url_path`].
//!
//! # Example
//!
//! ```no_run
//! # use silkenweb::{
//! #     elements::{
//! #         html::{button, div, p},
//! #         ElementEvents,
//! #     },
//! #     mount,
//! #     prelude::ParentBuilder,
//! #     router,
//! # };
//!
//! div()
//!     .child(
//!         button()
//!             .on_click(|_, _| router::set_url_path("/route_1"))
//!             .text("Go to route 1"),
//!     )
//!     .child(
//!         button()
//!             .on_click(|_, _| router::set_url_path("/route_2"))
//!             .text("Go to route 2"),
//!     )
//!     .child(p().text_signal(
//!         router::url_path().signal_ref(|url_path| format!("URL Path is: {}", url_path)),
//!     ));
//! ```
use futures_signals::signal::{Mutable, ReadOnlyMutable};

use crate::{
    elements::html::{a, ABuilder},
    prelude::ElementEvents,
};

/// The path portion of the URL.
///
/// The path will never start with a '/'.
pub fn url_path() -> ReadOnlyMutable<String> {
    URL_PATH.with(|url_path| url_path.read_only())
}

/// Set the path portion of the URL.
///
/// The path is the part of the URL after the scheme, host and port. For
/// example, the path of <http://example.com/this/is/the/path> is "/this/is/the/path".
///
/// [`set_url_path`] will:
/// - Set the browser URL
/// - Push it onto the history stack so the forward and back buttons work
/// - Set the [`url_path()`] signal
///
/// See [module-level documentation](self) for an example.
pub fn set_url_path(path: &str) {
    arch::set_url_path(path.trim_start_matches('/').to_string())
}

/// Set up an HTML `<a>` element for routing.
///
/// Return an `<a>` element builder with the `href` attribute set to `path` and
/// an `on_click` handler. Modifier keys are correctly handled.
///
/// # Example
///
/// ```no_run
/// # use silkenweb::{
/// #     elements::html::a, macros::ParentBuilder, prelude::ElementEvents, router::anchor,
/// # };
/// let link = anchor("/my-path").text("click me");
/// ```
pub fn anchor(path: impl Into<String>) -> ABuilder {
    let path = path.into();

    a().href(&path).on_click(link_clicked(path))
}

/// An `on_click` handler for routed `<a>` elements.
///
/// This will correctly deal with modifier keys. See also: [`anchor`].
///
/// # Example
///
/// ```no_run
/// # use silkenweb::{
/// #     elements::html::a, macros::ParentBuilder, prelude::ElementEvents, router::link_clicked,
/// # };
/// let path = "/my_path";
/// let link = a()
///     .href(path)
///     .text("click me")
///     .on_click(link_clicked(path));
/// ```
pub fn link_clicked(
    path: impl Into<String>,
) -> impl FnMut(web_sys::MouseEvent, web_sys::HtmlAnchorElement) + 'static {
    let path = path.into();
    move |ev, _| {
        let modifier_key_pressed = ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key();

        if !modifier_key_pressed {
            ev.prevent_default();
            set_url_path(&path);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use futures_signals::signal::Mutable;

    use super::URL_PATH;

    pub fn new_url_path() -> Mutable<String> {
        Mutable::new(String::new())
    }

    pub fn set_url_path(path: String) {
        URL_PATH.with(move |url_path| url_path.set(path));
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use futures_signals::signal::Mutable;
    use silkenweb_base::{document, window};
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::URL_PATH;

    pub fn new_url_path() -> Mutable<String> {
        ON_POPSTATE
            .with(|on_popstate| window::set_onpopstate(Some(on_popstate.as_ref().unchecked_ref())));

        Mutable::new(local_pathname())
    }

    pub fn set_url_path(path: String) {
        let mut url = BASE_URI.with(String::clone);
        url.push_str(&path);

        URL_PATH.with(move |url_path| {
            window::history()
                .push_state_with_url(&JsValue::null(), "", Some(&url))
                .unwrap_throw();
            url_path.set(path);
        });
    }

    fn local_pathname() -> String {
        let url = window::location();

        BASE_URI.with(|base_uri| {
            url.href()
                .unwrap_throw()
                .strip_prefix(base_uri)
                .map_or_else(
                    || {
                        url.pathname()
                            .unwrap_throw()
                            .trim_start_matches('/')
                            .to_string()
                    },
                    |url| url.trim_start_matches('/').to_string(),
                )
        })
    }

    thread_local! {
        static BASE_URI: String = {
            let mut base_uri = document::base_uri();

            if ! base_uri.ends_with('/') {
                base_uri.push('/');
            }

            base_uri
        };

        static ON_POPSTATE: Closure<dyn FnMut(JsValue)> =
            Closure::wrap(Box::new(move |_event: JsValue|
                URL_PATH.with(|url_path| url_path.set(local_pathname()))
            ));
    }
}

thread_local! {
    static URL_PATH: Mutable<String> = arch::new_url_path();
}
