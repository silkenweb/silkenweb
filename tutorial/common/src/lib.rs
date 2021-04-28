#![allow(clippy::must_use_candidate)]
use silkenweb::{
    elements::{button, div, Button, DivBuilder},
    signal::{ReadSignal, Signal, WriteSignal},
    Builder,
};

// ANCHOR: define_counter
pub fn define_counter(count: &Signal<i64>) -> DivBuilder {
    let count_text: ReadSignal<String> = count.read().map(|i| format!("{}", i));

    div()
        .child(define_button("-", -1, count.write()))
        .text(count_text)
        .child(define_button("+", 1, count.write()))
}
// ANCHOR_END: define_counter

// ANCHOR: define_button
pub fn define_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
// ANCHOR_END: define_button
