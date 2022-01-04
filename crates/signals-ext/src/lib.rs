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
    ($name:ident; $( ($index:tt, $signal_var:ident, $signal_type:ident, $item_var:ident) ),*) => {
        impl<$( $signal_type , )* F, O> SignalProduct<( $( $signal_type , )* ), F> for ( $( $signal_type , )* )
        where
            $( $signal_type : Signal , )*
            F: FnMut($( & $signal_type ::Item, )*) -> O,
        {
            type Output = $name<$( $signal_type , )* F>;

            fn signal_ref(self, f: F) -> Self::Output {
                $name {
                    $( $signal_var : RefSignal::new(self.$index) , )*
                    f,
                }
            }
        }

        #[must_use = "Signals do nothing unless polled"]
        #[pin_project]
        pub struct $name<$( $signal_type , )* F>
        where
            $( $signal_type : Signal , )*
        {
            $(
                #[pin]
                $signal_var : RefSignal< $signal_type >,
            )*
            f: F,
        }

        impl<$( $signal_type , )* Output, F> Signal for $name<$( $signal_type , )* F>
        where
            $( $signal_type : Signal , )*
            F: FnMut($( & $signal_type ::Item, )*) -> Output,
        {
            type Item = Output;

            fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
                let mut all_done = true;
                let mut any_changed = false;
                let proj = self.project();

                $(let $item_var = proj. $signal_var .poll_signal(cx, &mut all_done, &mut any_changed);)*

                signal_result(all_done, any_changed, || (proj.f)(
                    $( $item_var .unwrap(),)*
                ))
            }
        }
    }
}

macro_rules! signal_products{
    ( ( $($index:tt),* ); []) => {};
    ( ( $($index:tt),* ); [$count:tt $(, $tail_count:tt)*] ) => { paste! {
        signal_product!( [< Map $count >] ; $( ( $index, [< s $index >], [< S $index >], [< i $index >]  ) ),*);
        signal_products!(($($index, )* $count); [$($tail_count),*]);
    }}
}

signal_products!((0, 1); [2, 3, 4, 5, 6, 7, 8, 9, 10]);

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
