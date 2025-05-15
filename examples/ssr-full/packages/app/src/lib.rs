use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{
    clone,
    document::DocumentHead,
    dom::Dom,
    elements::{
        html::{button, details, div, p, span, summary, Div},
        ElementEvents,
    },
    hydration::{hydrate, hydrate_in_head},
    node::element::{ParentElement, TextParentElement},
    prelude::{html::title, HtmlElement},
    router,
    task::spawn_local,
    value::Sig,
};

pub fn hydrate_app() {
    let (head, body) = app();

    spawn_local(async {
        let stats = hydrate_in_head("head", head).await;
        web_log::println!("Hydrate head: {}", stats);
        let stats = hydrate("body", body).await;
        web_log::println!("Hydrate body: {}", stats);
    });
}

pub fn app<D: Dom>() -> (DocumentHead<D>, Div<D>) {
    let title_text = Mutable::new("Silkenweb SSR Example");
    let details_open = Mutable::new(false);
    let details_open_sig = details_open
        .signal()
        .map(|is_open| format!("Details open: {is_open}"));

    let head = DocumentHead::new().child(title().id("title").text(Sig(title_text.signal())));
    let body = div()
        .id("body")
        .child(
            button()
                .on_click(move |_, _| title_text.set("My Title"))
                .text("Set Title"),
        )
        .child(
            button()
                .on_click(|_, _| router::set_url_path("page_1.html"))
                .text("Go to page 1"),
        )
        .child(
            button()
                .on_click(|_, _| router::set_url_path("page_2.html"))
                .text("Go to page 2"),
        )
        .child(p().text(Sig(router::url_path().signal_ref(|url_path| {
            format!(
                "URL Path is: {}",
                match url_path.as_str() {
                    "" => "index.html",
                    path => path,
                }
            )
        }))))
        .child(
            details()
                .child(summary().text("Mutation Observer: expand me for more details..."))
                .child(p().text("Check the text below to see the mutation observer in action!"))
                .observe_mutations({
                    |observe| {
                        clone!(details_open);
                        observe.open(move |elem, _prev| {
                            details_open.set(elem.open());
                        })
                    }
                }),
        )
        .child(span().text(Sig(details_open_sig)));

    (head, body)
}
