use silkenweb::{
    elements::html::{div, p},
    log_panics, mount,
    node::element::{ParentElement, TextParentElement},
    prelude::Element,
};
use twust::{tw1, tws1};

fn main() {
    log_panics();
    mount(
        "app",
        div().children([
            p().class(tw1!("text-red-600"))
                .class(tw1!("text-2xl"))
                .text("Set classes individually"),
            p().classes(tws1!["text-red-600", "text-2xl"])
                .text("Set multiple classes at once"),
        ]),
    );
}
