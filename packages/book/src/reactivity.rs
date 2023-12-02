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
    // Side Rendering (SSR) as this gives us more explicit control over the
    // [microtask queue], so we can see whats going on. Converting our app to a
    // `Node<Dry>` means we can render it to a `String` on the server side (this
    // is what the `Dry` DOM type is):
    let node: Node<Dry> = app.into();

    // Now we'll add a convenience function to see what our app looks like at
    // various points in time:
    fn check(app: &Node<Dry>, expected: &str) {
        assert_eq!(app.to_string(), expected);
    }

    // The first thing you might notice is that our app doesn't contain any text
    // yet:
    check(&node, "<p></p>");
    // This is a because we haven't processed the [microtask queue] yet, so lets do
    // that:
    render_now_sync();
    check(&node, "<p>0</p>");
    // Now we can see that `count` has been rendered. Similarly, if we change
    // `count`, it doesn't immediately have an effect:
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

    // ## Children
    //
    // So far we've seen how we can add reactivity to the text in a DOM node.
    // This is great, but very limiting. It would be much more powerful if we
    // could change the structure of the DOM based on our data.
    //
    // To do this, we can set the children of any DOM node based on a signal.
    // There are 3 ways of doing this:
    //
    // - add a sigle child based on a signal
    // - add an optional child based on a signal
    // - add and remove multiple children based on a vector signal
    //
    // We can mix and match any of these on a given parent node.

    // ### Single Child Signal
    //
    // We'll write a very simple app with 2 tabs as an example. Each tab will have a
    // different element type, and we'll just switch between tabs by setting a
    // mutable, rather than wiring up buttons to change the tab.
    //
    // First we define an enum to represent which tab is selected, and a
    // [`Mutable`] to hold the current state.
    #[derive(Copy, Clone)]
    enum Tab {
        First,
        Second,
    }

    let tab = Mutable::new(Tab::First);

    // Next we map a signal from the current tab to the element we want to display.
    // We have to convert the element into a [`Node`] because the elements for each
    // tab are different types.
    let tab_element = tab.signal().map(|t| -> Node {
        match t {
            Tab::First => p().text("First").into(),
            Tab::Second => div().text("Second").into(),
        }
    });

    // Now we define our app:
    let app: Node<Dry> = div().child(Sig(tab_element)).into();

    // Then render the initial page.
    render_now_sync();
    check(&app, "<div><p>First</p></div>");

    // As we would expect, when we change the tab through the [`Mutable`], our app
    // updates:
    tab.set(Tab::Second);
    render_now_sync();
    check(&app, "<div><div>Second</div></div>");

    // ### Optional Child Signal
    //
    // Optional child signals are much the same as child signals, but the
    // element will appear and disappear base on whether the value i current
    // `Some` or `None`.`

    // ### Child Signal Vector

    // [`Node`]: https://docs.rs/silkenweb/latest/silkenweb/node/struct.Node.html
    // [`Mutable`]: https://docs.rs/futures-signals/latest/futures_signals/signal/struct.Mutable.html
    // [`MutableVec`]: https://docs.rs/futures-signals/latest/futures_signals/signal_vec/struct.MutableVec.html
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
