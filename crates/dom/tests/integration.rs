use silkenweb_dom::{mount, render_updates, tag, unmount};
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

fn create_app_container(app_id: &str) {
    let app_container = document().create_element("div").unwrap();
    app_container.set_id(app_id);
    document()
        .body()
        .unwrap()
        .append_child(&app_container)
        .unwrap();
}

fn app_html() -> String {
    document()
        .query_selector("#app")
        .unwrap()
        .unwrap()
        .inner_html()
}

fn document() -> dom::Document {
    dom::window().unwrap().document().unwrap()
}

const APP_ID: &str = "app";
