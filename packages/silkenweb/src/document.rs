//! Document utilities.

use paste::paste;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::DOCUMENT;
use crate::{
    event::{bubbling_events, GlobalEventCallback},
    GlobalEventTarget,
};

/// Manage an event handler.
///
/// This will remove the event handler when dropped.
#[must_use]
pub struct EventCallback(GlobalEventCallback<Document>);

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

pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    DOCUMENT.with(|doc| doc.get_element_by_id(id))
}

pub fn create_element(tag: &str) -> web_sys::Element {
    DOCUMENT.with(|doc| doc.create_element(tag).unwrap_throw())
}

pub fn create_element_ns(namespace: &str, tag: &str) -> web_sys::Element {
    DOCUMENT.with(|doc| doc.create_element_ns(Some(namespace), tag).unwrap_throw())
}

pub fn create_text_node(text: &str) -> web_sys::Text {
    DOCUMENT.with(|doc| doc.create_text_node(text))
}

pub fn base_uri() -> String {
    DOCUMENT
        .with(|doc| doc.base_uri())
        .unwrap_throw()
        .unwrap_throw()
}

pub fn query_selector(selectors: &str) -> Result<Option<web_sys::Element>, JsValue> {
    DOCUMENT.with(|doc| doc.query_selector(selectors))
}

pub fn head() -> Option<web_sys::HtmlHeadElement> {
    DOCUMENT.with(|doc| doc.head())
}

pub fn body() -> Option<web_sys::HtmlElement> {
    DOCUMENT.with(|doc| doc.body())
}

pub struct Document;

impl GlobalEventTarget for Document {
    fn add_event_listener_with_callback(name: &'static str, listener: &::js_sys::Function) {
        DOCUMENT.with(|doc| {
            doc.add_event_listener_with_callback(name, listener)
                .unwrap_throw()
        })
    }

    fn remove_event_listener_with_callback(name: &'static str, listener: &::js_sys::Function) {
        DOCUMENT.with(|doc| {
            doc.remove_event_listener_with_callback(name, listener)
                .unwrap_throw()
        })
    }
}
