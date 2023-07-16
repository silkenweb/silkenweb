//! Test combinations of optional children followed by a `SignalVec` children.
//!
//! Exhaustively test all combinations of adding 3 optional children, adding a
//! signal vec of length 3 or less, and mutating the optional children and child
//! vec.
use std::fmt::Display;

use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, MutableVecLockMut, SignalVecExt},
};
use itertools::Itertools;
use silkenweb::{
    dom::Dom,
    elements::html::{div, Div},
    macros::Signal,
    mount,
    prelude::{HtmlElement, ParentElement},
    task::render_now,
    value::Sig,
};
use silkenweb_test::BrowserTest;

use crate::APP_ID;

#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_all_children() {
    for bits in 0..64 {
        let optional_children = [
            OptionalChildStates {
                initial: bit_is_set(bits, 0),
                updated: bit_is_set(bits, 1),
            },
            OptionalChildStates {
                initial: bit_is_set(bits, 2),
                updated: bit_is_set(bits, 3),
            },
            OptionalChildStates {
                initial: bit_is_set(bits, 4),
                updated: bit_is_set(bits, 5),
            },
        ];

        for initial_children_len in 0..3 {
            if initial_children_len > 0 {
                test_children(optional_children, initial_children_len, |children| {
                    children.pop();
                })
                .await;
            }

            test_children(optional_children, initial_children_len, |children| {
                children.push(initial_children_len);
            })
            .await;

            test_children(optional_children, initial_children_len, |children| {
                children.clear();
            })
            .await;

            for i in 0..initial_children_len {
                test_children(optional_children, initial_children_len, |children| {
                    children.remove(i);
                })
                .await;

                test_children(optional_children, initial_children_len, |children| {
                    children.insert(i, 100);
                })
                .await;
            }

            for from in 0..initial_children_len {
                for to in 0..(initial_children_len - 1) {
                    test_children(optional_children, initial_children_len, |children| {
                        children.move_from_to(from, to);
                    })
                    .await;
                }
            }
        }
    }
}

async fn test_children(
    optional_children: [OptionalChildStates; 3],
    children_vec_len: usize,
    children_mutator: impl FnOnce(&mut MutableVecLockMut<usize>),
) {
    let test = BrowserTest::new(APP_ID).await;

    let mut parent = div().id(APP_ID);
    let optional_child_mutables: Vec<Mutable<bool>> = optional_children
        .iter()
        .map(|state| Mutable::new(state.initial))
        .collect();
    let optional_child_signals = optional_child_mutables
        .iter()
        .enumerate()
        .map(|(index, is_some)| optional_child(index, is_some));

    for child in optional_child_signals {
        parent = parent.optional_child(Sig(child));
    }

    let children_vec = MutableVec::new_with_values((0..children_vec_len).collect());
    let parent = parent.children_signal(children_vec.signal_vec().map(child));

    mount(APP_ID, parent);
    check(
        &test,
        optional_children.iter().map(|state| state.initial),
        0..children_vec_len,
    )
    .await;

    for (child, state) in optional_child_mutables
        .into_iter()
        .zip(optional_children.iter())
    {
        child.set_neq(state.updated);
    }

    children_mutator(&mut children_vec.lock_mut());

    check(
        &test,
        optional_children.iter().map(|state| state.updated),
        children_vec.lock_ref().as_slice().iter().cloned(),
    )
    .await;
}

async fn check(
    test: &BrowserTest,
    optional_children: impl IntoIterator<Item = bool>,
    children: impl IntoIterator<Item = usize>,
) {
    render_now().await;

    let optional_children_html = optional_children
        .into_iter()
        .enumerate()
        .filter_map(|(index, is_some)| is_some.then(|| div_html(index)));
    let children_html = children.into_iter().map(div_html);
    let inner_html = optional_children_html.chain(children_html).join("");

    assert_eq!(test.html(), format!(r#"<div id="app">{inner_html}</div>"#))
}

fn child<D: Dom>(index: usize) -> Div<D> {
    div().text(format!("{index}"))
}

fn optional_child<D: Dom>(
    index: usize,
    is_some: &Mutable<bool>,
) -> impl Signal<Item = Option<Div<D>>> {
    is_some.signal_ref(move |is_some| is_some.then(|| child(index)))
}

fn div_html(inner: impl Display) -> String {
    format!("<div>{inner}</div>")
}

#[derive(Copy, Clone)]
struct OptionalChildStates {
    initial: bool,
    updated: bool,
}

fn bit_is_set(i: usize, bit_index: usize) -> bool {
    ((i >> bit_index) & 1) == 1
}
