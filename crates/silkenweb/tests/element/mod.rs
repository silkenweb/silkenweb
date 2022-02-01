use futures_signals::signal_vec::{MutableVec, MutableVecLockMut, SignalVecExt};
use silkenweb::{dom::node::element::Element, prelude::ParentBuilder};
use silkenweb_dom::render::render_now_sync;
use silkenweb_elements::html::{div, p};

use crate::{namespace_elems, namespace_text};

fn check(node: impl Into<Element>, expected: &str) {
    render_now_sync();
    assert_eq!(format!("{}", node.into()), expected)
}

#[test]
fn empty_element() {
    check(div(), "<div></div>");
}

#[test]
fn child() {
    check(div().child(p().text("Hello!")), "<div><p>Hello!</p></div>");
}

#[test]
fn children() {
    check(
        div().children([p().text("Hello"), p().text("World!")]),
        "<div><p>Hello</p><p>World!</p></div>",
    );
}

#[test]
fn namespaces() {
    check(namespace_elems(), &namespace_text());
}

#[test]
fn empty_children_signal() {
    check_children_signal([], |_| (), []);
}

#[test]
fn append_child() {
    check_children_signal([], |mut children| children.push(0), [0]);
}

#[test]
fn children_pop_to_empty() {
    check_children_signal(
        [0],
        |mut children| {
            children.pop();
        },
        [],
    );
}

#[test]
fn children_pop() {
    check_children_signal(
        [0, 1],
        |mut children| {
            children.pop();
        },
        [0],
    );
}

fn check_children_signal<const INITIAL_COUNT: usize, const EXPECTED_COUNT: usize>(
    initial: [usize; INITIAL_COUNT],
    f: impl FnOnce(MutableVecLockMut<usize>),
    expected: [usize; EXPECTED_COUNT],
) {
    let children = MutableVec::<usize>::new_with_values(initial.to_vec());
    let element = div().children_signal(children.signal_vec().map(|i| p().text(&format!("{}", i))));

    f(children.lock_mut());
    let mut expected_html = String::new();

    for i in expected {
        expected_html.push_str(&format!("<p>{}</p>", i));
    }

    check(element, &format!("<div>{}</div>", expected_html));
}
