use futures_signals::signal::Mutable;
use silkenweb::prelude::ParentBuilder;
use silkenweb_dom::{hydrate, render::render_now};
use silkenweb_elements::{
    html::{div, p},
    macros::Element,
};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{app_html, create_app_container, query_element, APP_ID};

#[wasm_bindgen_test]
async fn missing_text() {
    app_container(APP_ID, r#"<div data-silkenweb="1"><p></p></div>"#).await;
    let app = div().child(p().text("Hello, world!"));

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
                <p>
                </p>
            </div>
        "#,
    )
    .await;

    let app = div().child(p().text("Hello, world!"));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;
}

#[wasm_bindgen_test]
async fn basic_signal() {
    app_container(APP_ID, r#"<div data-silkenweb="1"><p></p></div>"#).await;
    let text = Mutable::new("Hello, world!");
    let app = div().child(p().text_signal(text.signal()));

    test_hydrate(
        APP_ID,
        app,
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
    )
    .await;

    text.set("Some more text");
    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Some more text</p></div>"#,
        app_html(APP_ID)
    );
}

async fn app_container(id: &str, inner_html: &str) {
    create_app_container(id).await;
    query_element(id).set_inner_html(inner_html);
}

async fn test_hydrate(id: &str, app: impl Into<Element>, expected_html: &str) {
    render_now().await;
    hydrate(id, app).await;

    assert_eq!(expected_html, app_html(id));
}

// TODO: Test element reconciliation: Empty text, additional elements, missing
// attributes, extra attributes
