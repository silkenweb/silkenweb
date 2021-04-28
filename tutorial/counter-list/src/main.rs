use silkenweb::{
    elements::{div, hr, Div},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::define_counter;

// ANCHOR: main
fn main() {
    let count = Signal::new(0);
    // ANCHOR: counter_list
    let counter_list = count.read().map(move |&count| define_counter_list(count));
    // ANCHOR_END: counter_list

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(define_counter(&count))
            .child(hr())
            .child(counter_list),
    );
}
// ANCHOR_END: main

// ANCHOR: define_counter_list
fn define_counter_list(count: i64) -> Div {
    let mut counters = div();

    for _i in 0..count {
        counters = counters.child(define_counter(&Signal::new(0)));
    }

    counters.build()
}
// ANCHOR_END: define_counter_list
