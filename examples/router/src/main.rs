//! A minimal routing example
use futures_signals::signal::SignalExt;
use silkenweb::{
    elements::{button, div, p},
    mount, router, signal,
};

fn main() {
    mount("app", {
        div()
            .child(
                button()
                    .on_click(|_, _| router::set_url_path("/route_1"))
                    .text("Go to route 1"),
            )
            .child(
                button()
                    .on_click(|_, _| router::set_url_path("/route_2"))
                    .text("Go to route 2"),
            )
            .child(p().text(signal(
                router::url().map(|url| format!("URL Path is: {}", url.pathname())),
            )))
    });
}
