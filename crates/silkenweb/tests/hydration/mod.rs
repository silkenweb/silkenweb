use futures_signals::signal::Mutable;
use silkenweb::prelude::ParentBuilder;
use silkenweb_dom::{hydrate, render::render_now};
use silkenweb_elements::html::{div, p};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{create_app_container, query_element, APP_ID};

#[wasm_bindgen_test]
async fn missing_text() {
    create_app_container(APP_ID).await;

    query_element(APP_ID).set_inner_html(r#"<div data-silkenweb="1"><p></p></div>"#);
    let app = div().child(p().text("Hello, world!"));

    hydrate(APP_ID, app);
    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
        query_element(APP_ID).inner_html()
    );
}

#[wasm_bindgen_test]
async fn basic_signal() {
    create_app_container(APP_ID).await;

    query_element(APP_ID).set_inner_html(r#"<div data-silkenweb="1"><p></p></div>"#);
    let text = Mutable::new("Hello, world!");
    let app = div().child(p().text_signal(text.signal()));

    hydrate(APP_ID, app);

    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Hello, world!</p></div>"#,
        query_element(APP_ID).inner_html()
    );

    text.set("Some more text");
    render_now().await;
    assert_eq!(
        r#"<div data-silkenweb="1"><p>Some more text</p></div>"#,
        query_element(APP_ID).inner_html()
    );
}

// TODO: Test element reconciliation: Empty text, additional elements, missing attributes, extra attributes