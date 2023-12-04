//! Document utilities.
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    pin::Pin,
};

use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVec, SignalVecExt},
};
use paste::paste;
use silkenweb_signals_ext::value::SignalOrValue;
use thiserror::Error;
use wasm_bindgen::JsCast;

use crate::{
    dom::{Dom, Dry},
    event::{bubbling_events, GlobalEventCallback},
    node::{
        element::{
            child_vec::{ChildVecHandle, ParentShared},
            Const, GenericElement, ParentElement,
        },
        ChildNode, Node,
    },
};

mod dry;
mod hydro;
mod wet;

/// Manage an event handler.
///
/// This will remove the event handler when dropped.
#[must_use]
pub struct EventCallback(GlobalEventCallback<silkenweb_base::Document>);

impl EventCallback {
    fn new<Event: JsCast>(name: &'static str, f: impl FnMut(Event) + 'static) -> Self {
        Self(GlobalEventCallback::new(name, f))
    }

    /// Make this event permanent.
    pub fn perpetual(self) {
        self.0.perpetual()
    }
}

macro_rules! events{
    ($($name:ident: $typ:ty),* $(,)?) => { paste!{ $(
        #[doc = "Add a `" $name "` event handler at the document level." ]
        ///
        /// This only has an effect on WASM targets.
        pub fn [< on_ $name >] (f: impl FnMut($typ) + 'static) -> EventCallback {
            EventCallback::new(stringify!($name), f)
        }
    )*}}
}

/// Add a `DOMCContentLoaded` event handler at the document level." ]
///
/// This only has an effect on WASM targets.
pub fn on_dom_content_loaded(f: impl FnMut(web_sys::Event) + 'static) -> EventCallback {
    EventCallback::new("DOMContentLoaded", f)
}

events! {
    fullscreenchange: web_sys::Event,
    fullscreenerror: web_sys::Event,
    lostpointercapture: web_sys::PointerEvent,
    pointerlockchange: web_sys::Event,
    pointerlockerror: web_sys::Event,
    readystatechange: web_sys::Event,
    scroll: web_sys::Event,
    scrollend: web_sys::Event,
    selectionchange: web_sys::Event,
    visibilitychange: web_sys::Event,

    // These generate a `ClipboardEvent`, but that is currently unstable in `web_sys`.
    copy: web_sys::Event,
    cut: web_sys::Event,
    paste: web_sys::Event,
}

bubbling_events!();

pub trait Document: Dom + Sized {
    /// Mount an element on the document.
    ///
    /// `id` is the id of the mount point element. The element will replace
    /// the mount point.
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>);

    /// Mount an element in the document `<head>`
    ///
    /// `id` is used by hydration, which will set a `data-silkenweb-head-id`
    /// attribute on each top level element in `head` so it can identify which
    /// elements to hydrate against. Mounting something with the same `id` twice
    /// will remove the first mounted `DocumentHead`.
    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Result<(), HeadNotFound>;

    /// Remove all mounted elements.
    ///
    /// All elements mounted with `mount` or `mount_in_head` will be removed.
    /// Mount points will not be restored. This is useful to ensure a clean
    /// environment for testing.
    fn unmount_all();

    /// Get the inner HTML of `<head>`.
    ///
    /// This only includes elements added with `mount_in_head`. It's useful for
    /// server side rendering, where it can be used to add any stylesheets
    /// required for the HTML.
    fn head_inner_html() -> String;
}

#[derive(Error, Debug)]
pub struct HeadNotFound;

impl Display for HeadNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Head element not found")
    }
}

// TODO: More docs
/// The document's `<head>` element.
pub struct DocumentHead<D: Dom> {
    child_vec: Pin<Box<dyn SignalVec<Item = Node<D>>>>,
}

impl<D: Dom> Default for DocumentHead<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Dom> DocumentHead<D> {
    pub fn new() -> Self {
        let child_vec = Box::pin(signal_vec::always(Vec::new()));
        Self { child_vec }
    }
}

impl<D: Dom> ParentElement<D> for DocumentHead<D> {
    fn child(self, child: impl SignalOrValue<Item = impl ChildNode<D>>) -> Self {
        self.optional_child(child.map(Some))
    }

    fn optional_child(self, child: impl SignalOrValue<Item = Option<impl ChildNode<D>>>) -> Self {
        child.select(
            |parent, child| {
                if let Some(child) = child {
                    return parent.children_signal(signal_vec::always(vec![child]));
                }

                parent
            },
            |parent, child| {
                let child_vec = child
                    .map(|child| child.into_iter().collect::<Vec<_>>())
                    .to_signal_vec();
                parent.children_signal(child_vec)
            },
            self,
        )
    }

    fn children<N>(self, children: impl IntoIterator<Item = N>) -> Self
    where
        N: Into<Node<D>>,
    {
        self.children_signal(signal_vec::always(
            children.into_iter().map(|child| child.into()).collect(),
        ))
    }

    fn children_signal<N>(mut self, children: impl SignalVec<Item = N> + 'static) -> Self
    where
        N: Into<Node<D>>,
    {
        self.child_vec = self
            .child_vec
            .chain(children.map(|child| child.into()))
            .boxed_local();
        self
    }
}

#[derive(Default)]
pub(crate) struct TaskLocal {
    mounted_in_dry_head: RefCell<HashMap<String, ChildVecHandle<Dry, ParentShared>>>,
}
