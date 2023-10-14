use futures_signals::signal::Mutable;
use silkenweb::{
    dom::Dom,
    elements::{
        html::{button, div, p, Div},
        ElementEvents,
    },
    hydration::hydrate,
    prelude::{
        html::{title, Title},
        HtmlElement, ParentElement,
    },
    router,
    task::spawn_local,
    value::Sig,
};

pub fn hydrate_app() {
    let (title, body) = app();

    spawn_local(async {
        hydrate("title", title).await;
        let stats = hydrate("body", body).await;
        web_log::println!("{}", stats);
    });
}

pub fn app<D: Dom>() -> (Title<D>, Div<D>) {
    let title_text = Mutable::new("Silkenweb SSR Example");

    let title = title().id("title").text(Sig(title_text.signal()));
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
        }))));

    (title, body)
}
