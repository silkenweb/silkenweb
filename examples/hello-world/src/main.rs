use silkenweb::{elements::html::p, mount, node::element::TextParentElement};

fn main() {
    mount("app", p().text("Hello, world!"));
}
