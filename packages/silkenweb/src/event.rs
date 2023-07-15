use silkenweb_macros::cfg_browser;

// Events that might bubble.
//
// The best sources I could find are:
// - <https://w3c.github.io/uievents/>
// - <https://en.wikipedia.org/wiki/DOM_event>
macro_rules! bubbling_events {
    () => {
        events! {
            auxclick: web_sys::MouseEvent,
            beforeinput: web_sys::InputEvent,
            change: web_sys::Event,
            click: web_sys::MouseEvent,
            compositionend: web_sys::CompositionEvent,
            compositionstart: web_sys::CompositionEvent,
            compositionupdate: web_sys::CompositionEvent,
            contextmenu: web_sys::MouseEvent,
            dblclick: web_sys::MouseEvent,
            drag: web_sys::DragEvent,
            dragend: web_sys::DragEvent,
            dragenter: web_sys::DragEvent,
            dragleave: web_sys::DragEvent,
            dragover: web_sys::DragEvent,
            dragstart: web_sys::DragEvent,
            drop: web_sys::DragEvent,
            focusin: web_sys::FocusEvent,
            focusout: web_sys::FocusEvent,
            input: web_sys::InputEvent,
            keydown: web_sys::KeyboardEvent,
            keyup: web_sys::KeyboardEvent,
            mousedown: web_sys::MouseEvent,
            mousemove: web_sys::MouseEvent,
            mouseout: web_sys::MouseEvent,
            mouseover: web_sys::MouseEvent,
            mouseup: web_sys::MouseEvent,
            reset: web_sys::Event,
            select: web_sys::Event,
            submit: web_sys::Event,
            touchcancel: web_sys::TouchEvent,
            touchend: web_sys::TouchEvent,
            touchmove: web_sys::TouchEvent,
            touchstart: web_sys::TouchEvent,
            wheel: web_sys::WheelEvent,
        }
    };
}

pub(crate) use bubbling_events;

#[cfg_browser(false)]
mod arch {
    use std::marker::PhantomData;

    use silkenweb_base::GlobalEventTarget;
    use wasm_bindgen::JsCast;

    pub struct GlobalEventCallback<T: GlobalEventTarget>(PhantomData<T>);

    impl<T: GlobalEventTarget> GlobalEventCallback<T> {
        pub fn new<Event: JsCast>(
            _name: &'static str,
            mut _f: impl FnMut(Event) + 'static,
        ) -> Self {
            Self(PhantomData)
        }

        pub fn perpetual(self) {}
    }

    impl<T: GlobalEventTarget> Drop for GlobalEventCallback<T> {
        fn drop(&mut self) {}
    }
}

#[cfg_browser(true)]
mod arch {
    use std::{cell::RefCell, marker::PhantomData};

    use silkenweb_base::GlobalEventTarget;
    use wasm_bindgen::{intern, prelude::Closure, JsCast, JsValue};

    pub struct GlobalEventCallback<T: GlobalEventTarget> {
        name: &'static str,
        callback: Option<Closure<dyn FnMut(JsValue)>>,
        phantom: PhantomData<T>,
    }

    impl<T: GlobalEventTarget> GlobalEventCallback<T> {
        pub fn new<Event: JsCast>(name: &'static str, mut f: impl FnMut(Event) + 'static) -> Self {
            let name = intern(name);
            let callback = Closure::wrap(Box::new(move |js_ev: JsValue| {
                // I *think* we can assume event and event.current_target aren't null
                f(js_ev.unchecked_into());
            }) as Box<dyn FnMut(JsValue)>);

            T::add_event_listener_with_callback(name, callback.as_ref().unchecked_ref());

            Self {
                name,
                callback: Some(callback),
                phantom: PhantomData,
            }
        }

        pub fn perpetual(mut self) {
            EVENTS.with(|events| {
                if let Some(callback) = self.callback.take() {
                    events.borrow_mut().push(callback)
                }
            })
        }
    }

    impl<T: GlobalEventTarget> Drop for GlobalEventCallback<T> {
        fn drop(&mut self) {
            if let Some(callback) = &self.callback {
                T::remove_event_listener_with_callback(
                    self.name,
                    callback.as_ref().as_ref().unchecked_ref(),
                );
            }
        }
    }

    thread_local! {
        static EVENTS: RefCell<Vec<Closure<dyn FnMut(JsValue)>>> = RefCell::new(Vec::new())
    }
}

pub use arch::GlobalEventCallback;
