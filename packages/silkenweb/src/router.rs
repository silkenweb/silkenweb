//! URL based routing.
//!
//! Get the URL path with [`url_path`], and set it with [`set_url_path`] or a
//! link to a fragment like `<a href="#anchor" ...>`.
//!
//! # Example
//!
//! ```no_run
//! # use html::{button, div, p, Div};
//! # use silkenweb::{prelude::*, router};
//! # let doc: Div =
//! div()
//!     .child(
//!         button()
//!             .on_click(|_, _| router::set_url_path("route_1"))
//!             .text("Go to route 1"),
//!     )
//!     .child(
//!         button()
//!             .on_click(|_, _| router::set_url_path("route_2"))
//!             .text("Go to route 2"),
//!     )
//!     .child(p().text(Sig(
//!         router::url_path().signal_ref(|url_path| format!("URL Path is: {url_path}")),
//!     )));
//! ```
use std::{collections::HashMap, fmt::Display};

use futures_signals::signal::{Mutable, ReadOnlyMutable};

use crate::{
    dom::Dom,
    elements::html::{a, A},
    prelude::ElementEvents,
};

/// Represent the path portion of a URL (including any query string)
#[derive(Clone, Eq, PartialEq)]
pub struct UrlPath {
    url: String,
    path_end: usize,
    query_end: usize,
}

impl UrlPath {
    /// Create a new `UrlPath`
    ///
    /// `path` should have any special characters percent escaped.
    /// Any leading `'/'`s are removed.
    pub fn new(path: &str) -> Self {
        let url = path.trim_start_matches('/').to_string();
        let query_end = url.find('#').unwrap_or(url.len());
        let path_end = url[..query_end].find('?').unwrap_or(query_end);

        Self {
            url,
            path_end,
            query_end,
        }
    }

    /// Get the path portion of the `UrlPath`
    ///
    /// ```
    /// # use silkenweb::router::UrlPath;
    /// assert_eq!(UrlPath::new("path?query_string").path(), "path");
    /// assert_eq!(UrlPath::new("?query_string").path(), "");
    /// assert_eq!(UrlPath::new("?").path(), "");
    /// assert_eq!(UrlPath::new("").path(), "");
    /// ```
    pub fn path(&self) -> &str {
        &self.url[..self.path_end]
    }

    /// Get the path components of the `UrlPath`
    ///
    /// ```
    /// # use silkenweb::router::UrlPath;
    /// let path = UrlPath::new("path1/path2/path3");
    /// let components: Vec<&str> = path.path_components().collect();
    /// assert_eq!(&components, &["path1", "path2", "path3"]);
    ///
    /// let path = UrlPath::new("");
    /// assert_eq!(path.path_components().next(), None);
    ///
    /// let path = UrlPath::new("path1//path2"); // Note the double `'/'`
    /// let components: Vec<&str> = path.path_components().collect();
    /// assert_eq!(&components, &["path1", "", "path2"]);
    pub fn path_components(&self) -> impl Iterator<Item = &str> {
        let path = self.path();
        let mut components = path.split('/');

        if path.is_empty() {
            components.next();
        }

        components
    }

    /// As [`UrlPath::path_components`] but collected into a `Vec`
    pub fn path_components_vec(&self) -> Vec<&str> {
        self.path_components().collect()
    }

    /// Get the query string portion of the `UrlPath`
    ///
    /// ```
    /// # use silkenweb::router::UrlPath;
    /// assert_eq!(
    ///     UrlPath::new("path?query_string").query_string(),
    ///     "query_string"
    /// );
    /// assert_eq!(UrlPath::new("?query_string").query_string(), "query_string");
    /// assert_eq!(UrlPath::new("?").query_string(), "");
    /// assert_eq!(UrlPath::new("").query_string(), "");
    /// assert_eq!(UrlPath::new("#hash").query_string(), "");
    /// assert_eq!(
    ///     UrlPath::new("?query_string#hash").query_string(),
    ///     "query_string"
    /// );
    /// ```
    pub fn query_string(&self) -> &str {
        self.range(self.path_end, self.query_end)
    }

