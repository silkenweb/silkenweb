//! Utilities for writing tests for Silkenweb apps.
use silkenweb::{document::Document, dom::DefaultDom, task::render_now};
use silkenweb_base::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

/// Setup a test.
///
/// This:
///
/// - Clears the render queue.
/// - Unmounts any mounted elements.
/// - Removes any existing mount points.
/// - Creates new mount points with `id`s from `mount_point_ids`.
pub async fn setup_test(mount_point_ids: impl IntoIterator<Item = &str>) {
    // Clear the render queue
    render_now().await;
    DefaultDom::unmount_all();

    if let Some(existing) = try_find_html_element(APP_CONTAINER_ID) {
        existing.remove()
    }

    let app_container = document::create_element("div");
    app_container.set_id(APP_CONTAINER_ID);

    for mount_point_id in mount_point_ids {
        let mount_point = document::create_element("div");
        mount_point.set_id(mount_point_id);
        app_container
            .append_child(&mount_point)
            .expect_throw("Couldn't add mount point to app container");
    }

    let body = document::body().expect_throw("Couldn't get document body");
    body.append_child(&app_container)
        .expect_throw("Couldn't add app container to body");
}

/// Get the HTML of a test.
///
/// # Panics
///
/// This panics if [`setup_test`] wasn't called.
pub fn test_html() -> String {
    find_html_element(APP_CONTAINER_ID).inner_html()
}

/// Find an element by `id`
///
/// # Panics
///
/// This panics if the element is not found, or is not an [`HtmlElement`]
///
/// [`HtmlElement`]: web_sys::HtmlElement
pub fn find_html_element(id: &str) -> web_sys::HtmlElement {
    try_find_html_element(id).expect_throw("Element not found")
}

/// Find an element by `id`
///
/// This returns `None` if the element is not found.
///
/// # Panics
///
/// This panics if the element is found, and is not an [`HtmlElement`]
///
/// [`HtmlElement`]: web_sys::HtmlElement
pub fn try_find_html_element(id: &str) -> Option<web_sys::HtmlElement> {
    document::query_selector(&format!("#{id}"))
        .expect_throw("Error searching for element")
        .map(|elem| {
            elem.dyn_into()
                .expect_throw("Element was not an `HTMLElement")
        })
}

const APP_CONTAINER_ID: &str = "silkenweb-test-mount-point";
