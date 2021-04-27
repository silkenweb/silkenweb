use silkenweb::{
    elements::{button, div, Button, DivBuilder},
    memo::MemoCache,
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    console_error_panic_hook::set_once();
    let child_counts = MemoCache::default();
    let count = Signal::new(0);

    mount(
        "app",
        counter(&count).child(count.read().map(move |&count| {
            let child_counts = child_counts.frame();

            let mut counters = div();

            for i in 0..count {
                let child = child_counts.cache(i, || counter(&Signal::new(0)).build());

                counters = counters.child(child);
            }

            counters
        })),
    );
}

fn counter(count: &Signal<i64>) -> DivBuilder {
    div()
        .child(update_count("-", -1, count.write()))
        .text(count.read().map(|i| format!("{}", i)))
        .child(update_count("+", 1, count.write()))
}

fn update_count(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
