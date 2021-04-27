use std::{cell::Cell, iter, rc::Rc};

use silkenweb::{
    clone,
    element_list::ElementList,
    elements::{button, div, Button, DivBuilder},
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(ElementList::new(div(), move |()| counter(), iter::empty()));
    let push_elem = list.write();
    let pop_elem = list.write();
    let id = Rc::new(Cell::new(0));

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move |_, _| pop_elem.mutate(ElementList::pop))
                    .text("-"),
            )
            .text(list.read().map(|list| format!("{}", list.len())))
            .child(
                button()
                    .on_click(move |_, _| {
                        push_elem.mutate({
                            clone!(id);
                            move |l| {
                                let current_id = id.replace(id.get() + 1);
                                l.insert(current_id, ());
                            }
                        })
                    })
                    .text("+"),
            )
            .child(list.read()),
    );
}

fn counter() -> DivBuilder {
    let count = Signal::new(0);

    div()
        .child(update_count("-", -1, count.write()))
        .text(count.read().map(|i| format!("{}", i)))
        .child(update_count("+", 1, count.write()))
}

fn update_count(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
