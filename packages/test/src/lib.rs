//! Utilities for writing tests for Silkenweb apps.
use silkenweb::{dom::DefaultDom, render::DocumentRender, task::render_now};
use silkenweb_base::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

/// Setup a browser test.
///
/// This will cleanup the test when `drop`ed.
#[must_use]
pub struct BrowserTest;

impl BrowserTest {
    /// Setup a test.
    ///
    /// Actions:
    ///
    /// - Clear the render queue.
    /// - Check this isn't a nested browser test.
    /// - Unmount any mounted elements.
    /// - Create a new mount point with `id` = `mount_point_id`.
    pub async fn new(mount_point_id: &str) -> Self {
        // Clear the render queue
        render_now().await;

        if try_html_element(APP_CONTAINER_ID).is_some() {
            panic!("`Test` cannot be nested")
        }

        DefaultDom::unmount_all();

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

        Self
    }

    /// Get the HTML of a test.
    pub fn html(&self) -> String {
        html_element(APP_CONTAINER_ID).inner_html()
    }
}

impl Drop for BrowserTest {
    fn drop(&mut self) {
        DefaultDom::unmount_all();
        html_element(APP_CONTAINER_ID).remove();
    }
}

/// Find an element by `id`
///
/// # Panics
///
/// This panics if the element is not found, or is not an [`HtmlElement`]
///
/// [`HtmlElement`]: web_sys::HtmlElement
pub fn html_element(id: &str) -> web_sys::HtmlElement {
    try_html_element(id).expect_throw("Element not found")
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
pub fn try_html_element(id: &str) -> Option<web_sys::HtmlElement> {
    document::query_selector(&format!("#{id}"))
        .expect_throw("Error searching for element")
        .map(|elem| {
            elem.dyn_into()
                .expect_throw("Element was not an `HTMLElement")
        })
}

const APP_CONTAINER_ID: &str = "silkenweb-test-mount-point";
