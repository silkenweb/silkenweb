use wasm_bindgen::JsValue;
use web_sys as dom;

use crate::window;

macro_rules! unexpected_exception {
    ($name:literal) => {
        concat!("`Storage::", $name, "`shouldn't throw")
    };
}

pub struct Storage(dom::Storage);

impl Storage {
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    ///
    /// # Errors
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    /// for when an error will be returned.
    /// The error value is unspecified and will depend on the browser.
    pub fn local() -> Result<Self, JsValue> {
        Ok(Self(window().local_storage()?.unwrap()))
    }

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    ///
    /// # Errors
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    /// for when an error will be returned.
    /// The error value is unspecified and will depend on the browser.
    pub fn session() -> Result<Self, JsValue> {
        Ok(Self(window().session_storage()?.unwrap()))
    }

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    pub fn get(&self, key: &str) -> Option<String> {
        self.0
            .get_item(key)
            .expect(unexpected_exception!("getItem"))
    }

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)
    ///
    /// # Errors
    ///
    /// If the storage is full, an error is returned.
    /// The error value is unspecified and will depend on the browser.
    pub fn insert(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.0.set_item(key, value)
    }

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    pub fn remove(&self, key: &str) {
        self.0
            .remove_item(key)
            .expect(unexpected_exception!("removeItem"))
    }

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    pub fn clear(&self) {
        self.0.clear().expect(unexpected_exception!("clear"))
    }

    pub fn len(&self) -> u32 {
        self.0.length().expect(unexpected_exception!("length"))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn keys(&self) -> impl Iterator<Item = String> {
        StorageIter {
            container: self.0.clone(),
            index: 0,
        }
    }
}

struct StorageIter {
    container: dom::Storage,
    index: u32,
}

impl Iterator for StorageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .container
            .key(self.index)
            .expect(unexpected_exception!("key"));
        self.index += 1;
        item
    }
}
