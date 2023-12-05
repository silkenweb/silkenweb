use futures_signals::signal::Mutable;
use silkenweb::{
    document::DocumentHead,
    elements::html::*,
    hydration::{hydrate, hydrate_in_head},
    log_panics,
    node::element::Element,
    prelude::*,
    task::spawn_local,
    value::Sig,
};

// For a more complete example, see `examples/ssr-full`
fn main() {
    log_panics();

    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{i}"));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    let head = DocumentHead::new().children([
        meta().name("my-meta").content("abc"),
        meta().name("my-other-meta").content("xyz"),
    ]);
    let app = div()
        .id("app")
        .child(button().on_click(inc).text("+"))
        .child(
            p().attribute("data-silkenweb-test", true)
                .text(Sig(count_text)),
        );

    spawn_local(async {
        let stats = hydrate_in_head("my-head-id", head).await;
        web_log::println!("{}", stats);
        let stats = hydrate("app", app).await;
        web_log::println!("{}", stats);
    });
}
