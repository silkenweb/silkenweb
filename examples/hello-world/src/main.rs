//! A minimal example
use silkenweb::{dom::mount, elements::html::p, prelude::ParentBuilder};

fn main() {
    mount("app", p().text("Hello, world!"));
}
