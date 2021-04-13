use std::iter;

use surfinia::{button, div, mount, use_list_state, use_state, Div, ElementBuilder};

fn counter() -> Div {
    let (count, set_count) = use_state(0);

    count.with(move |i| {
        let inc = set_count.clone();
        let dec = set_count.clone();

        div()
            .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
            .text(format!("{}", i))
            .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
    })
}

fn main() {
    console_error_panic_hook::set_once();
    let (list, list_mut) = use_list_state(ElementBuilder::new("div"), iter::repeat(()).take(10));
    let push_elem = list_mut.clone();
    let pop_elem = list_mut;

    mount(
        "app",
        div()
            .child(button().on_click(move || pop_elem.pop()).text("-"))
            .child(button().on_click(move || push_elem.push(())).text("+"))
            .child(list.with(move |()| counter())),
    );
}
