use silkenweb::{
    elements::{button, div},
    mount,
    render_updates,
    signal::Signal,
    tag,
    unmount,
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
    const INC_ID: &str = "increment";
    const COUNTER_ID: &str = "counter";

    fn check_counter(expected: usize) {
        assert_eq!(
            format!("+{}", expected),
            query_element(COUNTER_ID).inner_text()
        );
    }

    create_app_container(APP_ID);

    let count = Signal::new(0);
    let inc = count.write();

    mount(
        APP_ID,
        div()
            .id(COUNTER_ID)
            .child(
                button()
                    .id(INC_ID)
                    .on_click(move |_, _| inc.replace(|i| i + 1))
                    .text("+"),
            )
            .text(count.read().map(|i| format!("{}", i))),
    );
    
    render_updates();
    check_counter(0);
    query_element(INC_ID).click();
    check_counter(0);
    render_updates();
    check_counter(1);
}

fn create_app_container(app_id: &str) {
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
    document()
        .query_selector(&format!("#{}", APP_ID))
        .unwrap()
        .unwrap()
        .inner_html()
}

fn document() -> dom::Document {
    dom::window().unwrap().document().unwrap()
}

const APP_ID: &str = "app";
