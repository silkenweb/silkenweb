use silkenweb::{
    css,
    elements::html::{div, p, style},
    mount,
    node::element::ElementBuilder,
    prelude::ParentBuilder,
};

fn main() {
    mount(
        "app",
        div()
            .child(style().text(css!(
                "
                .text-color {
                    color: limegreen;
                }
            "
            )))
            .child(p().classes(["text-color"]).text("Hello, world!")),
    );
}
