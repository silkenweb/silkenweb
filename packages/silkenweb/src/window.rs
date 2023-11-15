//! Window utilities.

use paste::paste;
use silkenweb_base::Window;
use wasm_bindgen::JsCast;

use crate::event::{bubbling_events, GlobalEventCallback};

/// Manage an event handler.
///
/// This will remove the event handler when dropped.
#[must_use = "`EventCallback` will be removed when it is dropped. Use the `perpetual` method to make it permanent."]
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
