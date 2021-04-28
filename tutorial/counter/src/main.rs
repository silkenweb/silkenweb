use silkenweb::{
    elements::{button, div, Button},
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    let count = Signal::new(0);
    let count_text = count.read().map(|i| format!("{}", i));
    let app = div()
        .child(render_button("-", -1, count.write()))
        .text(count_text)
        .child(render_button("+", 1, count.write()));

    mount("app", app);
}

fn render_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
