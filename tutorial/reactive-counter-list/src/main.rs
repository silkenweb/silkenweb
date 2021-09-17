use silkenweb::{
    containers::ChangingVec,
    elements::{button, div, hr, Button, Div},
    mount,
    signal::Signal,
    Builder,
};
use silkenweb_tutorial_common::define_counter;

// ANCHOR: main
fn main() {
    // TODO: Update tutorial text
    // ANCHOR: new_list
    // TODO Rename all list stuff to `counters`
    let list = Signal::new(ChangingVec::new());
    // ANCHOR_END: new_list

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(
                div()
                    .child(pop_button(&list))
                    // ANCHOR: list_len
                    .text(list.read().map(|list| format!("{}", list.data().len())))
                    // ANCHOR_END: list_len
                    .child(push_button(&list)),
            )
            .child(hr())
            .child(div().children(&list.read())),
    );
}
// ANCHOR_END: main

// ANCHOR: push_button
fn push_button(list: &Signal<ChangingVec<Div>>) -> Button {
    let list_write = list.write();
    button()
        .on_click(move |_, _| {
            // ANCHOR: mutate_list
            list_write.mutate(|v| v.push(define_counter(&Signal::new(0)).build()));
            // ANCHOR_END: mutate_list
        })
        .text("+")
        .build()
}
// ANCHOR_END: push_button

// ANCHOR: pop_button
fn pop_button(list: &Signal<ChangingVec<Div>>) -> Button {
    let list_read = list.read();
    let list_write = list.write();

    // TODO: Docs on why we use `current` rather than a signal.
    button()
        .on_click(move |_, _| {
            if !list_read.current().data().is_empty() {
                list_write.mutate(ChangingVec::pop);
            }
        })
        .text("-")
        .build()
}
// ANCHOR_END: pop_button
