use lib::{state, tag, ElementBuilder, StateSetter};

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

    state(0, |i, set_i| {
        let mut c = counter(i, set_i);

        for _j in 0..i {
            c = c.child(state(0, counter));
        }

        c
    })
    .append_to_body();
}
