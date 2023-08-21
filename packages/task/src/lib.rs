use clonelet::clone;
use futures::{Future, FutureExt, StreamExt};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, MutableVecLockMut, SignalVec, SignalVecExt, VecDiff},
};
use silkenweb_macros::cfg_browser;

#[cfg_browser(false)]
/// Server only task tools.
pub mod server {
    use std::{
        pin::pin,
        sync::Arc,
        task::{Context, Poll, Wake},
    };

    use crossbeam::sync::{Parker, Unparker};
    use futures::Future;

    /// Synchronous version of [`run_tasks`][super::run_tasks].
    ///
    /// This is only available on the server.
    pub fn run_tasks_sync() {
        super::arch::run_tasks_sync()
    }

    struct ThreadWaker(Unparker);

    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    /// Run a future to completion on the current thread.
    ///
    /// This doesn't use the microtask executor, so it's safe to call
    /// [run_tasks] from within the future. It's also safe to call `block_on`
    /// recursively.
    ///
    /// [run_tasks]: super::run_tasks
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
}

#[cfg_browser(false)]
mod arch {
    use std::{cell::RefCell, future::Future};

    use futures::{
        executor::{LocalPool, LocalSpawner},
        task::LocalSpawnExt,
    };
    use tokio::task_local;

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

    task_local! {
        pub static RUNTIME: Runtime;
    }

    fn with_runtime<R>(f: impl FnOnce(&Runtime) -> R) -> R {
        match RUNTIME.try_with(f) {
            Ok(r) => r,
            Err(_) => panic!("Must be run from within `silkenweb_task::task::scope`"),
        }
    }

    pub fn scope<Fut>(f: Fut) -> impl Future<Output = Fut::Output>
    where
        Fut: Future,
    {
        RUNTIME.scope(Runtime::default(), f)
    }

    pub fn sync_scope<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        RUNTIME.sync_scope(Runtime::default(), f)
    }

    pub async fn run_tasks() {
        run_tasks_sync()
    }

    pub fn run_tasks_sync() {
        with_runtime(|runtime| runtime.executor.borrow_mut().run_until_stalled())
    }

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        with_runtime(|runtime| runtime.spawner.spawn_local(future).unwrap())
    }
}

#[cfg_browser(true)]
mod arch {
    use std::future::Future;

    use js_sys::Promise;
    use wasm_bindgen::{JsValue, UnwrapThrowExt};
    use wasm_bindgen_futures::JsFuture;

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

    // Microtasks are run in the order they were queued in Javascript, so we just
    // put a task on the queue and `await` it.
    pub async fn run_tasks() {
        let wait_for_microtasks = Promise::resolve(&JsValue::NULL);
        JsFuture::from(wait_for_microtasks).await.unwrap_throw();
    }

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future)
    }
}

/// Run futures on the microtask queue, until no more progress can be
/// made.
///
/// Don't call this from a future already on the microtask queue.
pub async fn run_tasks() {
    arch::run_tasks().await
}

/// Run a future with a local task queue.
///
/// On the server, this creates a [`tokio`] task local queue. You can put
/// futures on this queue with [`spawn_local`].
///
/// On the browser, this does nothing and returns the original future.
pub use arch::scope;
/// Synchronous version of [`scope`].
pub use arch::sync_scope;

/// Spawn a future on the microtask queue.
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    arch::spawn_local(future)
}

/// [`Signal`] methods that require a task queue.
pub trait TaskSignal: Signal {
    /// Convert `self` to a [`Mutable`].
    ///
    /// This uses the microtask queue to spawn a future that drives the signal.
    /// The resulting `Mutable` can be used to memoize the signal, allowing many
    /// signals to be derived from it.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use futures_signals::signal::Mutable;
    /// # use silkenweb_task::{sync_scope, server::run_tasks_sync, TaskSignal};
    /// #
    /// let source = Mutable::new(0);
    /// let signal = source.signal();
    ///
    /// // A scope isn't required on browser platforms
    /// sync_scope(|| {
    ///     let copy = signal.to_mutable();
    ///     assert_eq!(copy.get(), 0);
    ///     source.set(1);
    ///     run_tasks_sync();
    ///     assert_eq!(copy.get(), 1);
    /// });
    /// ```
    fn to_mutable(self) -> ReadOnlyMutable<Self::Item>;

    /// Run `callback` on each signal value.
    ///
    /// The future is spawned on the microtask queue. This is equivalent to
    /// `spawn_local(sig.for_each(callback))`.
    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(Self::Item) -> U + 'static;
}

impl<Sig> TaskSignal for Sig
where
    Sig: Signal + 'static,
{
    fn to_mutable(self) -> ReadOnlyMutable<Self::Item> {
        let mut s = Box::pin(self.to_stream());
        let first_value = s
            .next()
            .now_or_never()
            .expect("A `Signal`'s initial value must be `Ready` immediately")
            .expect("`Signal`s must have an initial value");
        let mutable = Mutable::new(first_value);

        spawn_local({
            clone!(mutable);

            async move {
                while let Some(value) = s.next().await {
                    mutable.set(value);
                }
            }
        });

        mutable.read_only()
    }

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(Self::Item) -> U + 'static,
    {
        spawn_local(self.for_each(callback));
    }
}

/// [`SignalVec`] methods that require a task queue.
pub trait TaskSignalVec: SignalVec {
    /// Convert `self` to a [`MutableVec`].
    ///
    /// This uses the microtask queue to spawn a future that drives the signal.
    /// The resulting `MutableVec` can be used to memoize the signal, allowing
    /// many signals to be derived from it.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use futures_signals::signal_vec::MutableVec;
    /// # use silkenweb_task::{sync_scope, server::run_tasks_sync, TaskSignalVec};
    /// #
    /// let source = MutableVec::new();
    /// let signal = source.signal_vec();
    ///
    /// // A scope isn't required on browser platforms
    /// sync_scope(|| {
    ///     let copy = signal.to_mutable();
    ///     assert!(copy.lock_ref().is_empty());
    ///     source.lock_mut().push_cloned(1);
    ///     run_tasks_sync();
    ///     assert_eq!(*copy.lock_ref(), [1]);
    /// });
    /// ```
    fn to_mutable(self) -> MutableVec<Self::Item>;

    /// Run `callback` on each signal delta.
    ///
    /// The future is spawned on the microtask queue. This is equivalent to
    /// `spawn_local(sig.for_each(callback))`.
    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(VecDiff<Self::Item>) -> U + 'static;
}

impl<Sig> TaskSignalVec for Sig
where
    Self::Item: Clone + 'static,
    Sig: SignalVec + 'static,
{
    fn to_mutable(self) -> MutableVec<Self::Item> {
        let mv = MutableVec::new();

        self.spawn_for_each({
            clone!(mv);

            move |diff| {
                MutableVecLockMut::apply_vec_diff(&mut mv.lock_mut(), diff);
                async {}
            }
        });

        mv
    }

    fn spawn_for_each<U, F>(self, callback: F)
    where
        U: Future<Output = ()> + 'static,
        F: FnMut(VecDiff<Self::Item>) -> U + 'static,
    {
        spawn_local(self.for_each(callback));
    }
}
