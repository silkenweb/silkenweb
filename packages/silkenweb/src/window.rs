//! Window utilities.

use js_sys::Function;
use paste::paste;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::{
    event::{bubbling_events, GlobalEventCallback},
    GlobalEventTarget,
};

/// Manage an event handler.
///
/// This will remove the event handler when dropped.
#[must_use]
pub struct EventCallback(GlobalEventCallback<Window>);

impl EventCallback {
    fn new<Event: JsCast>(name: &'static str, f: impl FnMut(Event) + 'static) -> Self {
        Self(GlobalEventCallback::new(name, f))
    }

    /// Make this event permanent.
    pub fn perpetual(self) {
        self.0.perpetual()
    }
}

/// Add a `DOMCContentLoaded` event handler at the window level." ]
///
/// This only has an effect on WASM targets.
pub fn on_dom_content_loaded(f: impl FnMut(web_sys::Event) + 'static) -> EventCallback {
    EventCallback::new("DOMContentLoaded", f)
}

pub fn request_animation_frame(callback: &::js_sys::Function) {
    WINDOW.with(|win| win.request_animation_frame(callback).unwrap_throw());
}

pub fn history() -> web_sys::History {
    WINDOW.with(|win| win.history().unwrap_throw())
}

pub fn location() -> web_sys::Location {
    WINDOW.with(|win| win.location())
}

pub fn set_onpopstate(value: Option<&Function>) {
    WINDOW.with(|win| win.set_onpopstate(value));
}

pub fn local_storage() -> Result<web_sys::Storage, JsValue> {
    WINDOW.with(|w| w.local_storage().map(|w| w.unwrap_throw()))
}

pub fn session_storage() -> Result<web_sys::Storage, JsValue> {
    WINDOW.with(|w| w.session_storage().map(|w| w.unwrap_throw()))
}

pub fn performance() -> Option<web_sys::Performance> {
    WINDOW.with(|w| w.performance())
}
macro_rules! events{
    ($($name:ident: $typ:ty),* $(,)?) => { paste!{ $(
        #[doc = "Add a `" $name "` event handler at the window level." ]
        ///
        /// This only has an effect on WASM targets.
        pub fn [< on_ $name >] (f: impl FnMut($typ) + 'static) -> EventCallback {
            EventCallback::new(stringify!($name), f)
        }
    )*}}
}

events! {
    afterprint: web_sys::Event,
    appinstalled: web_sys::Event,
    beforeinstallprompt: web_sys::Event,
    beforeprint: web_sys::Event,
    beforeunload: web_sys::BeforeUnloadEvent,
    blur: web_sys::FocusEvent,
    devicemotion: web_sys::DeviceMotionEvent,
    deviceorientation: web_sys::DeviceOrientationEvent,
    deviceorientationabsolute: web_sys::DeviceOrientationEvent,
    error: web_sys::Event,
    focus: web_sys::FocusEvent,
    gamepadconnected: web_sys::Event,
    gamepaddisconnected: web_sys::Event,
    hashchange: web_sys::HashChangeEvent,
    languagechange: web_sys::Event,
    load: web_sys::Event,
    message: web_sys::MessageEvent,
    messageerror: web_sys::MessageEvent,
    offline: web_sys::Event,
    online: web_sys::Event,
    pagehide: web_sys::PageTransitionEvent,
    pageshow: web_sys::PageTransitionEvent,
    popstate: web_sys::PopStateEvent,
    rejectionhandled: web_sys::PromiseRejectionEvent,
    resize: web_sys::UiEvent,
    storage: web_sys::StorageEvent,
    unhandledrejection: web_sys::PromiseRejectionEvent,
    unload: web_sys::Event,

    // These generate a `ClipboardEvent`, but that is currently unstable in `web_sys`.
    copy: web_sys::Event,
    cut: web_sys::Event,
    paste: web_sys::Event,
}

bubbling_events!();

pub struct Window;

impl GlobalEventTarget for Window {
    fn add_event_listener_with_callback(name: &'static str, listener: &::js_sys::Function) {
        WINDOW.with(|win| {
            win.add_event_listener_with_callback(name, listener)
                .unwrap_throw()
        })
    }

    fn remove_event_listener_with_callback(name: &'static str, listener: &::js_sys::Function) {
        WINDOW.with(|win| {
            win.remove_event_listener_with_callback(name, listener)
                .unwrap_throw()
        })
    }
}

thread_local!(
    static WINDOW: web_sys::Window = web_sys::window().expect_throw("Window must be available");
    static DOCUMENT: web_sys::Document = WINDOW.with(|win| {
        win.document()
            .expect_throw("Window must contain a document")
    });
);
