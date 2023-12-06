use silkenweb::{elements::html::p, mount, node::element::ParentElement};

fn main() {
    mount("app", p().text("Hello, world!"));
}
