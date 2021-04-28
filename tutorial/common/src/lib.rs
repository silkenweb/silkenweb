#![allow(clippy::must_use_candidate)]
use silkenweb::{
    elements::{button, div, Button, DivBuilder},
    signal::{Signal, WriteSignal},
    Builder,
};

pub fn render_counter(count: &Signal<i64>) -> DivBuilder {
    let count_text = count.read().map(|i| format!("{}", i));

    div()
        .child(render_button("-", -1, count.write()))
        .text(count_text)
        .child(render_button("+", 1, count.write()))
}

pub fn render_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
