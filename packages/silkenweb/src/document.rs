use silkenweb_base::document;
use wasm_bindgen::{intern, prelude::Closure, JsCast, JsValue};
use web_sys::MouseEvent;

pub struct EventCallback {
    name: &'static str,
    callback: Closure<dyn FnMut(JsValue)>,
}

impl EventCallback {
    // TODO: Do nothing on non-wasm32 targets
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

// TODO: Other events
pub fn on_mouseup(f: impl FnMut(MouseEvent) + 'static) -> EventCallback {
    EventCallback::new("mouseup", f)
}

pub fn on_mousemove(f: impl FnMut(MouseEvent) + 'static) -> EventCallback {
    EventCallback::new("mousemove", f)
}
