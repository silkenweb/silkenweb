//! Document utilities.
use std::{
    cell::RefCell,
    collections::HashMap,
    pin::{pin, Pin},
    task,
};

use futures::{channel::oneshot, Future};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVec, SignalVecExt},
};
use paste::paste;
use pin_project::pin_project;
use silkenweb_base::document;
use silkenweb_signals_ext::value::SignalOrValue;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{
    dom::{self, Dom, Dry, Hydro, Wet},
    event::{bubbling_events, GlobalEventCallback},
    hydration::HydrationStats,
    node::{
        element::{
            child_vec::{ChildVecHandle, ParentShared},
            Const, Element, GenericElement,
        },
        Node,
    },
    HEAD_ID_ATTRIBUTE,
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
    type MountOutput;
    type MountInHeadOutput;

    /// Mount an element on the document.
    ///
    /// `id` is the id of the mount point element. The element will replace
    /// the mount point.
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> Self::MountOutput;

    /// Mount some children in the document `<head>`
    ///
    /// `id` is used by hydration, which will set a `data-silkenweb-head-id`
    /// attribute on each top level element in `head` so it can identify which
    /// elements to hydrate against.
    ///
    /// # Panics
    ///
    /// Mounting something with the same `id` will cause a panic.
    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Self::MountInHeadOutput;

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

/// The document's `<head>` element.
///
/// This allows you to create a set of (possibly reactive) children that can be
/// added to the head. See
/// [`hydrate_in_head`][`crate::hydration::hydrate_in_head`] for more details.
pub struct DocumentHead<D: Dom> {
    child_vec: Pin<Box<dyn SignalVec<Item = GenericElement<D>>>>,
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

impl<D: Dom> DocumentHead<D> {
    /// Add a child.
    ///
    /// This like [`ParentElement::child`][`crate::node::element::ParentElement::child`],
    /// but it only accepts element children, not text.
    pub fn child(
        self,
        child: impl SignalOrValue<Item = impl Into<GenericElement<D>> + 'static>,
    ) -> Self {
        self.optional_child(child.map(Some))
    }

    /// Add an optional child.
    ///
    /// This like [`ParentElement::optional_child`][`crate::node::element::ParentElement::optional_child`],
    /// but it only accepts element children, not text.
    pub fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Into<GenericElement<D>> + 'static>>,
    ) -> Self {
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

    /// Add some children
    ///
    /// This like [`ParentElement::children`][`crate::node::element::ParentElement::children`],
    /// but it only accepts element children, not text.
    pub fn children<N>(self, children: impl IntoIterator<Item = N>) -> Self
    where
        N: Into<GenericElement<D>>,
    {
        self.children_signal(signal_vec::always(
            children.into_iter().map(|child| child.into()).collect(),
        ))
    }

    /// Add some reactive children.
    ///
    /// This like [`ParentElement::children_signal`][`crate::node::element::ParentElement::children_signal`],
    /// but it only accepts element children, not text.
    pub fn children_signal<E>(mut self, children: impl SignalVec<Item = E> + 'static) -> Self
    where
        E: Into<GenericElement<D>>,
    {
        self.child_vec = self
            .child_vec
            .chain(children.map(|child| child.into()))
            .boxed_local();
        self
    }
}

// If we used `RemoteHandle`, we'd have to wait on the future for
// `HydrationStats` even if we want to discard it. Using `oneshot`, we can
// discard the whole future if we don't need `HydrationStats`.

/// The type of [`Hydro::MountOutput`][`crate::dom::Hydro::MountOutput`].
#[pin_project]
pub struct MountHydro(#[pin] oneshot::Receiver<HydrationStats>);

impl Future for MountHydro {
    type Output = HydrationStats;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        poll_receiver(self.project().0, cx)
    }
}

/// The type of
/// [`Hydro::MountInHeadOutput`][`crate::dom::Hydro::MountInHeadOutput`].
#[pin_project]
pub struct MountHydroHead(#[pin] oneshot::Receiver<HydrationStats>);

impl Future for MountHydroHead {
    type Output = HydrationStats;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        poll_receiver(self.project().0, cx)
    }
}

fn poll_receiver<T>(
    receiver: Pin<&mut oneshot::Receiver<T>>,
    cx: &mut task::Context<'_>,
) -> task::Poll<T> {
    receiver.poll(cx).map(|r| {
        // We send from a future that is run using `spawn_local`, so we can't cancel it.
        r.unwrap_throw()
    })
}

fn document_head() -> <Wet as dom::private::Dom>::Element {
    <Wet as dom::private::Dom>::Element::from_element(document::head().unwrap_throw().into())
}

fn children_with_id<D: Dom>(head: DocumentHead<D>, id: &str) -> impl SignalVec<Item = Node<D>> {
    head.child_vec.map({
        let id = id.to_string();
        move |child| child.attribute(HEAD_ID_ATTRIBUTE, id.clone()).into()
    })
}

#[derive(Default)]
pub(crate) struct TaskLocal {
    mounted_in_dry_head: RefCell<HashMap<String, ChildVecHandle<Dry, ParentShared>>>,
}

fn wet_insert_mounted(id: &str, element: GenericElement<Wet, Const>) {
    let existing = WET_MOUNTED.with(|mounted| mounted.borrow_mut().insert(id.to_string(), element));

    assert!(
        existing.is_none(),
        "Attempt to insert duplicate id ({id}) into document"
    );
}

fn wet_unmount() {
    for element in WET_MOUNTED.take().into_values() {
        element.dom_element().remove()
    }
}

struct MountedInHead<D: Dom>(RefCell<HashMap<String, ChildVecHandle<D, ParentShared>>>);

impl<D: Dom> MountedInHead<D> {
    fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    fn mount(&self, id: &str, child_vec: ChildVecHandle<D, ParentShared>) {
        WET_MOUNTED_IN_HEAD.with(|m| m.check_not_mounted(id, "wet"));
        HYDRO_MOUNTED_IN_HEAD.with(|m| m.check_not_mounted(id, "hydro"));
        let existing = self.0.borrow_mut().insert(id.to_string(), child_vec);

        assert!(
            existing.is_none(),
            "Attempt to insert duplicate id ({id}) into head"
        );
    }

    fn check_not_mounted(&self, id: &str, dom_type: &str) {
        assert!(
            !self.0.borrow().contains_key(id),
            "Id ({id}) is already mounted in {dom_type} head",
        );
    }

    fn unmount_all(&self) {
        for element in self.0.take().into_values() {
            element.clear();
        }
    }

    fn inner_html(&self) -> String {
        let mut html = String::new();

        for elem in self.0.borrow().values() {
            html.push_str(&elem.inner_html());
        }

        html
    }
}

thread_local! {
    static WET_MOUNTED: RefCell<HashMap<String, GenericElement<Wet, Const>>> = RefCell::new(HashMap::new());
    static WET_MOUNTED_IN_HEAD: MountedInHead<Wet> = MountedInHead::new();
    static HYDRO_MOUNTED_IN_HEAD: MountedInHead<Hydro> = MountedInHead::new();
}
