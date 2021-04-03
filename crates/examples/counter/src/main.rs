use surfinia::{append_to_body, button, div, State};

fn main() {
    let count_state = State::new(0);
    let inc = count_state.setter();
    let dec = count_state.setter();

    append_to_body(
        div()
            .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
            .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
            .child(count_state.with(|i| div().text(format!("Count = {}", i)))),
    );
}
