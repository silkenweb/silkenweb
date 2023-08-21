//! Microtask and render queue tools.
//!
//! On wasm32 targets, the microtask queue is the javascript [microtask queue].
//! On other targets, it is simulated with a thread local executor. The render
//! queue holds tasks to be run on an animation frame. Any updates to the DOM
//! via silkenweb are put on the render queue. See [requestAnimationFrame on
//! MDN] for details.
//!
//! [microtask queue]: <https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide>
//! [requestAnimationFrame on MDN]: <https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame>
use std::cell::{Cell, RefCell};

use arch::Raf;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use silkenweb_macros::cfg_browser;

pub(crate) mod local;

// TODO: Docs
pub use arch::{scope, sync_scope};
pub use silkenweb_task::{run_tasks, spawn_local, TaskSignal, TaskSignalVec};

#[cfg_browser(false)]
/// Server only task tools.
pub mod server {
    pub use silkenweb_task::server::{block_on, run_tasks_sync};

    use super::Render;

    /// Synchronous version of [`render_now`][super::render_now].
    ///
    /// This is only available on the server.
    pub fn render_now_sync() {
        Render::with(Render::render_effects);
        run_tasks_sync();
    }
}

#[cfg_browser(false)]
mod arch {
    use futures::Future;

    use super::local;

    pub struct Raf;

    impl Raf {
        pub fn new() -> Self {
            Self
        }

        pub fn request_animation_frame(&self) {}
    }

    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        local::TASK_LOCAL.scope(local::TaskLocal::default(), silkenweb_task::scope(f))
    }

    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        local::TASK_LOCAL.sync_scope(local::TaskLocal::default(), || {
            silkenweb_task::sync_scope(f)
        })
    }
}

#[cfg_browser(true)]
mod arch {
    use std::future::Future;

    use silkenweb_base::window;
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::Render;

    pub struct Raf {
        on_raf: Closure<dyn FnMut(JsValue)>,
    }

    impl Raf {
        pub fn new() -> Self {
            Self {
                on_raf: Closure::wrap(Box::new(|time_stamp: JsValue| {
                    Render::with(|render| render.on_raf(time_stamp.as_f64().unwrap_throw()));
                })),
            }
        }

        pub fn request_animation_frame(&self) {
            window::request_animation_frame(self.on_raf.as_ref().unchecked_ref());
        }
    }

    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        silkenweb_task::scope(f)
    }

    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        silkenweb_task::sync_scope(f)
    }
}

/// Run a closure on the next animation frame.
///
/// An animation frame will be requested with `requestAnimationFrame`.
pub fn on_animation_frame(f: impl FnOnce() + 'static) {
    Render::with(|render| render.on_animation_frame(f));
}

pub(super) fn animation_timestamp() -> impl Signal<Item = f64> {
    Render::with(Render::animation_timestamp)
}

pub(super) fn request_animation_frame() {
    Render::with(Render::request_animation_frame);
}

/// Render any pending updates.
///
/// Tasks on the microtask queue wil be executed first, then the effect queue
/// will be processed. This is mostly useful for testing.
pub async fn render_now() {
    run_tasks().await;
    Render::with(Render::render_effects);
}

pub(crate) struct TaskLocal {
    render: Render,
}

impl Default for TaskLocal {
    fn default() -> Self {
        Self {
            render: Render::new(),
        }
    }
}

struct Render {
    raf: Raf,
    raf_pending: Cell<bool>,
    pending_effects: RefCell<Vec<Box<dyn FnOnce()>>>,
    animation_timestamp_millis: Mutable<f64>,
}

impl Render {
    fn new() -> Self {
        Self {
            raf: Raf::new(),
            raf_pending: Cell::new(false),
            pending_effects: RefCell::new(Vec::new()),
            animation_timestamp_millis: Mutable::new(0.0),
        }
    }

    fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        local::with(|local| f(&local.task.render))
    }

    #[cfg_browser(true)]
    fn on_raf(&self, time_stamp: f64) {
        self.raf_pending.set(false);
        self.animation_timestamp_millis.set(time_stamp);
        self.render_effects();
    }

    fn on_animation_frame(&self, x: impl FnOnce() + 'static) {
        self.pending_effects.borrow_mut().push(Box::new(x));
        self.request_animation_frame();
    }

    fn animation_timestamp(&self) -> impl Signal<Item = f64> {
        // The first timestamp will be from the previous animation or 0.0,
        // `animation_timestamp_millis` will yield 1 timestamp before `base` and the
        // next timestamp after `base`.
        let base = self.base_timestamp();

        self.animation_timestamp_millis.signal().map(move |ts| {
            let relative_ts = ts - base;

            if relative_ts > 0.0 {
                relative_ts
            } else {
                0.0
            }
        })
    }

    #[cfg_browser(true)]
    fn base_timestamp(&self) -> f64 {
        silkenweb_base::window::performance().unwrap().now()
    }

    #[cfg_browser(false)]
    fn base_timestamp(&self) -> f64 {
        self.animation_timestamp_millis.get()
    }

    pub fn render_effects(&self) {
        for effect in self.pending_effects.take() {
            effect();
        }
    }

    fn request_animation_frame(&self) {
        if !self.raf_pending.get() {
            self.raf_pending.set(true);
            self.raf.request_animation_frame();
        }
    }
}
