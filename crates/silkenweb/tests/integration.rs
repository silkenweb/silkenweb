use silkenweb::{
    elements::{button, div, p, PBuilder},
    mount, render_updates,
    signal::{Signal, WriteSignal},
    tag, unmount,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys as dom;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn mount_unmount() {
    create_app_container(APP_ID);

    let message = "Hello, world!";
    mount(APP_ID, tag("p").text(message));
    render_updates();
    assert_eq!(format!(r#"<p>{}</p>"#, message), app_html());
    unmount(APP_ID);
    assert_eq!("", app_html());
}

#[wasm_bindgen_test]
fn simple_counter() {
    const BUTTON_ID: &str = "increment";
    const COUNTER_ID: &str = "counter";

    create_app_container(APP_ID);

    let count = Signal::new(0);
    let inc = count.write();

    mount(
        APP_ID,
        div()
            .id(COUNTER_ID)
            .child(
                button()
                    .id(BUTTON_ID)
                    .on_click(move |_, _| inc.replace(|i| i + 1))
                    .text("+"),
            )
            .text(count.read().map(|i| format!("{}", i))),
    );

    render_updates();
    let counter_text = || query_element(COUNTER_ID).inner_text();
    assert_eq!("+0", counter_text(), "Counter is initially zero");
    query_element(BUTTON_ID).click();
    assert_eq!("+0", counter_text(), "Counter unchanged before render");
    render_updates();
    assert_eq!("+1", counter_text(), "Counter incremented after render");
}

const TEXT_ID: &str = "text";

fn text_content(text_id: &str) -> String {
    query_element(text_id).inner_text()
}

#[wasm_bindgen_test]
fn reactive_text() {
    create_app_container(APP_ID);

    let text_signal = Signal::new("0");
    verify_reactive_text(
        p().id("text").text(&text_signal.read()),
        TEXT_ID,
        &text_signal.write(),
    );
}

// Verify reactive text when passing the signal by reference.
#[wasm_bindgen_test]
fn reactive_text_reference() {
    create_app_container(APP_ID);

    let text_signal = Signal::new("0");
    verify_reactive_text(
        p().id("text").text(&text_signal.read()),
        TEXT_ID,
        &text_signal.write(),
    );
}

// Make sure text is overwritten when called twice
#[wasm_bindgen_test]
fn reactive_text_reset() {
    create_app_container(APP_ID);

    let text_overwritten = Signal::new("This should be overwritten");
    let text_signal = Signal::new("0");

    mount(
        APP_ID,
        p().id(TEXT_ID)
            .text(text_overwritten.read())
            .text(text_signal.read()),
    );

    render_updates();
    assert_eq!("0", text_content(TEXT_ID));
    text_overwritten.write().set("This should be discarded");
    render_updates();
    assert_eq!("0", text_content(TEXT_ID), "The last call to `text()` wins");
    text_signal.write().set("1");
    render_updates();
    assert_eq!(
        "1",
        text_content(TEXT_ID),
        "The text is reactive to the last `text()` call"
    );
}

fn verify_reactive_text(paragraph: PBuilder, text_id: &str, text: &WriteSignal<&'static str>) {
    mount(APP_ID, paragraph);
    render_updates();
    assert_eq!("0", text_content(text_id));
    text.set("1");
    assert_eq!(
        "0",
        text_content(text_id),
        "Text unaffected by signal before render"
    );
    render_updates();
    assert_eq!(
        "1",
        text_content(text_id),
        "Text updated by signal after render"
    );
}

fn create_app_container(app_id: &str) {
    // Clear the render queue
    render_updates();
    let app_container = document().create_element("div").unwrap();
    app_container.set_id(app_id);
    let body = document().body().unwrap();
    body.append_child(&app_container).unwrap();
}

fn query_element(id: &str) -> dom::HtmlElement {
    document()
        .query_selector(&format!("#{}", id))
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap()
}

fn app_html() -> String {
    query_element(APP_ID).inner_html()
}

fn document() -> dom::Document {
    dom::window().unwrap().document().unwrap()
}

const APP_ID: &str = "app";
