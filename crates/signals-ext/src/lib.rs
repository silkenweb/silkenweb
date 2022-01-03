use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_signals::signal::Signal;
use paste::paste;
use pin_project::pin_project;

pub trait SignalProduct<Tuple, F> {
    type Output;
    fn signal_ref(self, f: F) -> Self::Output;
}

macro_rules! signal_product{
    ($name:ident; $($index:literal),*) => { paste!{
        impl<$( [< S $index >] , )* F, O> SignalProduct<( $( [< S $index >] , )* ), F> for ( $( [< S $index >] , )* )
        where
            $( [< S $index >] : Signal , )*
            F: FnMut($( & [< S $index >] ::Item, )*) -> O,
        {
            type Output = $name<$( [< S $index >] , )* F>;

            fn signal_ref(self, f: F) -> Self::Output {
                $name {
                    $( [< s $index >] : RefSignal::new(self.$index) , )*
                    f,
                }
            }
        }

        #[pin_project]
        pub struct $name<$( [< S $index >] , )* F>
        where
            $( [< S $index >] : Signal , )*
        {
            $(
                #[pin]
                [< s $index >] : RefSignal< [< S $index >] >,
            )*
            f: F,
        }

        impl<$( [< S $index >] , )* Output, F> Signal for $name<$( [< S $index >] , )* F>
        where
            $( [< S $index >] : Signal , )*
            F: FnMut($( & [< S $index >] ::Item, )*) -> Output,
        {
            type Item = Output;

            fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
                let mut all_done = true;
                let mut any_changed = false;
                let proj = self.project();

                $(let [< i $index >] = proj. [< s $index >] .poll_signal(cx, &mut all_done, &mut any_changed);)*

                signal_result(all_done, any_changed, || (proj.f)(
                    $( [< i $index >] .unwrap(),)*
                ))
            }
        }
    }}
}

signal_product!(Map2; 0, 1);
signal_product!(Map3; 0, 1, 2);

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
