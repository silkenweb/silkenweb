pub mod list_state;
pub mod memo;
pub mod state;

use std::cell::RefCell;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::window;

// TODO: This shouldn't be public
pub fn queue_update(x: impl 'static + FnOnce()) {
    let len = {
        PENDING_UPDATES.with(|update_queue| {
            let mut update_queue = update_queue.borrow_mut();

            update_queue.push(Box::new(x));
            update_queue.len()
        })
    };

    if len == 1 {
        request_process_updates();
    }
}

fn queue_effect(x: impl 'static + FnOnce()) {
    PENDING_EFFECTS.with(|pending_effects| pending_effects.borrow_mut().push(Box::new(x)));
}

fn request_process_updates() {
    ON_ANIMATION_FRAME.with(|process_updates| {
        window()
            .request_animation_frame(process_updates.as_ref().unchecked_ref())
            .unwrap()
    });
}

fn process_updates() {
    PENDING_UPDATES.with(|update_queue| {
        let update_queue = update_queue.take();

        // TODO: Can we reinstate this? Queue updates from element updater?
        // if update_queue.len() != 1 {
        //     let mut updates_by_depth: Vec<_> = update_queue
        //         .into_iter()
        //         .filter_map(|u| u.parent().upgrade().map(|p| (p.borrow().dom_depth(),
        // u)))         .collect();

        //     updates_by_depth.sort_unstable_by_key(|(key, _)| *key);

        //     update_queue = updates_by_depth
        //         .into_iter()
        //         .map(|(_, value)| value)
        //         .collect();
        // }

        for update in update_queue {
            update();
        }
    });

    PENDING_EFFECTS.with(|effect_queue| {
        for effect in effect_queue.take() {
            effect();
        }
    });
}

thread_local!(
    static PENDING_UPDATES: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(Vec::new());
    static PENDING_EFFECTS: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(Vec::new());
    static ON_ANIMATION_FRAME: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_time_stamp: JsValue| {
            process_updates();
        }));
);
