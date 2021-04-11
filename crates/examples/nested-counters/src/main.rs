use std::{cell::RefCell, collections::HashMap, rc::Rc};

use surfinia::{
    button,
    div,
    mount,
    use_state,
    Builder,
    Div,
    DivBuilder,
    GetState,
    Reference,
    SetState,
};

fn counter(count: &GetState<u32>, set_count: &SetState<u32>) -> DivBuilder {
    let inc = set_count.clone();
    let dec = set_count.clone();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    let mut child_counts = Reference::new(Rc::new(RefCell::new(HashMap::<u32, Div>::new())));

    mount(
        "app",
        child_counts.with(|child_counts| {
            let (count, set_count) = use_state(0);

            counter(&count, &set_count).child(count.with({
                let child_counts = child_counts.clone();
                move |&count| {
                    let mut counters = div();

                    for i in 0..count {
                        let (count, set_count) = use_state(0);

                        let child = child_counts
                            .as_ref()
                            .borrow_mut()
                            .entry(i)
                            .or_insert_with(|| counter(&count, &set_count).build())
                            .clone();

                        counters = counters.child(child);
                    }

                    counters
                }
            }))
        }),
    );
}
