use silkenweb::prelude::ParentBuilder;
use silkenweb_elements::{
    html::{div, p},
    HtmlElement,
};

macro_rules! render_test {
    ($name:ident, $node:expr, $expected:expr) => {
        #[cfg(all(not(target_arch = "wasm32"), feature = "server-side-render"))]
        #[test]
        fn $name() {
            native::check($node, $expected);
        }

        #[cfg(all(target_arch = "wasm32", not(feature = "server-side-render")))]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {
            browser::check($node, $expected).await;
        }
    };
}

render_test!(empty_element, div(), "<div></div>");
render_test!(
    single_attribute,
    div().id("my-id"),
    r#"<div id="my-id"></div>"#
);
render_test!(
    multi_attribute,
    div().id("my-id").class(["my-class"]),
    r#"<div id="my-id" class="my-class"></div>"#
);

render_test!(
    child,
    div().child(p().text("Hello!")),
    "<div><p>Hello!</p></div>"
);
render_test!(
    children,
    div().children([p().text("Hello"), p().text("World!")]),
    "<div><p>Hello</p><p>World!</p></div>"
);

macro_rules! children_signal_test {
    ($name:ident, $initial:expr, $operations:expr, $expected:expr) => {
        #[cfg(all(not(target_arch = "wasm32"), feature = "server-side-render"))]
        #[test]
        fn $name() {
            native::check_children_signal($initial, $operations, $expected);
        }

        #[cfg(all(target_arch = "wasm32", not(feature = "server-side-render")))]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {
            browser::check_children_signal($initial, $operations, $expected).await;
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

mod shared {
    use futures_signals::signal_vec::{MutableVec, MutableVecLockMut, SignalVecExt};
    use silkenweb::prelude::ParentBuilder;
    use silkenweb_elements::{
        html::{div, p, Div},
        macros::Element,
    };

    pub fn check(node: impl Into<Element>, expected: &str) {
        assert_eq!(format!("{}", node.into()), expected)
    }

    pub fn check_children_signal(
        initial: &[usize],
        f: impl FnOnce(MutableVecLockMut<usize>),
        expected: &[usize],
    ) -> (Div, String) {
        let children = MutableVec::<usize>::new_with_values(initial.to_vec());
        let element =
            div().children_signal(children.signal_vec().map(|i| p().text(&format!("{}", i))));

        f(children.lock_mut());
        let mut expected_html = String::new();

        for i in expected {
            expected_html.push_str(&format!("<p>{}</p>", i));
        }

        (element, format!("<div>{}</div>", expected_html))
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "server-side-render"))]
mod native {
    use futures_signals::signal_vec::MutableVecLockMut;
    use silkenweb_dom::render::render_now_sync;
    use silkenweb_elements::macros::Element;

    use crate::element::shared;

    pub fn check(node: impl Into<Element>, expected: &str) {
        render_now_sync();
        shared::check(node, expected);
    }

    pub fn check_children_signal<const INITIAL_COUNT: usize, const EXPECTED_COUNT: usize>(
        initial: [usize; INITIAL_COUNT],
        f: impl FnOnce(MutableVecLockMut<usize>),
        expected: [usize; EXPECTED_COUNT],
    ) {
        let (element, expected) = shared::check_children_signal(&initial, f, &expected);
        render_now_sync();
        shared::check(element, &expected);
    }
}

#[cfg(all(target_arch = "wasm32", not(feature = "server-side-render")))]
mod browser {
    use futures_signals::signal_vec::MutableVecLockMut;
    use silkenweb_dom::render::render_now;
    use silkenweb_elements::macros::Element;

    use crate::element::shared;

    pub async fn check(node: impl Into<Element>, expected: &str) {
        render_now().await;
        shared::check(node, expected);
    }

    pub async fn check_children_signal<const INITIAL_COUNT: usize, const EXPECTED_COUNT: usize>(
        initial: [usize; INITIAL_COUNT],
        f: impl FnOnce(MutableVecLockMut<usize>),
        expected: [usize; EXPECTED_COUNT],
    ) {
        let (element, expected) = shared::check_children_signal(&initial, f, &expected);
        render_now().await;
        check(element, &expected).await;
    }
}
