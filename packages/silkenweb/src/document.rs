//! Document utilities.

use paste::paste;
use wasm_bindgen::JsCast;

use crate::{
    dom::Dom,
    event::{bubbling_events, GlobalEventCallback},
};

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

#[deprecated(note = "Use render::Document instead")]
pub trait Document: Dom + Sized {}
