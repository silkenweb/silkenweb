use std::rc::Rc;

use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::{
    elements::{button, div, hr, Button, Div, DivBuilder},
    mount, Builder, signal, ParentBuilder,
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
                    .text(signal(list.signal_vec_cloned().len().map(|len| format!("{}", len))))
                    .child(push_button(list.clone())),
            )
            .child(hr())
            .child(div().dyn_children(list.signal_vec_cloned())),
    );
}

fn push_button(list: Rc<MutableVec<Div>>) -> Button {
    button()
        .on_click(move |_, _| list.lock_mut().push_cloned(define_counter().build()))
        .text("+")
        .build()
}

fn pop_button(list: Rc<MutableVec<Div>>) -> Button {
    button()
        .on_click(move |_, _| {
            list.lock_mut().pop();
        })
        .text("-")
        .build()
}

pub fn define_counter() -> DivBuilder {
    let count = Rc::new(Mutable::new(0));
    let count_text = count.signal().map(|i| format!("{}", i));

    div()
        .child(define_button("-", -1, count.clone()))
        .text(signal(count_text))
        .child(define_button("+", 1, count))
}

pub fn define_button(label: &str, delta: i64, count: Rc<Mutable<i64>>) -> Button {
    button()
        .on_click(move |_, _| {
            count.replace_with(move |i| *i + delta);
        })
        .text(label)
        .build()
}
