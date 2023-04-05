use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{elements::html::*, log_panics, prelude::*, value::Sig};
use silkenweb_example_web_components_wrapper::{ui5_button, ButtonDesign};

fn main() {
    log_panics();

    let count = Mutable::new(0);
    let count_text = count.signal().map(|i| format!("{i}"));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div()
        .child(
            ui5_button()
                .design(ButtonDesign::Emphasized)
                .text("+")
                .on_click(inc),
        )
        .child(p().text(Sig(count_text)));

    mount("app", app);
}
