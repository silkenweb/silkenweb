//! Document utilities.
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    pin::{pin, Pin},
    task,
    thread::LocalKey,
};

use futures::{future::RemoteHandle, Future};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{self, SignalVec, SignalVecExt},
};
use paste::paste;
use pin_project::pin_project;
use silkenweb_signals_ext::value::SignalOrValue;
use thiserror::Error;
use wasm_bindgen::JsCast;

use crate::{
    dom::{Dom, Dry, Wet},
    event::{bubbling_events, GlobalEventCallback},
    hydration::HydrationStats,
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
    type MountOutput;
    type MountInHeadOutput;

    /// Mount an element on the document.
    ///
    /// `id` is the id of the mount point element. The element will replace
    /// the mount point.
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> Self::MountOutput;

    /// Mount an element in the document `<head>`
    ///
    /// `id` is used by hydration, which will set a `data-silkenweb-head-id`
    /// attribute on each top level element in `head` so it can identify which
    /// elements to hydrate against. Mounting something with the same `id` twice
    /// will remove the first mounted `DocumentHead`.
    fn mount_in_head(
        id: &str,
        head: DocumentHead<Self>,
    ) -> Result<Self::MountInHeadOutput, HeadNotFound>;

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

#[pin_project]
pub struct MountHydro(#[pin] RemoteHandle<HydrationStats>);

impl Future for MountHydro {
    type Output = HydrationStats;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        self.project().0.poll(cx)
    }
}

#[pin_project]
pub struct MountHydroHead(#[pin] RemoteHandle<HydrationStats>);

impl Future for MountHydroHead {
    type Output = HydrationStats;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        self.project().0.poll(cx)
    }
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

type MountedChildVecMap<D> = RefCell<HashMap<String, ChildVecHandle<D, ParentShared>>>;

fn unmount_head<D: Dom>(
    mounted_in_head: &'static LocalKey<MountedChildVecMap<D>>,
) {
    for element in mounted_in_head.take().into_values() {
        element.clear();
    }
}

fn head_inner_html<D: Dom>(
    mounted_in_head: &'static LocalKey<MountedChildVecMap<D>>,
) -> String {
    let mut html = String::new();

    mounted_in_head.with(|mounted| {
        for elem in mounted.borrow().values() {
            html.push_str(&elem.inner_html());
        }
    });

    html
}

thread_local! {
    static WET_MOUNTED: RefCell<HashMap<String, GenericElement<Wet, Const>>> = RefCell::new(HashMap::new());
}
