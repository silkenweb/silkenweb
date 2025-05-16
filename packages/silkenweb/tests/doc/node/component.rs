use silkenweb::{
    css,
    dom::Dry,
    elements::html::{dd, div, dl, dt, span},
    node::{
        element::{ParentElement, TextParentElement},
        Component,
    },
};

pub fn component() {
    let name = span().text("HTML");
    let description = span().text("HyperText Markup Language");

    css!(content = "span {border: 3px solid red}");

    let mut term = Component::<Dry>::styled(stylesheet::text());
    let name_slot = term.slot(name);
    let description_slot = term.slot(description);

    term.child(
        div().child(span().text("Term Definition")).child(
            dl().child(dt().child(name_slot))
                .child(dd().child(description_slot)),
        ),
    );
}
