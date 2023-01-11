use silkenweb::{
    elements::html::{dd, dl, dt, span},
    mount,
    node::{
        component::Component,
        element::{GenericElement, ParentElement},
    },
    prelude::{HtmlElement, Node},
    value::Value,
};

// TODO: Wrap in a struct with constructor

struct Term(Component);

impl Term {
    pub fn new(
        // TODO: `trait ChildNode: Into<Node> + Value + 'static`
        name: impl HtmlElement + Into<Node> + Value + 'static,
        description: impl HtmlElement + Into<Node> + Value + 'static,
    ) -> Self {
        let mut term = Component::new();
        let name = term.slot(name);
        let description = term.slot(description);
        Self(term.child(dl().child(dt().child(name)).child(dd().child(description))))
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
