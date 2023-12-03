use futures_signals::signal::Mutable;
use silkenweb::{
    document::{Document, DocumentHead},
    dom::DefaultDom,
    elements::html::{button, div, p, P},
    mount,
    node::element::{ParentElement, TextParentElement},
    prelude::{ElementEvents, HtmlElement},
    task::render_now,
    value::Sig,
};
use silkenweb_test::{html_element, BrowserTest};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

macro_rules! isomorphic_test {
    (async fn $name:ident() $body:block) => {
        #[silkenweb_macros::cfg_browser(false)]
        #[test]
        fn $name() {
            silkenweb::task::server::block_on(::silkenweb::task::scope(async { $body }));
        }

        #[silkenweb_macros::cfg_browser(true)]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {
            $body
        }
    };
}

mod children;
mod component;
mod css;
mod element;
mod hydration;
mod template;

wasm_bindgen_test_configure!(run_in_browser);

isomorphic_test! {
    async fn head_inner_html() {
        DefaultDom::unmount_all();

        assert_eq!(DefaultDom::head_inner_html(), "");

        // Test escaping
        let id = "my-id:0:1";
        DefaultDom::mount_in_head(id, DocumentHead::new().child(div())).unwrap();
        let head_html = r#"<div></div>"#;
        render_now().await;
        assert_eq!(DefaultDom::head_inner_html(), head_html);
        DefaultDom::mount_in_head(id, DocumentHead::new().child(div())).unwrap();
        render_now().await;
        assert_eq!(DefaultDom::head_inner_html(), head_html);
    }
}

#[wasm_bindgen_test]
async fn mount_unmount() {
    let test = BrowserTest::new(APP_ID).await;

    let message = "Hello, world!";
    let mount_handle = mount(APP_ID, p().id(APP_ID).text(message));
    render_now().await;
    assert_eq!(format!(r#"<p id="app">{message}</p>"#), test.html());
    mount_handle.unmount();
    assert_eq!(r#"<div id="app"></div>"#, test.html());
}

#[wasm_bindgen_test]
async fn simple_counter() {
    const BUTTON_ID: &str = "increment";
    const COUNTER_ID: &str = "counter";

    let _test = BrowserTest::new(APP_ID).await;

    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{i}"));

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
    let counter_text = || html_element(COUNTER_ID).inner_text();
    assert_eq!("+0", counter_text(), "Counter is initially zero");
    html_element(BUTTON_ID).click();
    assert_eq!("+0", counter_text(), "Counter unchanged before render");
    render_now().await;
    assert_eq!("+1", counter_text(), "Counter incremented after render");
}

const TEXT_ID: &str = "text";

fn text_content(text_id: &str) -> String {
    html_element(text_id).inner_text()
}

#[wasm_bindgen_test]
async fn reactive_text() {
    let _test = BrowserTest::new(APP_ID).await;

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
    let _test = BrowserTest::new(APP_ID).await;

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
    let _test = BrowserTest::new(APP_ID).await;

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

async fn verify_reactive_text(paragraph: P, text_id: &str, text: &mut Mutable<&'static str>) {
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

#[test]
fn macro_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macro-ui/*.rs");
}

const APP_ID: &str = "app";
