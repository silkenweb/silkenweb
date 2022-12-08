use silkenweb::{
    elements::html::{dd, div, dl, dt, slot, span},
    mount,
    node::element::{ParentElement, ShadowRootParent},
    prelude::HtmlElement,
};

fn main() {
    const TERM_SLOT: &str = "term";
    const DESCRIPTION_SLOT: &str = "description";

    let term_template = dl()
        .child(dt().child(slot().name(TERM_SLOT)))
        .child(dd().child(slot().name(DESCRIPTION_SLOT)));

    let html_term = div().children([
        span().slot(TERM_SLOT).text("HTML"),
        span()
            .slot(DESCRIPTION_SLOT)
            .text("HyperText Markup Language"),
    ]);

    mount("app", html_term.attach_shadow_children([term_template]));
}
