use silkenweb::{
    css,
    elements::html::{div, p, style},
    mount,
    prelude::{HtmlElement, ParentBuilder},
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
            .child(p().class(["text-color"]).text("Hello, world!")),
    );
}
