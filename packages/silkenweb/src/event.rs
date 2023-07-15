use silkenweb_macros::cfg_browser;

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
