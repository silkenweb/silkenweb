use std::{cell::Cell, rc::Rc};

use futures_signals::signal_vec::{MutableVec, SignalVec};
use serde::{Deserialize, Serialize};
use silkenweb::Storage;

use crate::item_model::TodoItem;

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
                serde_json::from_str(&app_str).unwrap()
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
                .insert(STORAGE_KEY, &serde_json::to_string(self).unwrap())
                .expect("Out of space");
        }
    }

    pub fn new_todo(&self, text: String) {
        let todo_id = self.todo_id.get();
        self.todo_id.replace(todo_id + 1);

        self.items
            .lock_mut()
            .push_cloned(TodoItem::new(todo_id, text));
        self.save();
    }

    pub fn set_completed_states(&self, completed: bool) {
        for item in self.items.lock_ref().iter() {
            item.set_completed(completed);
        }

        self.save();
    }

    pub fn set_completed(&self, item: &TodoItem, completed: bool) {
        item.set_completed(completed);
        self.save();
    }

    pub fn clear_completed_todos(&self) {
        self.items.lock_mut().retain(|item| !item.is_completed());
        self.save();
    }

    pub fn remove_item(&self, todo_id: u128) {
        self.items.lock_mut().retain(|item| item.id() != todo_id);
        self.save();
    }

    pub fn items_signal(&self) -> impl 'static + SignalVec<Item = Rc<TodoItem>> {
        self.items.signal_vec_cloned()
    }
}

const STORAGE_KEY: &str = "silkenweb-examples-todomvc";

#[derive(Display, Copy, Clone, Eq, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}
