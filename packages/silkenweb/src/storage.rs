//! Local and session storage.
use silkenweb_base::window;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

macro_rules! unexpected_exception {
    ($name:literal) => {
        concat!("`Storage::", $name, "`shouldn't throw")
    };
}

// TODO: Provide a server side implementation with a trait to define the
// interface

/// Local and session storage.
pub struct Storage(web_sys::Storage);

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
        Ok(Self(window::local_storage()?))
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
        Ok(Self(window::session_storage()?))
    }

    /// Get the value associated with the key.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    pub fn get(&self, key: &str) -> Option<String> {
        self.0
            .get_item(key)
            .expect_throw(unexpected_exception!("getItem"))
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
        self.0.set_item(key, value)
    }

    /// Remove a key/value pair.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    pub fn remove(&self, key: &str) {
        self.0
            .remove_item(key)
            .expect_throw(unexpected_exception!("removeItem"))
    }

    /// Clear the storage.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    pub fn clear(&self) {
        self.0.clear().expect_throw(unexpected_exception!("clear"))
    }

    /// The number of stored keys.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    pub fn len(&self) -> u32 {
        self.0
            .length()
            .expect_throw(unexpected_exception!("length"))
    }

    /// Is the storage empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all the stored keys.
    pub fn keys(&self) -> impl Iterator<Item = String> {
        StorageIter {
            container: self.0.clone(),
            index: 0,
        }
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
