use std::mem;

use web_sys as dom;

use super::dom_children::{append_child, insert_child_before, remove_child};

/// Groups of children with the same parent
///
/// This manages insertion and removal of groups of children
pub struct ChildGroups {
    parent: dom::Element,
    // The stack size of `BTreeMap` is the same as `Vec`, but it allocs 192 bytes on the first
    // insert and cannot be shrunk to fit.
    children: Vec<Option<dom::Node>>,
}

impl ChildGroups {
    pub fn new(parent: dom::Element) -> Self {
        Self {
            parent,
            children: Vec::new(),
        }
    }

    pub fn new_group(&mut self) -> usize {
        let index = self.children.len();
        self.children.push(None);
        index
    }

    pub fn get_next_group_elem(&self, index: usize) -> Option<&dom::Node> {
        self.children.split_at(index + 1).1.iter().flatten().next()
    }

    pub fn append_new_group(&mut self, child: &dom::Node) {
        let group_index = self.new_group();
        self.insert_only_child(group_index, child);
    }

    pub fn insert_only_child(&mut self, index: usize, child: &dom::Node) {
        assert!(!self.upsert_only_child(index, child));
    }

    /// Return `true` iff there was an existing node.
    pub fn upsert_only_child(&mut self, index: usize, child: &dom::Node) -> bool {
        let existed = mem::replace(&mut self.children[index], Some(child.clone()))
            .map(|existing| remove_child(&self.parent, &existing))
            .is_some();

        self.insert_last_child(index, child);

        existed
    }

    pub fn insert_last_child(&self, index: usize, child: &dom::Node) {
        match self.get_next_group_elem(index) {
            Some(next_child) => insert_child_before(&self.parent, child, next_child),
            None => append_child(&self.parent, child),
        }
    }

    pub fn set_first_child(&mut self, index: usize, child: &dom::Node) {
        self.children[index] = Some(child.clone());
    }

    pub fn remove_child(&mut self, index: usize) {
        if let Some(existing) = mem::replace(&mut self.children[index], None) {
            remove_child(&self.parent, &existing);
        }
    }

    pub fn clear_first_child(&mut self, index: usize) {
        self.children[index] = None;
    }

    pub fn shrink_to_fit(&mut self) {
        self.children.shrink_to_fit();
    }
}
