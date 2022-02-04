use std::cell::{Cell, RefCell};

use futures_signals::signal::{Mutable, Signal};

use crate::tasks::wait_for_microtasks;

#[cfg(feature = "server-side-render")]
mod raf {
    pub struct Raf;

    impl Raf {
        pub fn new() -> Self {
            Self
        }

        pub fn request_render(&self) {}
    }
}

#[cfg(not(feature = "server-side-render"))]
mod raf {
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    use super::RENDER;
    use crate::global::window;

    pub struct Raf {
        on_animation_frame: Closure<dyn FnMut(JsValue)>,
    }

    impl Raf {
        pub fn new() -> Self {
            Self {
                on_animation_frame: Closure::wrap(Box::new(|time_stamp: JsValue| {
                    RENDER.with(|render| {
                        render.on_animation_frame(time_stamp.as_f64().unwrap_throw())
                    });
                })),
            }
        }

        pub fn request_render(&self) {
            window::request_animation_frame(self.on_animation_frame.as_ref().unchecked_ref());
        }
    }
}

pub(super) fn queue_update(f: impl FnOnce() + 'static) {
    RENDER.with(|r| r.queue_update(f));
}

/// Run a closure after the next render.
pub fn after_render(f: impl FnOnce() + 'static) {
    RENDER.with(|r| r.after_render(f));
}

pub fn animation_timestamp() -> impl Signal<Item = f64> {
    RENDER.with(Render::animation_timestamp)
}

/// Render any pending updates.
///
/// This is mostly useful for testing.
pub async fn render_now() {
    wait_for_microtasks().await;
    RENDER.with(Render::render_updates);
}

#[cfg(not(target_arch = "wasm32"))]
pub mod server {
    use std::{
        sync::Arc,
        task::{Context, Poll, Wake},
        thread,
        thread::Thread,
    };

    use futures::Future;

    use super::{Render, RENDER};
    use crate::tasks;

    pub fn render_now_sync() {
        tasks::run();
        RENDER.with(Render::render_updates);
    }

    struct ThreadWaker(Thread);

    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    // Adapted from <https://doc.rust-lang.org/stable/std/task/trait.Wake.html>
    /// Run a future to completion on the current thread.
    pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
        // Pin the future so it can be polled.
        let mut fut = Box::pin(fut);

        // Create a new context to be passed to the future.
        let t = thread::current();
        let waker = Arc::new(ThreadWaker(t)).into();
        let mut cx = Context::from_waker(&waker);

        // Run the future to completion.
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(res) => return res,
                Poll::Pending => thread::park(),
            }
        }
    }
}

pub fn request_render() {
    RENDER.with(Render::request_render);
}

struct Render {
    raf: raf::Raf,
    raf_pending: Cell<bool>,
    pending_updates: RefCell<Vec<Box<dyn FnOnce()>>>,
    pending_effects: RefCell<Vec<Box<dyn FnOnce()>>>,
    animation_timestamp_millis: Mutable<f64>,
}

impl Render {
    fn new() -> Self {
        Self {
            raf: raf::Raf::new(),
            raf_pending: Cell::new(false),
            pending_updates: RefCell::new(Vec::new()),
            pending_effects: RefCell::new(Vec::new()),
            animation_timestamp_millis: Mutable::new(0.0),
        }
    }

    #[cfg(not(feature = "server-side-render"))]
    fn on_animation_frame(&self, time_stamp: f64) {
        self.raf_pending.set(false);
        self.animation_timestamp_millis.set(time_stamp);
        self.render_updates();
    }

    fn queue_update(&self, x: impl FnOnce() + 'static) {
        self.pending_updates.borrow_mut().push(Box::new(x));
        self.request_render();
    }

    fn after_render(&self, x: impl FnOnce() + 'static) {
        self.pending_effects.borrow_mut().push(Box::new(x));
        self.request_render();
    }

    fn animation_timestamp(&self) -> impl Signal<Item = f64> {
        let base_timestamp = self.animation_timestamp_millis.get();
        self.animation_timestamp_millis
            .signal_ref(move |t| t - base_timestamp)
    }

    pub fn render_updates(&self) {
        for update in self.pending_updates.take() {
            update();
        }

        for effect in self.pending_effects.take() {
            effect();
        }
    }

    fn request_render(&self) {
        if !self.raf_pending.get() {
            self.raf_pending.set(true);
            self.raf.request_render();
        }
    }
}

thread_local!(
    static RENDER: Render = Render::new();
);
