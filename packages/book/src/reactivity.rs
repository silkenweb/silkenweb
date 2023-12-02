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
    // # Reactivity
    //
    // ## A Simple App
    //
    // Lets create a simple example app to see how Silkenweb handles changes to
    // underlying data. We'll use a very simple counter for our example. First off,
    // we need something to store the count:
    let count = Mutable::new(0);
    // Next we need a signal so Silkenweb can react to changes to the count:
    let count_signal = count.signal();
    // This signal is an integer type, and we want a `String` signal that we can use
    // as the text of a DOM element. We use `map` to get a new `String` signal:
    let count_text = count_signal.map(|n| format!("{n}"));
    // Now we can specify what we want the DOM tree to look like:
    let app = p().text(Sig(count_text));

    // ## Investigating Reactivity
    //
    // Lets see how our app reacts to changes in `count`. We're going to use Server
    // Side Rendering (SSR). This gives us more explicit control over the [microtask
    // queue], so we can see whats going on. Converting our app to a `Node<Dry>`
    // means we can render it to a `String` on the server side (this is what the
    // `Dry` DOM type is):
    let node: Node<Dry> = app.into();

    // Now we'll add a convenience function to see what our app looks like at
    // various points in time:
    fn check(app: &Node<Dry>, expected: &str) {
        assert_eq!(app.to_string(), expected);
    }

    // Firstly, our app doesn't contain any text.
    check(&node, "<p></p>");
    // This is a because we haven't processed the [microtask queue] yet, so lets do
    // that now:
    render_now_sync();
    // And we see that `count` has been rendered.
    check(&node, "<p>0</p>");
    // If we change `count`, it doesn't immediately have an effect:
    count.set(1);
    check(&node, "<p>0</p>");
    // We need to process the [microtask queue] again, then our app will update:
    render_now_sync();
    check(&node, "<p>1</p>");

    // ## Event Handlers
    //
    // Normally you'd update your `Mutable` data with an event handler. Here's an
    // example counter app with a button that updates the count when clicked:
    let count = Mutable::new(0);
    let count_text = count.signal().map(|n| format!("{n}"));
    let increment = button().text("Increment").on_click(move |_, _| {
        count.replace_with(|n| *n + 1);
    });
    let app = div().child(p().text(Sig(count_text))).child(increment);
    mount(app);

    // [microtask queue]: https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide
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
