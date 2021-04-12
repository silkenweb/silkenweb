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
    let (list, list_mut) = use_list_state(ElementBuilder::new("div"), iter::repeat(()).take(0));
    let push_elem = list_mut.clone();
    let pop_elem = list_mut;

    mount(
        "app",
        div()
            .child(button().on_click(move || push_elem.push(())).text("+"))
            .child(button().on_click(move || pop_elem.pop()).text("-"))
            .child(list.with(move |()| counter())),
    );
}
