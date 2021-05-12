use std::cell::{Cell, RefCell};

use silkenweb_reactive::signal::{ReadSignal, Signal};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::window;

pub fn queue_update(x: impl 'static + FnOnce()) {
    RENDER.with(|r| r.queue_update(x));
}

/// Run a closure after the next render.
pub fn after_render(x: impl 'static + FnOnce()) {
    RENDER.with(|r| r.after_render(x));
}

pub(crate) fn animation_timestamp() -> ReadSignal<f64> {
    RENDER.with(Render::animation_timestamp)
}

/// Render any pending updates.
///
/// This is mostly useful for testing.
pub fn render_updates() {
    RENDER.with(Render::render_updates);
}

pub(super) fn request_render_updates() {
    RENDER.with(Render::request_render_updates);
}

struct Render {
    raf_pending: Cell<bool>,
    pending_updates: RefCell<Vec<Box<dyn FnOnce()>>>,
    pending_effects: RefCell<Vec<Box<dyn FnOnce()>>>,
    on_animation_frame: Closure<dyn FnMut(JsValue)>,
    animation_timestamp_millis: Signal<f64>,
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
                    render.update_animations(time_stamp.as_f64().unwrap());
                    render_updates();
                });
            })),
            animation_timestamp_millis: Signal::new(0.0),
        }
    }

    fn queue_update(&self, x: impl 'static + FnOnce()) {
        self.pending_updates.borrow_mut().push(Box::new(x));
        self.request_render_updates();
    }

    fn after_render(&self, x: impl 'static + FnOnce()) {
        self.pending_effects.borrow_mut().push(Box::new(x));
    }

    fn animation_timestamp(&self) -> ReadSignal<f64> {
        self.animation_timestamp_millis.read()
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
                .unwrap();
        }
    }

    fn update_animations(&self, timestamp: f64) {
        self.animation_timestamp_millis.write().set(timestamp);
    }
}

thread_local!(
    static RENDER: Render = Render::new();
);
