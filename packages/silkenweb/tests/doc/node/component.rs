use silkenweb::{
    css,
    elements::html::{dd, div, dl, dt, span},
    mount,
    node::{
        element::{ParentElement, TextParentElement},
        Component,
    },
};

pub fn component() {
    let name = span().text("HTML");
    let description = span().text("HyperText Markup Language");

    css!(content = "span {border: 3px solid red}");

    let mut term = Component::styled(stylesheet::text());
    let name_slot = term.slot(name);
    let description_slot = term.slot(description);

    let app = term.child(
        div().child(span().text("Term Definition")).child(
            dl().child(dt().child(name_slot))
                .child(dd().child(description_slot)),
        ),
    );
    mount("app", app);
}
