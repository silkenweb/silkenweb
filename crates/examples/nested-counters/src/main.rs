use surfinia::{button, div, mount, Builder, Div, DivBuilder, Memo, State};

thread_local!(
    static COUNTER_MEMO: Memo<u32, Div> = Memo::initialize();
);

fn counter_memo(state: &State<u32>) -> Div {
    COUNTER_MEMO.with(|counter_memo| counter_memo.memo(|state| counter(state).build(), state))
}

fn counter(count_state: &State<u32>) -> DivBuilder {
    let inc = count_state.setter();
    let dec = count_state.setter();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count_state.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    let count_state = State::new(|| 10);

    mount(
        "app",
        counter(&count_state).child(count_state.with(|&i| {
            let mut counters = div();

            for _j in 0..i {
                counters = counters.child(counter_memo(&State::new(|| 0)));
            }

            counters
        })),
    );
}
