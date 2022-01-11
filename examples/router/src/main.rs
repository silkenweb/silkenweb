//! A minimal routing example
use silkenweb::{
    dom::mount,
    elements::{
        html::{button, div, p},
        ElementEvents, ParentBuilder,
    },
    router,
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
            .child(p().text_signal(
                router::url().signal_ref(|url| format!("URL Path is: {}", url.pathname())),
            ))
    });
}
