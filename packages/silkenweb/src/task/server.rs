use std::{
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake},
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
use crossbeam::sync::{Parker, Unparker};
use futures::Future;
use silkenweb_macros::cfg_browser;

struct ThreadWaker(Unparker);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Run a future to completion on the current thread.
///
/// This doesn't use the microtask executor, so it's safe to call
/// [render_now] from within the future. It's also safe to call `block_on`
/// recursively.
///
/// [render_now]: super::render_now
pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);

    // Use a `Parker` instance rather than global `thread::park/unpark`, so no one
    // else can steal our `unpark`s and they don't get confused with recursive
    // `block_on` `unpark`s.
    let parker = Parker::new();
    // Make sure we create a new waker each call, rather than using a global, so
    // recursive `block_on`s don't use the same waker.
    let waker = Arc::new(ThreadWaker(parker.unparker().clone())).into();
    let mut cx = Context::from_waker(&waker);

    // Run the future to completion.
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(res) => return res,
            Poll::Pending => parker.park(),
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
