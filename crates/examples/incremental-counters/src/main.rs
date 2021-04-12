use std::iter;

use surfinia::{button, div, mount, use_list_state, use_state, DivBuilder, ElementBuilder};

fn counter() -> DivBuilder {
    let (count, set_count) = use_state(0);
    let inc = set_count.clone();
    let dec = set_count;

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    console_error_panic_hook::set_once();
    let (count, set_count) = use_list_state(ElementBuilder::new("div"), iter::repeat(()).take(0));

    mount(
        "app",
        div()
            .child(button().on_click(move || set_count.append(())).text("+"))
            .child(count.with(move |()| counter())),
    );
}
