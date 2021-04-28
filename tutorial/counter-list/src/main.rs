use silkenweb::{
    elements::{div, hr, Div},
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
            .child(count.read().map(move |&count| define_counter_list(count))),
    );
}

fn define_counter_list(count: i64) -> Div {
    let mut counters = div();

    for _i in 0..count {
        counters = counters.child(define_counter(&Signal::new(0)));
    }

    counters.build()
}
