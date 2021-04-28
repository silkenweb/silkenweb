use silkenweb::{
    elements::{div, hr, Div},
    memo::{Memo, MemoCache},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::define_counter;

fn main() {
    let count = Signal::new(0);

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(define_counter(&count))
            .child(hr())
            .child(count.read().map({
                let counter_elem_cache = MemoCache::new();

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
