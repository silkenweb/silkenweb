//! A minimal routing example
use silkenweb::{
    elements::{button, div, p},
    mount, router,
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
            .child(p().text(router::url().map(|url| format!("URL Path is: {}", url.path()))))
    });
}
