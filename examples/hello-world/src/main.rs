use silkenweb::{elements::html::p, mount, prelude::ParentElement};

fn main() {
    mount("app", p().text("Hello, world!"));
}
