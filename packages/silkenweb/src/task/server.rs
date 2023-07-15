use std::{
    sync::Arc,
    task::{Context, Poll, Wake},
    thread,
    thread::Thread,
};

/// Synchronous version of [render_now].
///
/// This is only available on the server.
///
/// # Panics
///
/// This panic if called on browser platforms.
///
/// [render_now]: super::render_now
pub use arch::render_now_sync;
/// Run a future with a local task queue.
///
/// On the server, this creates a [`tokio`] task local queue for any futures
/// spawned with [`spawn_local`].
///
/// On the browser, this does nothing and returns the original future.
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
pub use arch::scope;
/// Synchronous version of [`scope`].
pub use arch::sync_scope;
use futures::Future;
use silkenweb_macros::cfg_browser;

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

#[cfg_browser(true)]
mod arch {
    use std::future::Future;

    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        f
    }

    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }

    pub fn render_now_sync() {
        panic!("Synchronous rendering is only available in the browser");
    }
}

#[cfg_browser(false)]
mod arch {
    use std::future::Future;

    use crate::task::{local, Render};

    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        local::TASK_LOCAL.scope(local::TaskLocal::default(), f)
    }

    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        local::TASK_LOCAL.sync_scope(local::TaskLocal::default(), f)
    }

    pub fn render_now_sync() {
        crate::task::arch::run();
        Render::with(Render::render_effects);
    }
}
