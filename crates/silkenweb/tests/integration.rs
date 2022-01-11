use futures_signals::signal::Mutable;
use silkenweb::{
    dom::{mount, render::render_now, tag, unmount},
    elements::{
        html::{button, div, p, PBuilder},
        ElementEvents,
    },
};
use silkenweb_elements::{HtmlElement, ParentBuilder};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn mount_unmount() {
    create_app_container(APP_ID).await;

    let message = "Hello, world!";
    mount(APP_ID, tag("p").text(message));
    render_now().await;
    assert_eq!(format!(r#"<p>{}</p>"#, message), app_html());
    unmount(APP_ID);
    assert_eq!("", app_html());
}

#[wasm_bindgen_test]
async fn simple_counter() {
    const BUTTON_ID: &str = "increment";
    const COUNTER_ID: &str = "counter";

    create_app_container(APP_ID).await;

    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{}", i));

    mount(
        APP_ID,
        div()
            .id(COUNTER_ID)
            .child(
                button()
                    .id(BUTTON_ID)
                    .on_click(move |_, _| {
                        count.replace_with(|i| *i + 1);
                    })
                    .text("+"),
            )
            .text_signal(count_text),
    );

    render_now().await;
    let counter_text = || query_element(COUNTER_ID).inner_text();
    assert_eq!("+0", counter_text(), "Counter is initially zero");
    query_element(BUTTON_ID).click();
    assert_eq!("+0", counter_text(), "Counter unchanged before render");
    render_now().await;
    assert_eq!("+1", counter_text(), "Counter incremented after render");
}

const TEXT_ID: &str = "text";

fn text_content(text_id: &str) -> String {
    query_element(text_id).inner_text()
}

#[wasm_bindgen_test]
async fn reactive_text() {
    create_app_container(APP_ID).await;

    let mut text_signal = Mutable::new("0");
    verify_reactive_text(
        p().id("text").text_signal(text_signal.signal()),
        TEXT_ID,
        &mut text_signal,
    )
    .await;
}

// Verify reactive text when passing the signal by reference.
#[wasm_bindgen_test]
async fn reactive_text_reference() {
    create_app_container(APP_ID).await;

    let mut text_signal = Mutable::new("0");
    verify_reactive_text(
        p().id("text").text_signal(text_signal.signal()),
        TEXT_ID,
        &mut text_signal,
    )
    .await;
}

// Make we support multiple reactive text children
#[wasm_bindgen_test]
async fn multiple_reactive_text() {
    create_app_container(APP_ID).await;

    let first_text = Mutable::new("{First 0}");
    let second_text = Mutable::new("{Second 0}");

    mount(
        APP_ID,
        p().id(TEXT_ID)
            .text_signal(first_text.signal())
            .text_signal(second_text.signal()),
    );

    render_now().await;
    assert_eq!("{First 0}{Second 0}", text_content(TEXT_ID));
    first_text.set("{First 1}");
    render_now().await;
    assert_eq!(
        "{First 1}{Second 0}",
        text_content(TEXT_ID),
        "First is updated"
    );
    second_text.set("{Second 1}");
    render_now().await;
    assert_eq!(
        "{First 1}{Second 1}",
        text_content(TEXT_ID),
        "Second is updated"
    );
}

async fn verify_reactive_text(
    paragraph: PBuilder,
    text_id: &str,
    text: &mut Mutable<&'static str>,
) {
    mount(APP_ID, paragraph);
    render_now().await;
    assert_eq!("0", text_content(text_id));
    text.set("1");
    assert_eq!(
        "0",
        text_content(text_id),
        "Text unaffected by signal before render"
    );
    render_now().await;
    assert_eq!(
        "1",
        text_content(text_id),
        "Text updated by signal after render"
    );
}

async fn create_app_container(app_id: &str) {
    // Clear the render queue
    render_now().await;
    let app_container = document().create_element("div").unwrap_throw();
    app_container.set_id(app_id);
    let body = document().body().unwrap_throw();
    body.append_child(&app_container).unwrap_throw();
}

fn query_element(id: &str) -> web_sys::HtmlElement {
    document()
        .query_selector(&format!("#{}", id))
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw()
}

fn app_html() -> String {
    query_element(APP_ID).inner_html()
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap_throw().document().unwrap_throw()
}

const APP_ID: &str = "app";
