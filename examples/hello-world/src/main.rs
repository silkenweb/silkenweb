//! A minimal example
use silkenweb::{elements::p, mount};

fn main() {
    mount("app", p().text("Hello, world!"));
}
