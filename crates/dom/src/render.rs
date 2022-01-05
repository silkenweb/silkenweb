use std::cell::{Cell, RefCell};

use futures_signals::signal::{Mutable, Signal, SignalExt};
use js_sys::Promise;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::window;

// TODO: This causes a lot of cloning. Decide whether to run this synchronously
// earlier, so we don't always need to clone.
pub fn queue_update(f: impl 'static + FnOnce()) {
        RENDER.with(|r| r.queue_update(f));
}

/// Run a closure after the next render.
pub fn after_render(x: impl 'static + FnOnce()) {
    RENDER.with(|r| r.after_render(x));
}

pub fn animation_timestamp() -> impl Signal<Item = f64> {
    RENDER.with(Render::animation_timestamp)
}

// TODO: This should work when a microtask creates more microtasks, but needs
// testing. For example a `Signal::map` that updates a `Mutable` with another
// listener.
async fn wait_for_microtasks() {
    let promise = Promise::resolve(&JsValue::NULL);
    JsFuture::from(promise).await.unwrap_throw();
}

/// Render any pending updates.
///
/// This is mostly useful for testing.
pub async fn render_updates() {
    wait_for_microtasks().await;
    RENDER.with(Render::render_updates);
}

pub fn request_render_updates() {
    RENDER.with(Render::request_render_updates);
}

struct Render {
    raf_pending: Cell<bool>,
    pending_updates: RefCell<Vec<Box<dyn FnOnce()>>>,
    pending_effects: RefCell<Vec<Box<dyn FnOnce()>>>,
    on_animation_frame: Closure<dyn FnMut(JsValue)>,
    animation_timestamp_millis: Mutable<f64>,
}

impl Render {
    pub fn new() -> Self {
        Self {
            raf_pending: Cell::new(false),
            pending_updates: RefCell::new(Vec::new()),
            pending_effects: RefCell::new(Vec::new()),
            on_animation_frame: Closure::wrap(Box::new(move |time_stamp: JsValue| {
                RENDER.with(|render| {
                    render.raf_pending.set(false);
                    render.update_animations(time_stamp.as_f64().unwrap_throw());
                    render.render_updates();
                });
            })),
            animation_timestamp_millis: Mutable::new(0.0),
        }
    }

    fn queue_update(&self, x: impl 'static + FnOnce()) {
        self.pending_updates.borrow_mut().push(Box::new(x));
        self.request_render_updates();
    }

    fn after_render(&self, x: impl 'static + FnOnce()) {
        self.pending_effects.borrow_mut().push(Box::new(x));
        self.request_render_updates();
    }

    fn animation_timestamp(&self) -> impl Signal<Item = f64> {
        let base_timestamp = self.animation_timestamp_millis.get();
        self.animation_timestamp_millis
            .signal()
            .map(move |t| t - base_timestamp)
    }

    pub fn render_updates(&self) {
        for update in self.pending_updates.take() {
            update();
        }

        for effect in self.pending_effects.take() {
            effect();
        }
    }

    fn request_render_updates(&self) {
        if !self.raf_pending.get() {
            self.raf_pending.set(true);

            window()
                .request_animation_frame(self.on_animation_frame.as_ref().unchecked_ref())
                .unwrap_throw();
        }
    }

    fn update_animations(&self, timestamp: f64) {
        self.animation_timestamp_millis.set(timestamp);
    }
}

thread_local!(
    static RENDER: Render = Render::new();
);
