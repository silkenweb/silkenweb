use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};
use serde::{Deserialize, Serialize};

use crate::app_model::TodoApp;

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    id: u128,
    text: Mutable<String>,
    completed: Mutable<bool>,
    #[serde(skip)]
    editing: Mutable<bool>,
}

impl TodoItem {
    pub fn new(id: u128, text: String) -> Rc<Self> {
        Rc::new(Self {
            id,
            text: Mutable::new(text),
            completed: Mutable::new(false),
            editing: Mutable::new(false),
        })
    }

    pub fn id(&self) -> u128 {
        self.id
    }

    pub fn is_completed(&self) -> bool {
        self.completed.get()
    }

    pub fn set_editing(&self) {
        self.editing.set(true);
    }

    // TODO: This is public so app can set it and then save. It shouldn't be public
    // for everyone.
    pub fn set_completed(&self, completed: bool) {
        self.completed.set_neq(completed);
    }

    pub fn save_edits(&self, app: &TodoApp, text: String) {
        if !self.editing.get() {
            return;
        }

        let text = text.trim();

        if text.is_empty() {
            self.remove(app);
        } else {
            self.text.set(text.to_string());
            self.editing.set(false);
        }

        app.save();
    }

    pub fn revert_edits(&self) -> String {
        self.editing.set(false);
        self.text.get_cloned()
    }

    pub fn remove(&self, app: &TodoApp) {
        app.remove_item(self.id)
    }

    pub fn text(&self) -> impl Signal<Item = String> {
        self.text.signal_cloned()
    }

    pub fn completed(&self) -> impl Signal<Item = bool> {
        self.completed.signal()
    }

    pub fn is_editing(&self) -> impl Signal<Item = bool> {
        self.editing.signal()
    }
}
