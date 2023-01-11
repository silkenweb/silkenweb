use silkenweb::{
    css,
    elements::html::{dd, dl, dt, span},
    mount,
    node::{
        element::{GenericElement, ParentElement},
        ChildNode, Component,
    },
    prelude::{html::div, HtmlElement},
};

css!(inline: "span {border: 3px solid red}");

struct Term(Component);

impl Term {
    pub fn new(
        name: impl HtmlElement + ChildNode,
        description: impl HtmlElement + ChildNode,
    ) -> Self {
        let mut term = Component::styled(stylesheet());
        let name = term.slot(name);
        let description = term.slot(description);
        Self(
            term.child(
                div()
                    .child(span().text("Term Definition"))
                    .child(dl().child(dt().child(name)).child(dd().child(description))),
            ),
        )
    }
}

impl From<Term> for GenericElement {
    fn from(value: Term) -> Self {
        value.0.into()
    }
}

fn main() {
    let term = Term::new(
        span().text("HTML"),
        span().text("HyperText Markup Language"),
    );
    mount("app", term);
}
