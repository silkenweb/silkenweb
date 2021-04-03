use surfinia::{append_to_body, button, div, state, DivBuilder, StateSetter};

fn counter(i: u64, set_i: StateSetter<u64>) -> DivBuilder {
    let inc_i = set_i.clone();
    let dec_i = set_i;

    div()
        .child(button().on_click(move || inc_i.set(i + 1)).text("+"))
        .child(button().on_click(move || dec_i.set(i - 1)).text("-"))
        .text(format!("Count = {}", i))
}

fn main() {
    append_to_body(state(0, |i, set_i| {
        let mut c = counter(i, set_i);

        for _j in 0..i {
            // TODO: Can we find a way to pass `counter` directly rather than wrapping it in
            // a closure?
            c = c.child(state(0, |i, set_i| counter(i, set_i).build()));
        }

        c.build()
    }));
}
