use futures_signals::signal::Mutable;
use silkenweb::{
    dom::hydro::Dry,
    elements::{
        html::{Button, Div, P},
        ElementEvents, HtmlElement,
    },
    hydration::hydrate,
    node::Node,
    prelude::ParentElement,
    task::render_now,
    unmount,
    value::Sig,
};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{app_html, create_app_container, query_element, APP_ID};

#[wasm_bindgen_test]
async fn missing_text() {
    app_container(APP_ID, r#"<div data-silkenweb="1"><p></p></div>"#).await;
    let app = Div::new().child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn blank_text() {
    app_container(
        APP_ID,
        r#"
            <div data-silkenweb="1">
                <p>Hello, world!</p>
            </div>
        "#,
    )
    .await;

    let app = Div::new().child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn extra_child() {
    app_container(
        APP_ID,
        r#"<div data-silkenweb="1"><p>Hello, world!</p><div></div></div>"#,
    )
    .await;

    let app = Div::new().child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn mismatched_element() {
    app_container(
        APP_ID,
        r#"<div data-silkenweb="1"><div>Hello, world!</div></div>"#,
    )
    .await;

    let app = Div::new().child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn extra_attribute() {
    app_container(
        APP_ID,
        r#"<div data-silkenweb="1" id="0"><p>Hello, world!</p></div>"#,
    )
    .await;

    let app = Div::new().child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn missing_attribute() {
    app_container(
        APP_ID,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;

    let app = Div::new().id("0").child(P::new().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1" id="0"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn event() {
    const BUTTON_ID: &str = "increment";
    const COUNTER_ID: &str = "counter";

    app_container(
        APP_ID,
        r#"<div id="counter"><button id="increment" data-silkenweb="button-data">+</button>0</div>"#,
    )
    .await;

    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{}", i));

    render_now().await;

    hydrate(
        APP_ID,
        Div::new()
            .id(COUNTER_ID)
            .child(
                Button::new()
                    .id(BUTTON_ID)
                    .on_click(move |_, _| {
                        count.replace_with(|i| *i + 1);
                    })
                    .text("+"),
            )
            .text(Sig(count_text)),
    )
    .await;

    render_now().await;
    let counter_text = || query_element(COUNTER_ID).inner_text();
    assert_eq!("+0", counter_text(), "Counter is initially zero");
    query_element(BUTTON_ID).click();
    render_now().await;
    assert_eq!("+1", counter_text(), "Counter incremented after render");

    assert_eq!(
        app_html(APP_ID),
        r#"<div id="counter"><button id="increment" data-silkenweb="button-data">+</button>1</div>"#
    );
    unmount(APP_ID);
}

#[wasm_bindgen_test]
async fn basic_signal() {
    app_container(APP_ID, r#"<div data-silkenweb="1"><p></p></div>"#).await;
    let text = Mutable::new("Hello, world!");
    let app = Div::new().child(P::new().text(Sig(text.signal())));

    render_now().await;
    hydrate(APP_ID, app).await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
        app_html(APP_ID)
    );

    text.set("Some more text");
    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Some more text</p></div>"#,
        app_html(APP_ID)
    );
    unmount(APP_ID);
}

#[wasm_bindgen_test]
async fn nested_signal() {
    app_container(APP_ID, r#"<div data-silkenweb="1"><p><p></p></p></div>"#).await;
    let text = Mutable::new("Hello, world!");
    let app = Div::new().child(P::new().child(P::new().text(Sig(text.signal()))));

    render_now().await;
    hydrate(APP_ID, app).await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p><p>Hello, world!</p></p></div>"#,
        app_html(APP_ID)
    );

    text.set("Some more text");
    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p><p>Some more text</p></p></div>"#,
        app_html(APP_ID)
    );
    unmount(APP_ID);
}

async fn app_container(id: &str, inner_html: &str) {
    create_app_container(id).await;
    query_element(id).set_inner_html(inner_html);
}

async fn test_hydrate(id: &str, app: impl Into<Node<Dry>>, expected_html: &str) {
    render_now().await;
    hydrate(id, app).await;

    assert_eq!(expected_html, app_html(id));
    unmount(id);
}
