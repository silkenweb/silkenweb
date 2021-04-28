use silkenweb::{
    elements::{div, hr, Div},
    memo::{MemoFrame, MemoCache},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::define_counter;

// ANCHOR: main
fn main() {
    let count = Signal::new(0);

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(define_counter(&count))
            .child(hr())
            .child(count.read().map({
                // ANCHOR: create_cache
                let counter_elem_cache = MemoCache::new();
                // ANCHOR_END: create_cache

                move |&count| {
                    // ANCHOR: create_frame
                    let counter_elems = counter_elem_cache.frame();
                    // ANCHOR_END: create_frame
                    define_counter_list(&counter_elems, count)
                }
            })),
    );
}
// ANCHOR_END: main

// ANCHOR: define_counter_list
fn define_counter_list(counter_elems: &MemoFrame, count: i64) -> Div {
    let mut counters = div();

    for i in 0..count {
        // ANCHOR: get_cached_counter
        let child = counter_elems.cache(i, || define_counter(&Signal::new(0)).build());
        // ANCHOR_END: get_cached_counter

        counters = counters.child(child);
    }

    counters.build()
}
// ANCHOR_END: define_counter_list
