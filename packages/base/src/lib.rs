use wasm_bindgen::UnwrapThrowExt;

pub mod macros;

pub mod window {
    use js_sys::Function;
    use wasm_bindgen::{JsValue, UnwrapThrowExt};

    use super::WINDOW;

    pub fn request_animation_frame(callback: &::js_sys::Function) {
        WINDOW.with(|win| win.request_animation_frame(callback).unwrap_throw());
    }

    pub fn history() -> web_sys::History {
        WINDOW.with(|win| win.history().unwrap_throw())
    }

    pub fn location() -> web_sys::Location {
        WINDOW.with(|win| win.location())
    }

    pub fn set_onpopstate(value: Option<&Function>) {
        WINDOW.with(|win| win.set_onpopstate(value));
    }

    pub fn local_storage() -> Result<web_sys::Storage, JsValue> {
        WINDOW.with(|w| w.local_storage().map(|w| w.unwrap_throw()))
    }

    pub fn session_storage() -> Result<web_sys::Storage, JsValue> {
        WINDOW.with(|w| w.session_storage().map(|w| w.unwrap_throw()))
    }
}

pub mod document {
    use wasm_bindgen::UnwrapThrowExt;

    use super::DOCUMENT;

    pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
        DOCUMENT.with(|doc| doc.get_element_by_id(id))
    }

    pub fn create_element(tag: &str) -> web_sys::Element {
        DOCUMENT.with(|doc| doc.create_element(tag).unwrap_throw())
    }

    pub fn create_element_ns(namespace: Option<&str>, tag: &str) -> web_sys::Element {
        DOCUMENT.with(|doc| doc.create_element_ns(namespace, tag).unwrap_throw())
    }

    pub fn create_text_node(text: &str) -> web_sys::Text {
        DOCUMENT.with(|doc| doc.create_text_node(text))
    }

    pub fn base_uri() -> String {
        DOCUMENT
            .with(|doc| doc.base_uri())
            .unwrap_throw()
            .unwrap_throw()
    }
}

#[cfg(target_arch = "wasm32")]
pub fn intern_str(s: &str) -> &str {
    use wasm_bindgen::intern;
    intern(s)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn intern_str(s: &str) -> &str {
    s
}

thread_local!(
    static WINDOW: web_sys::Window = web_sys::window().expect_throw("Window must be available");
    static DOCUMENT: web_sys::Document = WINDOW.with(|win| {
        win.document()
            .expect_throw("Window must contain a document")
    });
);
