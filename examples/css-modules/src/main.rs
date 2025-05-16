use silkenweb::{
    elements::html::div,
    log_panics, mount,
    node::element::{Element, ParentElement, TextParentElement},
};

mod red {
    silkenweb::css!(path = "red.css", auto_mount, transpile = (modules));

    pub use class::*;
}

mod green {
    silkenweb::css!(path = "green.css", auto_mount, transpile = (modules));

    pub use class::*;
}

fn main() {
    log_panics();

    // "red.css" and "green.css" use the same class name for text color, but this is
    // scoped using `modules`.
    let app = div().children([
        div().class(red::color()).text("Red text"),
        div().class(green::color()).text("Green text"),
    ]);
    mount("app", app);
}
