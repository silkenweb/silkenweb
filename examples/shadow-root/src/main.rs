use silkenweb::{
    elements::html::p,
    mount,
    node::element::{ParentBuilder, ShadowRootBuilder},
};

fn main() {
    mount(
        "app",
        p().attach_shadow_children([p().text("Hello, world!")]),
    );
}
