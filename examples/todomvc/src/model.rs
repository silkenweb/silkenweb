use std::{cell::Cell, rc::Rc};

use futures_signals::{
    signal::{Mutable, Signal},
    signal_vec::{MutableVec, SignalVec},
};
use serde::{Deserialize, Serialize};
use silkenweb::Storage;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Serialize, Deserialize)]
pub struct TodoApp {
    todo_id: Cell<u128>,
    items: MutableVec<Rc<TodoItem>>,
}

impl TodoApp {
    pub fn load() -> Rc<Self> {
        Rc::new(
            if let Some(app_str) = Storage::local()
                .ok()
                .and_then(|storage| storage.get(STORAGE_KEY))
            {
                serde_json::from_str(&app_str).unwrap_throw()
            } else {
                Self {
                    todo_id: Cell::new(0),
                    items: MutableVec::new(),
                }
            },
        )
    }

    pub fn save(&self) {
        if let Ok(storage) = Storage::local() {
            storage
                .insert(STORAGE_KEY, &serde_json::to_string(self).unwrap_throw())
                .expect_throw("Out of space");
        }
    }

    pub fn new_todo(&self, text: String) {
        let todo_id = self.todo_id.get();
        self.todo_id.set(todo_id + 1);

        self.items
            .lock_mut()
            .push_cloned(TodoItem::new(todo_id, text));
        self.save();
    }

    pub fn set_completed_states(&self, completed: bool) {
        for item in self.items.lock_ref().iter() {
            item.completed.set_neq(completed);
        }

        self.save();
    }

    pub fn set_completed(&self, item: &TodoItem, completed: bool) {
        item.completed.set_neq(completed);
        self.save();
    }

    pub fn clear_completed_todos(&self) {
        self.items.lock_mut().retain(|item| !item.completed.get());
        self.save();
    }

    pub fn remove_item(&self, todo_id: u128) {
        self.items.lock_mut().retain(|item| item.id != todo_id);
        self.save();
    }

    pub fn items_signal(&self) -> impl SignalVec<Item = Rc<TodoItem>> + 'static {
        self.items.signal_vec_cloned()
    }
}

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

    pub fn set_editing(&self) {
        self.editing.set(true);
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

const STORAGE_KEY: &str = "silkenweb-examples-todomvc";

#[derive(Display, Copy, Clone, Eq, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}
