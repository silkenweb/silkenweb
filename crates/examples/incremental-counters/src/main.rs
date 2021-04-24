use std::{cell::RefCell, iter, rc::Rc};

use silkenweb::{
    elements::{button, div, Div},
    list_state::ElementList,
    mount,
    state::Signal,
    Builder,
};

fn counter() -> Div {
    let count = Signal::new(0);
    let inc = count.write();
    let dec = count.write();

    div()
        .child(
            button()
                .on_click(move |_, _| dec.replace(|i| i - 1))
                .text("-"),
        )
        .text(count.read().map(|i| format!("{}", i)))
        .child(
            button()
                .on_click(move |_, _| inc.replace(|i| i + 1))
                .text("+"),
        )
        .build()
}

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(ElementList::new(div(), move |()| counter(), iter::empty()));
    let push_elem = list.write();
    let pop_elem = list.write();
    let id = Rc::new(RefCell::new(0));

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move |_, _| pop_elem.mutate(ElementList::pop))
                    .text("-"),
            )
            .child(
                button()
                    .on_click(move |_, _| {
                        push_elem.mutate({
                            let id = id.clone();
                            move |l| {
                                let current_id = id.replace_with(|current| *current + 1);
                                l.insert(current_id, ());
                            }
                        })
                    })
                    .text("+"),
            )
            .child(list.read()),
    );
}
