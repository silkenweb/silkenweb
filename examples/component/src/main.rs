use silkenweb::{
    css,
    elements::{
        html::{dd, div, dl, dt, span},
        HtmlElement,
    },
    log_panics, mount,
    node::{
        element::{ParentElement, TextParentElement},
        ChildNode, Component,
    },
    ChildElement,
};

css!(content = "span {border: 3px solid red}");

#[derive(ChildElement)]
struct Term(Component);

impl Term {
    pub fn new(
        name: impl HtmlElement + ChildNode,
        description: impl HtmlElement + ChildNode,
    ) -> Self {
        let mut term = Component::styled(stylesheet::text());
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

fn main() {
    log_panics();

    let term = Term::new(
        span().text("HTML"),
        span().text("HyperText Markup Language"),
    );
    mount("app", term);
}
