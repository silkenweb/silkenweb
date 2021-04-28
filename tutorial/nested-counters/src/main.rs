use silkenweb::{
    elements::{div, Div},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::define_counter;

fn main() {
    let count = Signal::new(0);

    mount(
        "app",
        define_counter(&count).child(count.read().map(move |&count| define_counter_list(count))),
    );
}

fn define_counter_list(count: i64) -> Div {
    let mut counters = div();

    for _i in 0..count {
        let child = define_counter(&Signal::new(0)).build();

        counters = counters.child(child);
    }

    counters.build()
}
