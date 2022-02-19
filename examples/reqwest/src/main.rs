// HTTP Requests using `reqwest`
use futures_signals::signal::Mutable;
use silkenweb::{
    clone,
    elements::html::{button, div, p},
    mount,
    prelude::{ElementEvents, ParentBuilder},
    task,
};

async fn get_ip() -> reqwest::Result<String> {
    reqwest::get("https://httpbin.org/ip").await?.text().await
}

fn main() {
    let text = Mutable::new("Press the button to get your IP address".to_owned());
    let text_signal = text.signal_cloned();

    mount(
        "app",
        div()
            .child(button().text("Get IP Address").on_click(move |_, _| {
                clone!(text);
                text.set("Loading...".to_owned());

                task::spawn_local(async move {
                    text.set(
                        get_ip()
                            .await
                            .unwrap_or_else(|err| format!("Error: {}", err)),
                    );
                });
            }))
            .child(p().text_signal(text_signal)),
    );
}
