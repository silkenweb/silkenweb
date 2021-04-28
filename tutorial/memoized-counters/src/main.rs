use silkenweb::{
    elements::{button, div, Button, Div, DivBuilder},
    memo::{Memo, MemoCache},
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    let count = Signal::new(0);

    mount(
        "app",
        define_counter(&count).child(count.read().map({
            let counter_elem_cache = MemoCache::default();

            move |&count| {
                let counter_elems = counter_elem_cache.frame();
                define_counter_list(&counter_elems, count)
            }
        })),
    );
}

fn define_counter_list(counter_elems: &Memo, count: i64) -> Div {
    let mut counters = div();

    for i in 0..count {
        let child = counter_elems.cache(i, || define_counter(&Signal::new(0)).build());

        counters = counters.child(child);
    }

    counters.build()
}

fn define_counter(count: &Signal<i64>) -> DivBuilder {
    div()
        .child(define_button("-", -1, count.write()))
        .text(count.read().map(|i| format!("{}", i)))
        .child(define_button("+", 1, count.write()))
}

fn define_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
