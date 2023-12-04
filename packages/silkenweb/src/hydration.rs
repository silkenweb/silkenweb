//! Hydrate the document with event handlers.
//!
//! "Hydrating" an element will attach any event handlers to the existing
//! document HTML. This allows you render HTML on the server and produce an
//! initial page that non-wasm clients can view, whilst wasm-enabled clients
//! still have a fully interactive app. See [`hydrate`] for more details on how
//! this is done.
use std::fmt;

use wasm_bindgen::JsCast;

use crate::{
    document::Document,
    dom::Hydro,
    node::element::{Const, GenericElement},
};

/// Statistics about the hydration process.
#[derive(Default)]
pub struct HydrationStats {
    nodes_added: u64,
    nodes_removed: u64,
    empty_text_removed: u64,
    attributes_set: u64,
    attributes_removed: u64,
}

impl HydrationStats {
    /// `true` if the only diffs between the existing HTML and the element were
    /// whitespace only text nodes.
    pub fn only_whitespace_diffs(&self) -> bool {
        self.nodes_added == 0
            && self.nodes_removed == 0
            && self.attributes_set == 0
            && self.attributes_removed == 0
    }

    /// `true` if there were no diffs between the existing HTML and the element.
    pub fn exact_match(&self) -> bool {
        self.empty_text_removed == 0 && self.only_whitespace_diffs()
    }

    /// The number of new nodes that were added during hydration.
    pub fn nodes_added(&self) -> u64 {
        self.nodes_added
    }

    /// The number of existing (non empty text) nodes that were removed during
    /// hydration.
    pub fn nodes_removed(&self) -> u64 {
        self.nodes_removed
    }

    /// The number of existing empty text nodes that were removed during
    /// hydration.
    pub fn empty_text_removed(&self) -> u64 {
        self.empty_text_removed
    }

    /// The number of new attributes that needed to be set during hydration.
    pub fn attributes_set(&self) -> u64 {
        self.attributes_set
    }

    /// The number of existing attributes that were removed during hydration.
    pub fn attributes_removed(&self) -> u64 {
        self.attributes_removed
    }

    pub(super) fn node_added(&mut self, _elem: &web_sys::Node) {
        self.nodes_added += 1;
    }

    pub(super) fn node_removed(&mut self, node: &web_sys::Node) {
        match node
            .dyn_ref::<web_sys::Text>()
            .and_then(|t| t.text_content())
        {
            Some(text) if text.trim().is_empty() => self.empty_text_removed += 1,
            _ => self.nodes_removed += 1,
        }
    }

    pub(super) fn attribute_set(&mut self, _elem: &web_sys::Element, _name: &str, _value: &str) {
        self.attributes_set += 1;
    }

    pub(super) fn attribute_removed(&mut self, _elem: &web_sys::Element, _name: &str) {
        self.attributes_removed += 1;
    }
}

impl fmt::Display for HydrationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Hydration stats:")?;
        writeln!(f, "    nodes added = {}", self.nodes_added)?;
        writeln!(f, "    nodes removed = {}", self.nodes_removed)?;
        writeln!(f, "    empty text removed = {}", self.empty_text_removed)?;
        writeln!(f, "    attributes set = {}", self.attributes_set)?;
        writeln!(f, "    attributes removed = {}", self.attributes_removed)
    }
}

/// Hydrate an element.
///
/// `id` is the id of the element in the
/// existing HTML.
///
/// [`hydrate`] will recursively attach the event handlers from `elem` to the
/// element with id=`id`. If any node in the existing HTML doesn't match the
/// corresponding node within `elem`, the existing node will be deleted. If a
/// matching node isn't found, a new one will be created. This way, hydration
/// never fails, but in the worst case will discard the original HTML. You can
/// track how well the existing HTML matched `elem` with the returned
/// [`HydrationStats`]. Generally speaking, extra nodes in the existing document
/// HTML will be removed, and hydration will continue. Extra nodes within `elem`
/// will cause the existing HTML to be replaced. This allows extra whitespace
/// nodes to be introduced to prettify the server HTML, without impacting the
/// hydration process.
///
/// Attributes will be added or removed when necessary to make sure the exisitng
/// HTML matches `elem`. Attributes beginning with `data-silkenweb` will be left
/// as they are in the existing HTML.
///
/// Effect handlers registered with [`effect`] will be called once an element is
/// hydrated.
///
/// # Warning
///
/// It's a good idea to create your app outsize the `async` block that calls
/// [`hydrate`], otherwise your signals won't be initialized before hydration.
///
/// ## Good
///
/// Creating the app outside the async block that calls [`hydrate`].
///
/// ```no_run
/// # use futures_signals::signal::always;
/// # use html::p;
/// # use silkenweb::{hydration::hydrate, prelude::*, task::spawn_local};
/// let app = p().text(Sig(always("Hello, world!")));
///
/// spawn_local(async {
///     hydrate("app", app).await;
/// });
/// ```
///
/// This will hydrate to `<p>Hello, world!</p>` correctly.
///
/// ## Bad
///
/// Creating the app inside the async block that calls [`hydrate`].
///
/// ```no_run
/// # use futures_signals::signal::always;
/// # use html::p;
/// # use silkenweb::{hydration::hydrate, prelude::*, task::spawn_local};
/// spawn_local(async {
///     let app = p().text(Sig(always("Hello, world!")));
///     hydrate("app", app).await;
/// });
/// ```
///
/// This will hydrate to `<p></p>`, removing any existing text. Then the signal
/// will initialize and set the text to "Hello, world!".
///
/// # Example
///
/// See [examples/hydration](http://github.com/silkenweb/silkenweb/tree/main/examples/hydration)
/// for an example.
///
/// [`effect`]: crate::node::element::Element::effect
/// [`eval_dom_node`]: crate::node::Node::eval_dom_node
// TODO: Remove this in favour of `Hydro::mount`, but put these docs on `mount`
// hydro instance.
pub async fn hydrate(id: &str, element: impl Into<GenericElement<Hydro, Const>>) -> HydrationStats {
    Hydro::mount(id, element)
}
