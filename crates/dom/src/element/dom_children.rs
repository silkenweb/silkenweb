use web_sys as dom;

use crate::{clone, render::queue_update};

pub fn insert_child_before(parent: &dom::Node, new_child: &dom::Node, next_child: &dom::Node) {
    clone!(parent, new_child, next_child);

    queue_update(move || {
        parent.insert_before(&new_child, Some(&next_child)).unwrap();
    });
}

pub fn append_child(parent: &dom::Node, child: &dom::Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.append_child(&child).unwrap();
    });
}

pub fn replace_child(parent: &dom::Node, new_child: &dom::Node, old_child: &dom::Node) {
    clone!(parent, new_child, old_child);

    queue_update(move || {
        parent.replace_child(&new_child, &old_child).unwrap();
    });
}

pub fn remove_child(parent: &dom::Node, child: &dom::Node) {
    clone!(parent, child);

    queue_update(move || {
        parent.remove_child(&child).unwrap();
    });
}