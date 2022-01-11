use wasm_bindgen::UnwrapThrowExt;
use web_sys::Node;

use crate::{clone, render::queue_update};

pub fn insert_child_before(parent: &Node, new_child: &Node, next_child: &Node) {
    clone!(parent, new_child, next_child);

    queue_update(move || {
        parent
            .insert_before(&new_child, Some(&next_child))
            .unwrap_throw();
    });
}

pub fn append_child(parent: &Node, child: &Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.append_child(&child).unwrap_throw();
    });
}

pub fn replace_child(parent: &Node, new_child: &Node, old_child: &Node) {
    clone!(parent, new_child, old_child);

    queue_update(move || {
        parent.replace_child(&new_child, &old_child).unwrap_throw();
    });
}

pub fn remove_child(parent: &Node, child: &Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.remove_child(&child).unwrap_throw();
    });
}
