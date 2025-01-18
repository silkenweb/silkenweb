use silkenweb::{elements::html::p, mount, node::element::ParentElement, prelude::Element};
use twust::tw;

fn main() {
    mount(
        "app",
        p().classes([tw!("text-red-600"), tw!("text-2xl")])
            .text("Hello, world!"),
    );
}
