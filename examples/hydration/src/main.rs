use futures_signals::signal::Mutable;
use silkenweb::{
    elements::html::*, hydration::hydrate, node::element::ElementBuilder, prelude::*,
    task::spawn_local,
};

// For a more complete example, see <https://github.com/silkenweb/ssr-example>
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

    spawn_local(async {
        let stats = hydrate("app", app).await;
        web_log::println!("{}", stats);
    });
}
