use std::{cell::RefCell, rc::Rc};

use silkenweb::{
    elements::{button, div, DivBuilder},
    memo::MemoCache,
    mount,
    signal::Signal,
    Builder,
};

fn main() {
    console_error_panic_hook::set_once();
    let child_counts = MemoCache::default();
    let call_count = Rc::new(RefCell::new(0));
    let count = Signal::new(0);

    mount(
        "app",
        counter(&count).child(count.read().map(move |&count| {
            let child_counts = child_counts.frame();
            *call_count.borrow_mut() += 1;
            web_log::println!("Call count = {}", call_count.borrow());

            let mut counters = div();

            for i in 0..count {
                let child = child_counts.cache(i, || counter(&Signal::new(0)).build());

                counters = counters.child(child);
            }

            counters
        })),
    );
}

fn counter(count: &Signal<u32>) -> DivBuilder {
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
}
