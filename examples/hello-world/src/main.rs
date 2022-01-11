//! A minimal example
use silkenweb::{html::p, mount, ParentBuilder};

fn main() {
    mount("app", p().text("Hello, world!"));
}
