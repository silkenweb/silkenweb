use std::{cell::RefCell, rc::Rc};

use surfinia_core::{
    hooks::{memo::Memo, state::Signal},
    mount,
    Builder,
};
use surfinia_html::{button, div, DivBuilder};

fn counter(count: &Signal<u32>) -> DivBuilder {
    let inc = count.writer();
    let dec = count.writer();

    div()
        .child(button().on_click(move || dec.replace(|i| i - 1)).text("-"))
        .text(count.map(|i| format!("{}", i)))
        .child(button().on_click(move || inc.replace(|i| i + 1)).text("+"))
}

fn main() {
    console_error_panic_hook::set_once();
    let child_counts = Memo::default();
    let call_count = Rc::new(RefCell::new(0));
    let count = Signal::new(0);

    mount(
        "app",
        counter(&count).child(count.map(move |&count| {
            *call_count.borrow_mut() += 1;
            web_log::println!("Call count = {}", call_count.borrow());

            let mut counters = div();
            child_counts.gc();

            for i in 0..count {
                let child = child_counts.cache(i, || counter(&Signal::new(0)).build());

                counters = counters.child(child);
            }

            counters
        })),
    );
}
