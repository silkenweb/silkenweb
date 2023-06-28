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

use arch::{wait_for_microtasks, Raf};
use futures::Future;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use silkenweb_macros::cfg_browser;

pub(crate) mod local;

/// Spawn a future on the microtask queue.
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    local::with(|local| local.task.runtime.spawn_local(future))
}

#[cfg_browser(false)]
mod arch {
    use std::{cell::RefCell, future::Future};

    use futures::{
        executor::{LocalPool, LocalSpawner},
        task::LocalSpawnExt,
    };

    use super::local;

    pub struct Raf;

    impl Raf {
        pub fn new() -> Self {
            Self
        }

        pub fn request_animation_frame(&self) {}
    }

    /// Run futures queued with `spawn_local`, until no more progress can be
    /// made. Don't call this from a future spawned using `spawn_local`, use
    /// `render::block_on`
    pub async fn wait_for_microtasks() {
        run();
    }

    pub fn run() {
        local::with(|local| local.task.runtime.executor.borrow_mut().run_until_stalled())
    }

    pub struct Runtime {
        executor: RefCell<LocalPool>,
        spawner: LocalSpawner,
    }

    impl Default for Runtime {
        fn default() -> Self {
            let executor = RefCell::new(LocalPool::new());
            let spawner = executor.borrow().spawner();

            Self { executor, spawner }
        }
    }

    impl Runtime {
        pub fn spawn_local<F>(&self, future: F)
        where
            F: Future<Output = ()> + 'static,
        {
            self.spawner.spawn_local(future).unwrap()
        }
    }
}

#[cfg_browser(true)]
mod arch {
    use std::future::Future;

    use js_sys::Promise;
    use silkenweb_base::window;
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
    use wasm_bindgen_futures::JsFuture;

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

    // Microtasks are run in the order they were queued in Javascript, so we just
    // put a task on the queue and `await` it.
    pub async fn wait_for_microtasks() {
        let wait_for_microtasks = Promise::resolve(&JsValue::NULL);
        JsFuture::from(wait_for_microtasks).await.unwrap_throw();
    }

    #[derive(Default)]
    pub struct Runtime;

    impl Runtime {
        pub fn spawn_local<F>(&self, future: F)
        where
            F: Future<Output = ()> + 'static,
        {
            wasm_bindgen_futures::spawn_local(future)
        }
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
    wait_for_microtasks().await;
    Render::with(Render::render_effects);
}

/// Server only tools.
///
/// Not available on wasm32 targets.
#[cfg_browser(false)]
pub mod server {
    use std::{
        sync::Arc,
        task::{Context, Poll, Wake},
        thread,
        thread::Thread,
    };

    use futures::Future;

    use super::{
        arch,
        local::{TaskLocal, TASK_LOCAL},
        Render,
    };

    /// Run a future with a local task queue.
    ///
    /// This creates a [`tokio`] task local queue for any futures spawned with
    /// [`spawn_local`].
    ///
    /// ```
    /// # use silkenweb::{prelude::*, task::{render_now, server::{block_on, scope}}};
    /// # use html::div;
    /// #
    /// block_on(scope(async {
    ///     let text = Mutable::new("Hello!");
    ///     let app: Node = div().text(Sig(text.signal())).into();
    ///     assert_eq!(app.to_string(), "<div></div>");
    ///     render_now().await;
    ///     assert_eq!(app.to_string(), "<div>Hello!</div>");
    /// }))
    /// ```
    ///
    /// [`spawn_local`]: super::spawn_local
    /// [`Signal`]: futures_signals::signal::Signal
    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        TASK_LOCAL.scope(TaskLocal::default(), f)
    }

    /// Synchronous version of [`scope`].
    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        TASK_LOCAL.sync_scope(TaskLocal::default(), f)
    }

    /// Synchronous version of [render_now].
    ///
    /// [render_now]: super::render_now
    pub fn render_now_sync() {
        arch::run();
        Render::with(Render::render_effects);
    }

    struct ThreadWaker(Thread);

    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    // Adapted from <https://doc.rust-lang.org/stable/std/task/trait.Wake.html>

    /// Run a future to completion on the current thread.
    ///
    /// This doesn't use the microtask executor, so it's safe to call
    /// [render_now] from within the future.
    ///
    /// [render_now]: super::render_now
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

pub(crate) struct TaskLocal {
    runtime: arch::Runtime,
    render: Render,
}

impl Default for TaskLocal {
    fn default() -> Self {
        Self {
            runtime: arch::Runtime::default(),
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
