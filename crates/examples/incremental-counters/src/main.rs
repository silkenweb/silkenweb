use std::iter;

use surfinia_core::{
    hooks::{
        list_state::ElementList,
        state::{use_state, GetState},
    },
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, Div};

fn counter() -> GetState<Div> {
    let (count, set_count) = use_state(0);

    count.with(move |i| {
        let inc = set_count.clone();
        let dec = set_count.clone();

        div()
            .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
            .text(format!("{}", i))
            .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
            .build()
    })
}

fn main() {
    console_error_panic_hook::set_once();
    let (list, list_mut) = use_state(element_list(div(), move |()| counter(), iter::empty()));
    let push_elem = list_mut.clone();
    let pop_elem = list_mut;

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move || pop_elem.edit(ElementList::pop))
                    .text("-"),
            )
            .child(
                button()
                    .on_click(move || push_elem.edit(|l| l.push(&())))
                    .text("+"),
            )
            .child(list),
    );
}
