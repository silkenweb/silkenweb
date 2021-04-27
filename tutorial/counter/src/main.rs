use silkenweb::{
    elements::{button, div, Button},
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    console_error_panic_hook::set_once();

    let count = Signal::new(0);

    mount(
        "app",
        div()
            .child(update_count("-", -1, count.write()))
            .text(count.read().map(|i| format!("{}", i)))
            .child(update_count("+", 1, count.write())),
    );
}

fn update_count(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
