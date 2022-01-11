use std::rc::Rc;

use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::{
    html::{button, div, hr, Button, Div},
    mount, ElementBuilder, ElementEvents, ParentBuilder,
};

fn main() {
    let list = Rc::new(MutableVec::new());

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(
                div()
                    .child(pop_button(list.clone()))
                    .text_signal(list.signal_vec().len().map(|len| format!("{}", len)))
                    .child(push_button(list.clone())),
            )
            .child(hr())
            .child(div().children_signal(list.signal_vec().map(|_| define_counter()))),
    );
}

#[derive(Copy, Clone)]
pub struct Counter;

fn push_button(list: Rc<MutableVec<Counter>>) -> Button {
    button()
        .on_click(move |_, _| list.lock_mut().push_cloned(Counter))
        .text("+")
        .build()
}

fn pop_button(list: Rc<MutableVec<Counter>>) -> Button {
    button()
        .on_click(move |_, _| {
            list.lock_mut().pop();
        })
        .text("-")
        .build()
}

pub fn define_counter() -> Div {
    let count = Rc::new(Mutable::new(0));
    let count_text = count.signal_ref(|i| format!("{}", i));

    div()
        .child(define_button("-", -1, count.clone()))
        .text_signal(count_text)
        .child(define_button("+", 1, count))
        .build()
}

pub fn define_button(label: &str, delta: i64, count: Rc<Mutable<i64>>) -> Button {
    button()
        .on_click(move |_, _| {
            count.replace_with(move |i| *i + delta);
        })
        .text(label)
        .build()
}
