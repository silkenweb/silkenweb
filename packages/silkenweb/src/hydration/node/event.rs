use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

pub struct EventCallback {
    target: web_sys::Node,
    name: &'static str,
    callback: Closure<dyn FnMut(JsValue)>,
}

impl EventCallback {
    /// `f` must be `'static` as JS callbacks are called once the stack frame is
    /// finished. See the [Closure::wrap] and
    /// <https://github.com/rustwasm/wasm-bindgen/issues/1914#issuecomment-566488497>
    pub fn new(
        target: web_sys::Node,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
    ) -> Self {
        let callback = Closure::new(f);
        target
            .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref())
            .unwrap_throw();

        Self {
            target,
            name,
            callback,
        }
    }
}

impl Drop for EventCallback {
    fn drop(&mut self) {
        self.target
            .remove_event_listener_with_callback(
                self.name,
                self.callback.as_ref().as_ref().unchecked_ref(),
            )
            .unwrap_throw();
    }
}
