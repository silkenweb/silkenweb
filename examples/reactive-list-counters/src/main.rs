use silkenweb::{
    element_list::OrderedElementList,
    elements::{button, div, Button, DivBuilder},
    mount,
    signal::{Signal, WriteSignal},
    Builder,
};

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(OrderedElementList::new(div()));

    mount(
        "app",
        div()
            .child(pop_button(&list))
            .text(list.read().map(|list| format!("{}", list.len())))
            .child(push_button(&list))
            .child(list.read()),
    );
}

fn push_button(list: &Signal<OrderedElementList<usize>>) -> Button {
    let push_elem = list.write();
    button()
        .on_click(move |_, _| {
            push_elem.mutate(move |list| list.insert(list.len(), counter().into()))
        })
        .text("+")
        .build()
}

fn pop_button(list: &Signal<OrderedElementList<usize>>) -> Button {
    let pop_elem = list.write();
    button()
        .on_click(move |_, _| {
            pop_elem.mutate(move |list| {
                if !list.is_empty() {
                    list.remove(&(list.len() - 1))
                }
            })
        })
        .text("-")
        .build()
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
