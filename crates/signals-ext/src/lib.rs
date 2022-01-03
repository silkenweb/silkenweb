use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_signals::signal::Signal;
use pin_project::pin_project;

pub trait SignalProduct<Tuple, F> {
    type Output;
    fn signal_ref(self, f: F) -> Self::Output;
}

impl<S0, S1, F, O> SignalProduct<(S0, S1), F> for (S0, S1)
where
    S0: Signal,
    S1: Signal,
    F: FnMut(&S0::Item, &S1::Item) -> O,
{
    type Output = Map2<S0, S1, F>;

    fn signal_ref(self, f: F) -> Self::Output {
        Map2 {
            s0: RefSignal::new(self.0),
            s1: RefSignal::new(self.1),
            f,
        }
    }
}

#[pin_project]
pub struct Map2<S0, S1, F>
where
    S0: Signal,
    S1: Signal,
{
    #[pin]
    s0: RefSignal<S0>,
    #[pin]
    s1: RefSignal<S1>,
    f: F,
}

impl<S0, S1, Output, F> Signal for Map2<S0, S1, F>
where
    S0: Signal,
    S1: Signal,
    F: FnMut(&S0::Item, &S1::Item) -> Output,
{
    type Item = Output;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut all_done = true;
        let mut any_changed = false;
        let proj = self.project();

        let i0 = proj.s0.poll_signal(cx, &mut all_done, &mut any_changed);
        let i1 = proj.s1.poll_signal(cx, &mut all_done, &mut any_changed);

        signal_result(all_done, any_changed, || (proj.f)(i0.unwrap(), i1.unwrap()))
    }
}

impl<S0, S1, S2, F, O> SignalProduct<(S0, S1, S2), F> for (S0, S1, S2)
where
    S0: Signal,
    S1: Signal,
    S2: Signal,
    F: FnMut(&S0::Item, &S1::Item, &S2::Item) -> O,
{
    type Output = Map3<S0, S1, S2, F>;

    fn signal_ref(self, f: F) -> Self::Output {
        Map3 {
            s0: RefSignal::new(self.0),
            s1: RefSignal::new(self.1),
            s2: RefSignal::new(self.2),
            f,
        }
    }
}

#[pin_project]
pub struct Map3<S0, S1, S2, F>
where
    S0: Signal,
    S1: Signal,
    S2: Signal,
{
    #[pin]
    s0: RefSignal<S0>,
    #[pin]
    s1: RefSignal<S1>,
    #[pin]
    s2: RefSignal<S2>,
    f: F,
}

impl<S0, S1, S2, Output, F> Signal for Map3<S0, S1, S2, F>
where
    S0: Signal,
    S1: Signal,
    S2: Signal,
    F: FnMut(&S0::Item, &S1::Item, &S2::Item) -> Output,
{
    type Item = Output;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut all_done = true;
        let mut any_changed = false;
        let proj = self.project();

        let i0 = proj.s0.poll_signal(cx, &mut all_done, &mut any_changed);
        let i1 = proj.s1.poll_signal(cx, &mut all_done, &mut any_changed);
        let i2 = proj.s2.poll_signal(cx, &mut all_done, &mut any_changed);

        signal_result(all_done, any_changed, || {
            (proj.f)(i0.unwrap(), i1.unwrap(), i2.unwrap())
        })
    }
}

fn signal_result<Output>(
    all_done: bool,
    any_changed: bool,
    mut f: impl FnMut() -> Output,
) -> Poll<Option<Output>> {
    if any_changed {
        Poll::Ready(Some(f()))
    } else if all_done {
        Poll::Ready(None)
    } else {
        Poll::Pending
    }
}

#[pin_project]
struct RefSignal<S: Signal> {
    #[pin]
    signal: Option<S>,
    item: Option<S::Item>,
}

impl<S: Signal> RefSignal<S> {
    pub fn new(s: S) -> Self {
        Self {
            signal: Some(s),
            item: None,
        }
    }

    pub fn poll_signal(
        self: Pin<&mut Self>,
        cx: &mut Context,
        all_done: &mut bool,
        any_changed: &mut bool,
    ) -> Option<&S::Item> {
        let proj = self.project();
        let mut signal = proj.signal;
        let item = proj.item;

        match signal
            .as_mut()
            .as_pin_mut()
            .map(|signal| signal.poll_change(cx))
        {
            None => {}
            Some(Poll::Ready(None)) => {
                signal.set(None);
            }
            Some(Poll::Ready(a)) => {
                *item = a;
                *any_changed = true;
                *all_done = false;
            }
            Some(Poll::Pending) => *all_done = false,
        };

        item.as_ref()
    }
}
