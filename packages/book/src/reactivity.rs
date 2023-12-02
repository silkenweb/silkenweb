use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{
    dom::Dry,
    elements::{
        html::{button, div, p},
        ElementEvents,
    },
    node::{element::ParentElement, Node},
    task::server::render_now_sync,
    value::Sig,
};

pub fn body() {
    let count = Mutable::new(0);
    let count_text = count.signal().map(|n| format!("{n}"));
    let app = p().text(Sig(count_text));
    // TODO: Say we going to use SSR as it gives us more explicit control over the
    // microtask queue, so we can see whats going on.

    // TODO: Introduce concepts:
    // - DOM types
    // - Dry DOM
    // - Running tasks on the microtask queue
    let node: Node<Dry> = app.into();

    fn check(app: &Node<Dry>, expected: &str) {
        assert_eq!(app.to_string(), expected);
    }

    check(&node, "<p></p>");
    render_now_sync();
    check(&node, "<p>0</p>");
    render_now_sync();
    count.set(1);
    check(&node, "<p>0</p>");
    render_now_sync();
    check(&node, "<p>1</p>");

    let count = Mutable::new(0);
    let count_text = count.signal().map(|n| format!("{n}"));
    let increment = button().text("Increment").on_click(move |_, _| {
        count.replace_with(|n| *n + 1);
    });
    let app = div().child(p().text(Sig(count_text))).child(increment);
    mount(app);
}

/// Provide a fake `mount` function so we can test `body`.
fn mount(_node: impl Into<Node>) {}

#[cfg(test)]
mod tests {
    use silkenweb::task::sync_scope;

    use super::body;

    #[test]
    fn test() {
        sync_scope(body)
    }
}