    /// Split the query string into key/value pairs
    ///
    /// ```
    /// # use silkenweb::router::UrlPath;
    /// let path = UrlPath::new("path?x=1&y=2&flag");
    /// let kv_args: Vec<(&str, Option<&str>)> = path.query().collect();
    /// assert_eq!(
    ///     &kv_args,
    ///     &[("x", Some("1")), ("y", Some("2")), ("flag", None)]
    /// );
    /// ```
    pub fn query(&self) -> impl Iterator<Item = (&str, Option<&str>)> {
        self.query_string()
            .split('&')
            .map(|kv| kv.split_once('=').map_or((kv, None), |(k, v)| (k, Some(v))))
    }

    /// As [`UrlPath::query`] but collected into a `HashMap`
    pub fn query_map(&self) -> HashMap<&str, Option<&str>> {
        self.query().collect()
    }

    /// Get the query string portion of the `UrlPath`
    ///
    /// ```
    /// # use silkenweb::router::UrlPath;
    /// assert_eq!(UrlPath::new("path?query_string#hash").hash(), "hash");
    /// assert_eq!(UrlPath::new("#hash").hash(), "hash");
    /// assert_eq!(UrlPath::new("#").hash(), "");
    /// assert_eq!(UrlPath::new("").hash(), "");
    /// ```
    pub fn hash(&self) -> &str {
        self.range(self.query_end, self.url.len())
    }

    /// Get the whole path as a `&str`
    pub fn as_str(&self) -> &str {
        &self.url
    }

    fn range(&self, previous_end: usize, end: usize) -> &str {
        let start = previous_end + 1;

        if start > end {
            ""
        } else {
            &self.url[start..end]
        }
    }
}

impl Display for UrlPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> From<&'a str> for UrlPath {
    fn from(path: &'a str) -> Self {
        Self::new(path)
    }
}

impl From<String> for UrlPath {
    fn from(path: String) -> Self {
        Self::new(&path)
    }
}

/// The path portion of the URL.
///
/// The path will never start with a '/'.
pub fn url_path() -> ReadOnlyMutable<UrlPath> {
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
pub fn set_url_path(path: impl Into<UrlPath>) {
    arch::set_url_path(path)
}

/// Set up an HTML `<a>` element for routing.
///
/// Return an `<a>` element builder with the `href` attribute set to `path` and
/// an `on_click` handler. Modifier keys are correctly handled.
///
/// # Example
///
/// ```no_run
/// # use html::{a, A};
/// use silkenweb::{prelude::*, router::anchor};
/// let link: A = anchor("/my-path").text("click me");
/// ```
pub fn anchor<D: Dom>(path: impl Into<String>) -> A<D> {
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
/// # use html::{a, A};
/// # use silkenweb::{prelude::*, router::link_clicked};
/// let path = "/my_path";
/// let link: A = a().href(path).text("click me").on_click(link_clicked(path));
/// ```
pub fn link_clicked(
    path: impl Into<String>,
) -> impl FnMut(web_sys::MouseEvent, web_sys::HtmlAnchorElement) + 'static {
    let path = path.into();
    move |ev, _| {
        let modifier_key_pressed = ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key();

        if !modifier_key_pressed {
            ev.prevent_default();
            set_url_path(path.as_str());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use futures_signals::signal::Mutable;

    use super::{UrlPath, URL_PATH};

    pub fn new_url_path() -> Mutable<UrlPath> {
        Mutable::new(UrlPath::new(""))
    }

    pub fn set_url_path(path: impl Into<UrlPath>) {
        URL_PATH.with(move |url_path| url_path.set(path.into()));
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use futures_signals::signal::Mutable;
    use silkenweb_base::{document, window};
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::{UrlPath, URL_PATH};

    pub fn new_url_path() -> Mutable<UrlPath> {
        ON_POPSTATE
            .with(|on_popstate| window::set_onpopstate(Some(on_popstate.as_ref().unchecked_ref())));

        Mutable::new(local_pathname())
    }

    pub fn set_url_path(path: impl Into<UrlPath>) {
        let path = path.into();
        let mut url = BASE_URI.with(String::clone);
        url.push_str(path.as_str());

        URL_PATH.with(move |url_path| {
            window::history()
                .push_state_with_url(&JsValue::null(), "", Some(&url))
                .unwrap_throw();
            url_path.set(path);
        });
    }

    fn local_pathname() -> UrlPath {
        let url = window::location();

        BASE_URI.with(|base_uri| {
            url.href()
                .unwrap_throw()
                .strip_prefix(base_uri)
                .map_or_else(
                    || UrlPath::new(&url.pathname().unwrap_throw()),
                    UrlPath::new,
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
    static URL_PATH: Mutable<UrlPath> = arch::new_url_path();
}
