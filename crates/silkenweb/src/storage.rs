use std::iter;

use silkenweb_dom::global::window;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

macro_rules! unexpected_exception {
    ($name:literal) => {
        concat!("`Storage::", $name, "`shouldn't throw")
    };
}

pub struct Storage;

impl Storage {
    /// Get the window's local storage.
    ///
    /// [MDN Documentation][mdn]
    ///
    /// # Errors
    ///
    /// The [error value][mdn] is unspecified and will depend on the browser.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
    pub fn local() -> Result<Self, JsValue> {
        Ok(Self)
    }

    /// Get the window's session storage.
    ///
    /// [MDN Documentation][mdn]
    ///
    /// # Errors
    ///
    /// The [error value][mdn] is unspecified and will depend on the browser.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
    pub fn session() -> Result<Self, JsValue> {
        Ok(Self)
    }

    /// Get the value associated with the key.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    pub fn get(&self, key: &str) -> Option<String> {
        None
    }

    /// Set the value associated with the key.
    /// [MDN Documentation][mdn]
    ///
    /// # Errors
    ///
    /// If the storage is full, an error is returned. The [error value][mdn] is
    /// unspecified and will depend on the browser.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem
    pub fn insert(&self, key: &str, value: &str) -> Result<(), JsValue> {
        Ok(())
    }

    /// Remove a key/value pair.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    pub fn remove(&self, key: &str) {
    }

    /// Clear the storage.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    pub fn clear(&self) {
    }

    /// The number of stored keys.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    pub fn len(&self) -> u32 {
        0
    }

    /// Is the storage empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all the stored keys.
    pub fn keys(&self) -> impl Iterator<Item = String> {
        iter::empty()
    }
}

struct StorageIter {
    container: web_sys::Storage,
    index: u32,
}

impl Iterator for StorageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .container
            .key(self.index)
            .expect_throw(unexpected_exception!("key"));
        self.index += 1;
        item
    }
}
