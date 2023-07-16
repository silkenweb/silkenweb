use std::rc::Rc;

use super::state::CounterState;
use silkenweb::{
    dom::Dom,
    elements::html::{div, Div},
    hydration::hydrate,
    prelude::{
        html::{button, h1, Button},
        ElementEvents, HtmlElement, ParentElement,
    },
    task::spawn_local,
    value::Sig,
};

#[allow(dead_code)]
pub fn hydrate_app() {
    let body = app();

    spawn_local(async {
        hydrate("body", body).await;
    });
}

pub fn app<D: Dom>() -> Div<D> {
    let count = Rc::new(CounterState::default());

    div()
        .id("body")
        .child(h1().text("Counter"))
        .child(div().text(Sig(count.text())))
        .child(count_button("+", 1, count.clone()))
        .child(count_button("-", -1, count))
}

pub fn count_button<D: Dom>(label: &str, delta: isize, count: Rc<CounterState>) -> Button<D> {
    button().on_click(move |_, _| count.add(delta)).text(label)
}
