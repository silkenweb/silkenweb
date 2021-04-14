use surfinia_core::{
    hooks::{
        memo::Memo,
        reference::Reference,
        state::{use_state, GetState, SetState},
    },
    mount,
    Builder,
};
use surfinia_html::{button, div, DivBuilder};

fn counter(count: &GetState<u32>, set_count: &SetState<u32>) -> DivBuilder {
    let inc = set_count.clone();
    let dec = set_count.clone();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    console_error_panic_hook::set_once();
    let child_counts = Memo::default();
    let call_count = Reference::new(0);
    let (count, set_count) = use_state(0);

    mount(
        "app",
        counter(&count, &set_count).child(count.with(move |&count| {
            *call_count.borrow_mut() += 1;
            web_log::println!("Call count = {}", call_count.borrow());

            let mut counters = div();

            for i in 0..count {
                let (count, set_count) = use_state(0);
                let child = child_counts.cache(i, || counter(&count, &set_count).build());

                counters = counters.child(child);
            }

            counters
        })),
    );
}
