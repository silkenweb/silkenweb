use wasm_bindgen::UnwrapThrowExt;
use web_sys as dom;

pub mod window {
    use js_sys::Function;
    use wasm_bindgen::{JsValue, UnwrapThrowExt};
    use web_sys as dom;

    use super::WINDOW;

    pub fn request_animation_frame(callback: &::js_sys::Function) {
        WINDOW.with(|win| win.request_animation_frame(callback).unwrap_throw());
    }

    pub fn history() -> dom::History {
        WINDOW.with(|win| win.history().unwrap_throw())
    }

    pub fn location() -> dom::Location {
        WINDOW.with(|win| win.location())
    }

    pub fn set_onpopstate(value: Option<&Function>) {
        WINDOW.with(|win| win.set_onpopstate(value));
    }

    pub fn local_storage() -> Result<dom::Storage, JsValue> {
        WINDOW.with(|w| w.local_storage().map(|w| w.unwrap_throw()))
    }

    pub fn session_storage() -> Result<dom::Storage, JsValue> {
        WINDOW.with(|w| w.session_storage().map(|w| w.unwrap_throw()))
    }
}

pub mod document {
    use wasm_bindgen::UnwrapThrowExt;
    use web_sys as dom;

    use super::DOCUMENT;

    pub fn get_element_by_id(id: &str) -> Option<dom::Element> {
        DOCUMENT.with(|doc| doc.get_element_by_id(id))
    }

    pub fn create_element(tag: &str) -> dom::Element {
        DOCUMENT.with(|doc| doc.create_element(tag).unwrap_throw())
    }

    pub fn create_element_ns(namespace: &str, tag: &str) -> dom::Element {
        DOCUMENT.with(|doc| doc.create_element_ns(Some(namespace), tag).unwrap_throw())
    }

    pub fn create_text_node(text: &str) -> dom::Text {
        DOCUMENT.with(|doc| doc.create_text_node(text))
    }
}

thread_local!(
    static WINDOW: dom::Window = dom::window().expect_throw("Window must be available");
    static DOCUMENT: dom::Document = WINDOW.with(|win| {
        win.document()
            .expect_throw("Window must contain a document")
    });
);
