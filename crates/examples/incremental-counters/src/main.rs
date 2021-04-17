use std::iter;

use surfinia_core::{
    hooks::{list_state::ElementList, state::Signal},
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, Div};

fn counter() -> Div {
    let count = Signal::new(0);
    let inc = count.write();
    let dec = count.write();

    div()
        .child(button().on_click(move |_| dec.replace(|i| i - 1)).text("-"))
        .text(count.read().map(|i| format!("{}", i)))
        .child(button().on_click(move |_| inc.replace(|i| i + 1)).text("+"))
        .build()
}

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(element_list(div(), move |()| counter(), iter::empty()));
    let push_elem = list.write();
    let pop_elem = list.write();

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move |_| pop_elem.mutate(ElementList::pop))
                    .text("-"),
            )
            .child(
                button()
                    .on_click(move |_| push_elem.mutate(|l| l.push(&())))
                    .text("+"),
            )
            .child(list.read()),
    );
}
