use surfinia::{button, div, mount, use_state, DivBuilder, GetState, SetState};

fn counter(count: &GetState<u32>, set_count: &SetState<u32>) -> DivBuilder {
    let inc = set_count.clone();
    let dec = set_count.clone();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    let (count, set_count) = use_state(0);

    mount(
        "app",
        counter(&count, &set_count).child(count.with(|&i| {
            let mut counters = div();

            for _j in 0..i {
                let (count, set_count) = use_state(0);

                counters = counters.child(counter(&count, &set_count));
            }

            counters
        })),
    );
}
