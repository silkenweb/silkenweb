use paste::paste;

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use wasm_bindgen::JsCast;

    pub struct EventCallback;

    impl EventCallback {
        pub fn new<Event: JsCast>(
            _name: &'static str,
            mut _f: impl FnMut(Event) + 'static,
        ) -> Self {
            Self
        }
    }

    impl Drop for EventCallback {
        fn drop(&mut self) {}
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use silkenweb_base::document;
    use wasm_bindgen::{intern, prelude::Closure, JsCast, JsValue};

    pub struct EventCallback {
        name: &'static str,
        callback: Closure<dyn FnMut(JsValue)>,
    }

    impl EventCallback {
        pub fn new<Event: JsCast>(name: &'static str, mut f: impl FnMut(Event) + 'static) -> Self {
            let name = intern(name);
            let callback = Closure::wrap(Box::new(move |js_ev: JsValue| {
                // I *think* we can assume event and event.current_target aren't null
                f(js_ev.unchecked_into());
            }) as Box<dyn FnMut(JsValue)>);

            document::add_event_listener_with_callback(name, callback.as_ref().unchecked_ref());

            Self { name, callback }
        }
    }

    impl Drop for EventCallback {
        fn drop(&mut self) {
            document::remove_event_listener_with_callback(
                self.name,
                self.callback.as_ref().as_ref().unchecked_ref(),
            );
        }
    }
}

pub use arch::EventCallback;

macro_rules! events{
    ($($name:ident: $typ:ty),* $(,)?) => { paste!{ $(
        pub fn [< on_ $name >] (f: impl FnMut($typ) + 'static) -> EventCallback {
            EventCallback::new(stringify!($name), f)
        }
    )*}}
}

events! {
    auxclick: web_sys::MouseEvent,
    click: web_sys::MouseEvent,
    compositionend: web_sys::CompositionEvent,
    compositionstart: web_sys::CompositionEvent,
    compositionupdate: web_sys::CompositionEvent,
    contextmenu: web_sys::MouseEvent,
    dblclick: web_sys::MouseEvent,
    focusin: web_sys::FocusEvent,
    focusout: web_sys::FocusEvent,
    fullscreenchange: web_sys::Event,
    fullscreenerror: web_sys::Event,
    keydown: web_sys::KeyboardEvent,
    keyup: web_sys::KeyboardEvent,
    mousedown: web_sys::MouseEvent,
    mouseenter: web_sys::MouseEvent,
    mouseleave: web_sys::MouseEvent,
    mousemove: web_sys::MouseEvent,
    mouseout: web_sys::MouseEvent,
    mouseover: web_sys::MouseEvent,
    mouseup: web_sys::MouseEvent,
    select: web_sys::Event,
    touchcancel: web_sys::TouchEvent,
    touchend: web_sys::TouchEvent,
    touchmove: web_sys::TouchEvent,
    touchstart: web_sys::TouchEvent,
    wheel: web_sys::WheelEvent
}
