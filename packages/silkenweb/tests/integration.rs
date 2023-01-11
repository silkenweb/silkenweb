use futures_signals::signal::Mutable;
use silkenweb::{
    dom::{self, Wet},
    elements::html::{button, div, p, P},
    mount,
    node::element::ParentElement,
    prelude::{ElementEvents, HtmlElement},
    remove_all_mounted,
    task::render_now,
    value::Sig,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

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

mod children;
mod component;
mod element;
mod hydration;
mod template;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
type PlatformDom = dom::Wet;

#[cfg(not(target_arch = "wasm32"))]
type PlatformDom = dom::Dry;

#[wasm_bindgen_test]
async fn mount_unmount() {
    create_app_container(APP_ID).await;

    let message = "Hello, world!";
    let mount_handle = mount(APP_ID, p().id(APP_ID).text(message));
    render_now().await;
    assert_eq!(format!(r#"<p id="app">{}</p>"#, message), app_html(APP_ID));
    mount_handle.unmount();
    assert_eq!(r#"<div id="app"></div>"#, app_html(APP_ID));
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
            .text(Sig(count_text)),
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
        p().id("text").text(Sig(text_signal.signal())),
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
        p().id("text").text(Sig(text_signal.signal())),
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
            .text(Sig(first_text.signal()))
            .text(Sig(second_text.signal())),
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

async fn verify_reactive_text(paragraph: P<Wet>, text_id: &str, text: &mut Mutable<&'static str>) {
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
    remove_all_mounted();
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

fn app_html(id: &str) -> String {
    query_element(id).outer_html()
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap_throw().document().unwrap_throw()
}

const APP_ID: &str = "app";
