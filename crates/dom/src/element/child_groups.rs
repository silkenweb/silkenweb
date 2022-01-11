use std::mem;

use wasm_bindgen::UnwrapThrowExt;

use super::dom_children::{append_child, insert_child_before, remove_child};

/// Groups of children with the same parent
///
/// This manages insertion and removal of groups of children
pub struct ChildGroups {
    parent: web_sys::Element,
    // The stack size of `BTreeMap` is the same as `Vec`, but it allocs 192 bytes on the first
    // insert and cannot be shrunk to fit.
    children: Vec<Option<web_sys::Node>>,
    // `true` if the last child group can change.
    last_is_dynamic: bool,
    group_count: usize,
}

impl ChildGroups {
    pub fn new(parent: web_sys::Element) -> Self {
        Self {
            parent,
            children: Vec::new(),
            last_is_dynamic: false,
            group_count: 0,
        }
    }

    pub fn is_single_group(&self) -> bool {
        self.group_count == 1
    }

    pub fn new_group(&mut self) -> usize {
        self.group_count += 1;
        self.last_is_dynamic = true;
        let index = self.children.len();
        self.children.push(None);
        index
    }

    pub fn get_next_group_elem(&self, index: usize) -> Option<&web_sys::Node> {
        self.children.split_at(index + 1).1.iter().flatten().next()
    }

    /// Append a new group. Don't wait for the next animation frame.
    pub fn append_new_group_sync(&mut self, child: &web_sys::Node) {
        if self.last_is_dynamic {
            self.children.push(Some(child.clone()));
        }

        self.group_count += 1;
        self.parent.append_child(child).unwrap_throw();
        // We didn't give out an index, so this can't be dynamic.
        self.last_is_dynamic = false;
    }

    pub fn insert_only_child(&mut self, index: usize, child: &web_sys::Node) {
        assert!(!self.upsert_only_child(index, child));
    }

    /// Return `true` iff there was an existing node.
    pub fn upsert_only_child(&mut self, index: usize, child: &web_sys::Node) -> bool {
        let existed = mem::replace(&mut self.children[index], Some(child.clone()))
            .map(|existing| remove_child(&self.parent, &existing))
            .is_some();

        self.insert_last_child(index, child);

        existed
    }

    pub fn insert_last_child(&self, index: usize, child: &web_sys::Node) {
        match self.get_next_group_elem(index) {
            Some(next_child) => insert_child_before(&self.parent, child, next_child),
            None => append_child(&self.parent, child),
        }
    }

    pub fn remove_child(&mut self, index: usize) {
        if let Some(existing) = mem::replace(&mut self.children[index], None) {
            remove_child(&self.parent, &existing);
        }
    }

    pub fn set_first_child(&mut self, index: usize, child: &web_sys::Node) {
        self.children[index] = Some(child.clone());
    }

    pub fn clear_first_child(&mut self, index: usize) {
        self.children[index] = None;
    }

    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
    }
}
