// HTTP Requests using `reqwasm`
//
// `reqwest` could also be used and will work server side if required, but code
// size will be much larger.
use futures_signals::signal::Mutable;
use reqwasm::http::Request;
use silkenweb::{
    clone,
    elements::html::{button, div, p},
    log_panics, mount,
    prelude::{ElementEvents, ParentElement},
    task,
    value::Sig,
};

async fn get_ip() -> Result<String, reqwasm::Error> {
    Request::get("https://httpbin.org/ip")
        .send()
        .await?
        .text()
        .await
}

fn main() {
    log_panics();
    let text = Mutable::new("Press the button to get your IP address".to_owned());
    let text_signal = text.signal_cloned();

    mount(
        "app",
        div()
            .child(button().text("Get IP Address").on_click(move |_, _| {
                clone!(text);
                text.set("Loading...".to_owned());

                task::spawn_local(async move {
                    text.set(get_ip().await.unwrap_or_else(|err| format!("Error: {err}")));
                });
            }))
            .child(p().text(Sig(text_signal))),
    );
}
