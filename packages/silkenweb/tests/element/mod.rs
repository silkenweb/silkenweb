use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, MutableVecLockMut, SignalVecExt},
};
use silkenweb::{
    dom,
    elements::{
        html::{Div, P},
        HtmlElement,
    },
    node::{element::Element, text, Node},
    prelude::ParentElement,
    task::render_now,
    value::Sig,
};

macro_rules! isomorphic_test {
    (async fn $name:ident() $body:block) => {
        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        fn $name() {
            silkenweb::task::server::block_on(async { $body });
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {
            $body
        }
    };
}

#[cfg(target_arch = "wasm32")]
type PlatformDom = dom::Wet;

#[cfg(not(target_arch = "wasm32"))]
type PlatformDom = dom::Dry;

macro_rules! render_test {
    ($name:ident, $node:expr, $expected:expr) => {
        isomorphic_test! {
            async fn $name() {
                assert_eq!(Node::<PlatformDom>::from($node).to_string(), $expected)
            }
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[should_panic]
fn block_on() {
    silkenweb::task::server::block_on(async { panic!("Make sure the future is run") });
}

render_test!(empty_element, Div::new(), "<div></div>");
render_test!(
    single_attribute,
    Div::new().id("my-id"),
    r#"<div id="my-id"></div>"#
);
render_test!(
    multi_attribute,
    Div::new().id("my-id").class("my-class"),
    r#"<div id="my-id" class="my-class"></div>"#
);
render_test!(
    boolean_false_attribute,
    Div::new().hidden(false),
    r#"<div></div>"#
);
render_test!(
    boolean_true_attribute,
    Div::new().hidden(true),
    r#"<div hidden=""></div>"#
);

render_test!(
    child,
    Div::new().child(P::new().text("Hello!")),
    "<div><p>Hello!</p></div>"
);
render_test!(
    children,
    Div::new().children([P::new().text("Hello"), P::new().text("World!")]),
    "<div><p>Hello</p><p>World!</p></div>"
);

// Make sure the test is actually run
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[should_panic]
fn test_children_signal_test() {
    silkenweb::task::server::block_on(children_signal_test(&[], |_| {}, &[0]));
}

macro_rules! children_signal_test {
    ($name:ident, $initial:expr, $operations:expr, $expected:expr) => {
        isomorphic_test! {
            async fn $name() {
                children_signal_test(&$initial, $operations, &$expected).await;
            }
        }
    };
}

children_signal_test!(empty_children_signal, [], |_| (), []);
children_signal_test!(append_child, [], |mut children| children.push(0), [0]);
children_signal_test!(
    children_pop_to_empty,
    [0],
    |mut children| {
        children.pop();
    },
    []
);

children_signal_test!(
    children_pop,
    [0, 1],
    |mut children| {
        children.pop();
    },
    [0]
);

children_signal_test!(
    children_insert_from_empty,
    [],
    |mut children| {
        children.insert(0, 0);
    },
    [0]
);

children_signal_test!(
    children_insert_first,
    [1],
    |mut children| {
        children.insert(0, 0);
    },
    [0, 1]
);

children_signal_test!(
    children_insert_middle,
    [0, 2],
    |mut children| {
        children.insert(1, 1);
    },
    [0, 1, 2]
);

children_signal_test!(
    children_insert_end,
    [0],
    |mut children| {
        children.insert(1, 1);
    },
    [0, 1]
);

children_signal_test!(
    children_replace_empty,
    [],
    |mut children| {
        children.replace(vec![0, 1]);
    },
    [0, 1]
);

children_signal_test!(
    children_replace_with_empty,
    [0, 1],
    |mut children| {
        children.replace(vec![]);
    },
    []
);

children_signal_test!(
    children_replace,
    [0, 1],
    |mut children| {
        children.replace(vec![2, 3]);
    },
    [2, 3]
);

children_signal_test!(
    children_clear,
    [0, 1],
    |mut children| {
        children.clear();
    },
    []
);

children_signal_test!(
    children_set,
    [0, 1],
    |mut children| {
        children.set(0, 1);
    },
    [1, 1]
);

children_signal_test!(
    children_remove_at,
    [0, 1, 2],
    |mut children| {
        children.remove(1);
    },
    [0, 2]
);

children_signal_test!(
    children_move_forward,
    [0, 1, 2, 3, 4],
    |mut children| {
        children.move_from_to(1, 2);
    },
    [0, 2, 1, 3, 4]
);

children_signal_test!(
    children_move_backward,
    [0, 1, 2, 3, 4],
    |mut children| {
        children.move_from_to(2, 1);
    },
    [0, 2, 1, 3, 4]
);

children_signal_test!(
    children_move_to_end,
    [0, 1, 2],
    |mut children| {
        children.move_from_to(1, 2);
    },
    [0, 2, 1]
);

children_signal_test!(
    children_move_from_end,
    [0, 1, 2],
    |mut children| {
        children.move_from_to(2, 1);
    },
    [0, 2, 1]
);

isomorphic_test! {
    async fn text_signal() {
        let text = Mutable::new("Initial text");
        let elem: Node<PlatformDom> = P::new().text(Sig(text.signal())).into();
        render_now().await;
        assert_eq!(elem.to_string(), "<p>Initial text</p>");
        text.set("Updated text");
        render_now().await;
        assert_eq!(elem.to_string(), "<p>Updated text</p>");
    }
}

isomorphic_test! {
    async fn classes_signal() {
        let test_class1 = Mutable::new(Some("test-class-1"));
        let test_class2 = Mutable::new(Some("test-class-2"));
        let elem: Node<PlatformDom> =
            Div::new()
                .classes(Sig(test_class1.signal()))
                .classes(Sig(test_class2.signal()))
                .into();

        render_now().await;
        assert_eq!(elem.to_string(), r#"<div class="test-class-1 test-class-2"></div>"#);

        test_class1.set(None);
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div class="test-class-2"></div>"#);

        test_class2.set(None);
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div class=""></div>"#);

        test_class1.set(Some("test-class-1"));
        test_class2.set(Some("test-class-2"));
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div class="test-class-1 test-class-2"></div>"#);
    }
}

isomorphic_test! {
    async fn attribute_signal() {
        let text = Mutable::new("Initial text");
        let elem: Node<PlatformDom> = Div::new().title(Sig(text.signal())).into();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Initial text"></div>"#);
        text.set("Updated text");
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Updated text"></div>"#);
    }
}

isomorphic_test! {
    async fn optional_attribute_signal() {
        let text = Mutable::new(Some("Initial text"));
        let elem: Node<PlatformDom> = Div::new().title(Sig(text.signal())).into();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Initial text"></div>"#);
        text.set(None);
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div></div>"#);
    }
}

isomorphic_test! {
    async fn text_node() {
        let elem: Node<PlatformDom> = Div::new().child(text("Hello, world!")).into();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div>Hello, world!</div>"#);
    }
}

pub async fn children_signal_test(
    initial: &[usize],
    f: impl Fn(MutableVecLockMut<usize>) + Clone,
    expected: &[usize],
) {
    async fn with_existing_children(
        initial_elem: Div<PlatformDom>,
        initial_child_text: &str,
        initial: &[usize],
        f: impl FnOnce(MutableVecLockMut<usize>),
        expected: &[usize],
    ) {
        let children = MutableVec::<usize>::new_with_values(initial.to_vec());
        let element: Node<PlatformDom> = initial_elem
            .children_signal(
                children
                    .signal_vec()
                    .map(|i| P::new().text(format!("{}", i))),
            )
            .into();

        f(children.lock_mut());
        let mut expected_html = String::new();

        for i in expected {
            expected_html.push_str(&format!("<p>{}</p>", i));
        }

        render_now().await;
        assert_eq!(
            element.to_string(),
            format!("<div>{}{}</div>", initial_child_text, expected_html)
        );
    }

    with_existing_children(Div::new(), "", initial, f.clone(), expected).await;
    with_existing_children(
        Div::new().child(Div::new()),
        "<div></div>",
        initial,
        f,
        expected,
    )
    .await;
}
