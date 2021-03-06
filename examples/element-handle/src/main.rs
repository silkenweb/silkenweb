use futures_signals::signal::Mutable;
use silkenweb::{elements::html::*, node::element::ElementBuilder, prelude::*};

fn main() {
    let text = Mutable::new("".to_string());
    let input = input();
    let input_handle = input.handle();

    let app = div()
        .child(input)
        .child(button().text("Read Input").on_click({
            clone!(text);
            move |_, _| text.set(input_handle.dom_element().value())
        }))
        .text_signal(text.signal_cloned());

    mount("app", app);
}
