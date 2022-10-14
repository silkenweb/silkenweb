use silkenweb::{
    elements::html::span, mount, node::element::ElementBuilder, prelude::ParentBuilder,
};
use silkenweb_bootstrap::css;

fn main() {
    let app = span().class([css::BADGE, css::TEXT_BG_PRIMARY]).text("Hello, world!");

    mount("app", app);
}
