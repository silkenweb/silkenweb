//! A minimal example
use silkenweb::{elements::p, mount, ParentBuilder};

fn main() {
    mount("app", p().text("Hello, world!"));
}
