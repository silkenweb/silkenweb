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
        render_counter(&count).child(count.read().map({
            let counter_elem_cache = MemoCache::default();

            move |&count| {
                let counter_elems = counter_elem_cache.frame();
                render_counter_list(&counter_elems, count)
            }
        })),
    );
}

fn render_counter_list(counter_elems: &Memo, count: i64) -> Div {
    let mut counters = div();

    for i in 0..count {
        let child = counter_elems.cache(i, || render_counter(&Signal::new(0)).build());

        counters = counters.child(child);
    }

    counters.build()
}

fn render_counter(count: &Signal<i64>) -> DivBuilder {
    div()
        .child(render_button("-", -1, count.write()))
        .text(count.read().map(|i| format!("{}", i)))
        .child(render_button("+", 1, count.write()))
}

fn render_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
