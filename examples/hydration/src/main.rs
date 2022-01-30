//! A minimal interactive example
use futures_signals::signal::Mutable;
use silkenweb::{
    dom::{hydrate_tracked, HydrationStats},
    elements::{html::*, macros::ElementBuilder},
    prelude::*,
};

fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div().child(button().on_click(inc).text("+")).child(
        p().attribute("data-silkenweb-test", true)
            .text_signal(count_text),
    );

    hydrate_tracked("app", app, HydrationStats::default());
}
