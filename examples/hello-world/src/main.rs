use silkenweb::{elements::html::p, mount, prelude::ParentBuilder};

fn main() {
    mount("app", p().text("Hello, world!"));
}
