use silkenweb::{
    containers::ChangeTrackingVec,
    elements::{button, div, hr, Button, Div, DivBuilder},
    mount,
    signal::{Signal, ReadSignal, WriteSignal},
    Builder,
};

fn main() {
    let list = Signal::new(ChangeTrackingVec::new());

    mount(
        "app",
        div()
            .text("How many counters would you like?")
            .child(
                div()
                    .child(pop_button(&list))
                    .text(list.read().map(|list| format!("{}", list.data().len())))
                    .child(push_button(&list)),
            )
            .child(hr())
            .child(div().children(&list.read())),
    );
}

fn push_button(list: &Signal<ChangeTrackingVec<Div>>) -> Button {
    let list_write = list.write();
    button()
        .on_click(move |_, _| {
            list_write.mutate(|v| v.push(define_counter(&Signal::new(0)).build()));
        })
        .text("+")
        .build()
}

fn pop_button(list: &Signal<ChangeTrackingVec<Div>>) -> Button {
    let list_read = list.read();
    let list_write = list.write();

    button()
        .on_click(move |_, _| {
            if !list_read.current().data().is_empty() {
                list_write.mutate(ChangeTrackingVec::pop);
            }
        })
        .text("-")
        .build()
}

pub fn define_counter(count: &Signal<i64>) -> DivBuilder {
    let count_text: ReadSignal<String> = count.read().map(|i| format!("{}", i));

    div()
        .child(define_button("-", -1, count.write()))
        .text(count_text)
        .child(define_button("+", 1, count.write()))
}

pub fn define_button(label: &str, delta: i64, set_count: WriteSignal<i64>) -> Button {
    button()
        .on_click(move |_, _| set_count.replace(move |&i| i + delta))
        .text(label)
        .build()
}
