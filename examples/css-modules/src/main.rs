use silkenweb::{css, elements::html::*, prelude::*};

css!(path = "stylesheet.css", auto_mount, transpile = (modules));

fn main() {
    let app = div().class(class::my_class()).text("Hello, world!");
    mount("app", app);
}
