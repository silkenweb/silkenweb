//! Utilities for tacking time.
use silkenweb_macros::cfg_browser;

#[cfg_browser(true)]
mod arch {
    use std::{future::Future, pin::Pin, task, time::Duration};

    use futures::Stream;
    use gloo_timers::future::{IntervalStream, TimeoutFuture};
    use pin_project::pin_project;
    use wasm_bindgen::UnwrapThrowExt;

    #[derive(Debug)]
    #[pin_project]
    pub struct Sleep(#[pin] TimeoutFuture);

    impl Future for Sleep {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }

    pub fn sleep(duration: Duration) -> Sleep {
        Sleep(gloo_timers::future::sleep(duration))
    }

    #[derive(Debug)]
    #[pin_project]
    pub struct Interval(#[pin] IntervalStream);

    impl Stream for Interval {
        type Item = ();

        fn poll_next(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> task::Poll<Option<Self::Item>> {
            self.project().0.poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (std::usize::MAX, None)
        }
    }

    pub fn interval(period: Duration) -> Interval {
        let period_ms = u32::try_from(period.as_millis())
            .expect_throw("failed to cast the duration into a u32 with Duration::as_millis.");
        Interval(IntervalStream::new(period_ms))
    }
}

#[cfg_browser(false)]
mod arch {
    use std::{future::Future, pin::Pin, task, time::Duration};

    use futures::{stream::Skip, Stream, StreamExt};
    use pin_project::pin_project;
    use tokio_stream::wrappers::IntervalStream;

    #[derive(Debug)]
    #[pin_project]
    pub struct Sleep(#[pin] tokio::time::Sleep);

    impl Future for Sleep {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }

    pub fn sleep(duration: Duration) -> Sleep {
        Sleep(tokio::time::sleep(duration))
    }

    #[derive(Debug)]
    #[pin_project]
    pub struct Interval(#[pin] Skip<IntervalStream>);

    impl Stream for Interval {
        type Item = ();

        fn poll_next(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> task::Poll<Option<Self::Item>> {
            self.project().0.poll_next(cx).map(|_| Some(()))
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX, None)
        }
    }

    pub fn interval(period: Duration) -> Interval {
        Interval(IntervalStream::new(tokio::time::interval(period)).skip(1))
    }
}

/// A stream that yields `()` periodically.
///
/// Yield `()` every `period`. The first value will be produced after a delay of
/// `period`.
///
/// # Panics
///
/// If `duration` can't be converted into a [`u32`] in milliseconds.
pub use arch::interval;
/// Sleep for `duration`.
///
/// # Panics
///
/// If `duration` can't be converted into a [`u32`] in milliseconds.
pub use arch::sleep;
/// [`Stream`] for [`interval`]
///
/// [`Stream`]: futures::Stream
pub use arch::Interval;
/// [`Future`] for [`sleep`]
///
/// [`Future`]: std::future::Future
pub use arch::Sleep;
