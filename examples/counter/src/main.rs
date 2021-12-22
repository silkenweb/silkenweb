//! A minimal interactive example
use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{
    elements::{button, div, p},
    mount, signal, ParentBuilder,
};

fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal().map(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(signal(count_text)));

    mount("app", app);
}
