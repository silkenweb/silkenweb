use lib::{ElementBuilder, state, tag, StateSetter};

fn counter(i: u64, set_i: StateSetter<u64>) -> ElementBuilder {
    let inc_i = set_i.clone();
    let dec_i = set_i;

    tag("div")
        .child(
            tag("button")
                .on("click", move |_| inc_i.set(i + 1))
                .text("+"),
        )
        .child(
            tag("button")
                .on("click", move |_| dec_i.set(i - 1))
                .text("-"),
        )
        .text(format!("Count = {}", i))
}

fn main() {
    console_error_panic_hook::set_once();
    web_log::println!("Running");

    state(0, |i, set_i| counter(i, set_i)).append_to_body();
}
