use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{elements::html::*, prelude::*, value::Sig};

fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal().map(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)));

    mount("app", app);
}
