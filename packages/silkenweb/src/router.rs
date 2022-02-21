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
//! #     router::{self, Url},
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
//!         router::url().signal_ref(|url| format!("URL Path is: {}", url.pathname())),
//!     ));
//! ```
use futures_signals::signal::{Mutable, ReadOnlyMutable};

pub trait Url {
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    fn hash(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    fn pathname(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    fn host(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    fn hostname(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    fn href(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)
    fn origin(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    fn password(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    fn port(&self) -> String;
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    fn protocol(&self) -> String;
}

impl Url for web_sys::Url {
    fn hash(&self) -> String {
        self.hash()
    }

    fn pathname(&self) -> String {
        self.pathname()
    }

    fn host(&self) -> String {
        self.host()
    }

    fn hostname(&self) -> String {
        self.hostname()
    }

    fn href(&self) -> String {
        self.href()
    }

    fn origin(&self) -> String {
        self.origin()
    }

    fn password(&self) -> String {
        self.password()
    }

    fn port(&self) -> String {
        self.port()
    }

    fn protocol(&self) -> String {
        self.protocol()
    }
}

// TODO: Wrap `Url` in a struct and provide a server side implementation with a
// trait to define the interface

/// A signal that will vary according to the current browser URL.
///
/// See [module-level documentation](self) for an example.
pub fn url() -> ReadOnlyMutable<impl Url> {
    URL.with(|url| url.read_only())
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
///
/// See [module-level documentation](self) for an example.
pub fn set_url_path(path: &str) {
    arch::set_url_path(path)
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    // TODO: Make url dependency optional depending on arch
    use futures_signals::signal::Mutable;

    use super::{Url, URL};

    pub type UrlType = url::Url;

    impl Url for UrlType {
        fn hash(&self) -> String {
            self.fragment()
                .map_or_else(String::new, |frag| format!("#{}", frag))
        }

        fn pathname(&self) -> String {
            self.path().to_string()
        }

        fn host(&self) -> String {
            self.host()
                .map_or_else(String::new, |host| host.to_string())
        }

        fn hostname(&self) -> String {
            self.domain().map_or_else(String::new, str::to_owned)
        }

        fn href(&self) -> String {
            self.to_string()
        }

        fn origin(&self) -> String {
            self.origin().unicode_serialization()
        }

        fn password(&self) -> String {
            self.password().map_or_else(String::new, str::to_string)
        }

        fn port(&self) -> String {
            self.port()
                .map_or_else(String::new, |port| format!("{}", port))
        }

        fn protocol(&self) -> String {
            format!("{}:", self.scheme())
        }
    }

    pub fn new_url_signal() -> Mutable<UrlType> {
        Mutable::new(UrlType::parse("http://127.0.0.1/").unwrap())
    }

    pub fn set_url_path(path: &str) {
        URL.with(move |url| url.lock_mut().set_path(path));
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use std::ops::DerefMut;

    use futures_signals::signal::Mutable;
    use silkenweb_base::window;
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::URL;

    pub type UrlType = web_sys::Url;

    pub fn new_url_signal() -> Mutable<UrlType> {
        let url = UrlType::new(
            &window::location()
                .href()
                .expect_throw("Must be able to get window 'href'"),
        )
        .expect_throw("URL must be valid");

        ON_POPSTATE
            .with(|on_popstate| window::set_onpopstate(Some(on_popstate.as_ref().unchecked_ref())));

        Mutable::new(url)
    }

    pub fn set_url_path(path: &str) {
        URL.with(move |url| {
            let mut url = url.lock_mut();
            // Force a `deref_mut` to make sure `url` is marked as modified. `set_pathname`
            // uses interior mutability, so we'll only `deref`.
            url.deref_mut().set_pathname(path);
            window::history()
                .push_state_with_url(&JsValue::null(), "", Some(&url.href()))
                .unwrap_throw();
        });
    }

    thread_local! {
        static ON_POPSTATE: Closure<dyn FnMut(JsValue)> =
            Closure::wrap(Box::new(move |_event: JsValue| {
                URL.with(|url| url.set(
                    UrlType::new(&window::location().href().expect_throw("HRef must exist")).expect_throw("URL must be valid")
                ));
            }));
    }
}

thread_local! {
    static URL: Mutable<arch::UrlType> = arch::new_url_signal();
}
