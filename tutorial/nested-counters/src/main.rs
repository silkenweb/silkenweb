use silkenweb::{
    elements::{div, Div},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::render_counter;

fn main() {
    let count = Signal::new(0);

    mount(
        "app",
        render_counter(&count).child(count.read().map(move |&count| render_counter_list(count))),
    );
}

fn render_counter_list(count: i64) -> Div {
    let mut counters = div();

    for _i in 0..count {
        let child = render_counter(&Signal::new(0)).build();

        counters = counters.child(child);
    }

    counters.build()
}
