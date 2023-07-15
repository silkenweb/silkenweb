use silkenweb::{document::Document, dom::DefaultDom, task::render_now};
use silkenweb_base::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub async fn setup_test(mount_point_id: &str) {
    // Clear the render queue
    render_now().await;
    DefaultDom::unmount_all();

    if let Some(existing) = try_find_element(APP_CONTAINER_ID) {
        existing.remove()
    }

    let app_container = document::create_element("div");
    app_container.set_id(APP_CONTAINER_ID);
    let mount_point = document::create_element("div");
    mount_point.set_id(mount_point_id);
    app_container
        .append_child(&mount_point)
        .expect_throw("Couldn't add mount point to app container");
    let body = document::body().expect_throw("Couldn't get document body");
    body.append_child(&app_container)
        .expect_throw("Couldn't add app container to body");
}

pub fn mounted_html() -> String {
    find_element(APP_CONTAINER_ID).inner_html()
}

pub fn find_element(id: &str) -> web_sys::HtmlElement {
    try_find_element(id).expect_throw("Element not found")
}

pub fn try_find_element(id: &str) -> Option<web_sys::HtmlElement> {
    document::query_selector(&format!("#{id}"))
        .expect_throw("Error searching for element")
        .map(|elem| {
            elem.dyn_into()
                .expect_throw("Element was not an `HTMLElement")
        })
}

const APP_CONTAINER_ID: &str = "silkenweb-test-mount-point";
