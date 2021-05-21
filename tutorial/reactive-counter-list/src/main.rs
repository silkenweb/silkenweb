use silkenweb::{
    containers::SignalVec,
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
    let list = SignalVec::new();
    // ANCHOR_END: new_list

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(
                div()
                    .child(pop_button(&list))
                    // ANCHOR: list_len
                    .text(list.read().map(|list| format!("{}", list.len())))
                    // ANCHOR_END: list_len
                    .child(push_button(&list)),
            )
            .child(hr())
            .child(div().children(&list.read())),
    );
}
// ANCHOR_END: main

// ANCHOR: push_button
fn push_button(list: &Signal<SignalVec<Div>>) -> Button {
    let push_elem = list.write();
    button()
        .on_click(move |_, _| {
            // ANCHOR: mutate_list
            push_elem.mutate(move |list| list.push(define_counter(&Signal::new(0)).build()))
            // ANCHOR_END: mutate_list
        })
        .text("+")
        .build()
}
// ANCHOR_END: push_button

// ANCHOR: pop_button
fn pop_button(list: &Signal<SignalVec<Div>>) -> Button {
    let pop_elem = list.write();
    button()
        .on_click(move |_, _| {
            pop_elem.mutate(move |list| {
                if !list.is_empty() {
                    list.pop();
                }
            })
        })
        .text("-")
        .build()
}
// ANCHOR_END: pop_button
