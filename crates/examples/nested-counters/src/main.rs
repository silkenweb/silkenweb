use surfinia::{append_to_body, button, div, DivBuilder, State};

fn counter(count_state: State<u32>) -> DivBuilder {
    let inc = count_state.clone();
    let dec = count_state.clone();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count_state.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    let count_state = State::new(0);

    append_to_body(counter(count_state.clone()).child(count_state.with(|&i| {
        let mut counters = div();

        for _j in 0..i {
            counters = counters.child(counter(State::new(0)));
        }

        counters
    })));
}
