//! URL based routing.
//!
//! Get the URL path with [`url_path`], and set it with [`set_url_path`] or a
//! link to a fragment like `<a href="#anchor" ...>`.
//!
//! # Example
//!
//! ```no_run
#![doc = function_body!("tests/doc/router.rs", module_example, [])]
//! ```
use std::{collections::HashMap, fmt::Display};

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use include_doc::function_body;
use silkenweb_macros::cfg_browser;

use crate::{
    dom::Dom,
    elements::{
        html::{a, A},
        ElementEvents,
    },
    task,
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
    #[doc = function_body!("tests/doc/router.rs", url_path, [])]
    /// ```
    pub fn path(&self) -> &str {
        &self.url[..self.path_end]
    }

    /// Get the path components of the `UrlPath`
    ///
    /// ```
    #[doc = function_body!("tests/doc/router.rs", url_path_components, [])]
    /// ```
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
    #[doc = function_body!("tests/doc/router.rs", url_query_string, [])]
    /// ```
    pub fn query_string(&self) -> &str {
        self.range(self.path_end, self.query_end)
    }

    /// Split the query string into key/value pairs
    ///
    /// ```
    #[doc = function_body!("tests/doc/router.rs", url_query, [])]
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
    #[doc = function_body!("tests/doc/router.rs", url_hash, [])]
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
    task::local::with(|local| local.router.0.read_only())
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
#[doc = function_body!("tests/doc/router.rs", anchor_example, [])]
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
#[doc = function_body!("tests/doc/router.rs", link_clicked_example, [])]
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

pub(crate) struct TaskLocal(Mutable<UrlPath>);

impl Default for TaskLocal {
    fn default() -> Self {
        Self(Mutable::new(arch::new_url_path()))
    }
}

#[cfg_browser(false)]
mod arch {
    use super::UrlPath;
    use crate::task;

    pub fn new_url_path() -> UrlPath {
        UrlPath::new("")
    }

    pub fn set_url_path(path: impl Into<UrlPath>) {
        task::local::with(move |local| local.router.0.set(path.into()));
    }
}

#[cfg_browser(true)]
mod arch {
    use silkenweb_base::{document, window};
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::UrlPath;
    use crate::task;

    pub fn new_url_path() -> UrlPath {
        ON_POPSTATE
            .with(|on_popstate| window::set_onpopstate(Some(on_popstate.as_ref().unchecked_ref())));

        local_pathname()
    }

    pub fn set_url_path(path: impl Into<UrlPath>) {
        let path = path.into();
        let mut url = BASE_URI.with(String::clone);
        url.push_str(path.as_str());

        task::local::with(move |local| {
            window::history()
                .push_state_with_url(&JsValue::null(), "", Some(&url))
                .unwrap_throw();
            local.router.0.set(path);
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
            let mut base_uri = document::base_uri().unwrap_or_else(
                || window::location().origin().unwrap_throw()
            );

            if ! base_uri.ends_with('/') {
                base_uri.push('/');
            }

            base_uri
        };

        static ON_POPSTATE: Closure<dyn FnMut(JsValue)> =
            Closure::wrap(Box::new(move |_event: JsValue|
                task::local::with(|local| local.router.0.set(local_pathname()))
            ));
    }
}
