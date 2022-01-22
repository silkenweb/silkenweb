use std::{cell::RefCell, mem, rc::Rc};

use futures_signals::signal_vec::VecDiff;
use wasm_bindgen::UnwrapThrowExt;

use super::{child_groups::ChildGroups, strict::StrictNodeBase, Element};
use crate::{element::strict::StrictNodeRef, render::queue_update};

pub struct ChildVec {
    parent: StrictNodeBase,
    child_groups: Rc<RefCell<ChildGroups>>,
    group_index: usize,
    children: Vec<Element>,
}

impl ChildVec {
    pub fn new(
        parent: StrictNodeBase,
        child_groups: Rc<RefCell<ChildGroups>>,
        group_index: usize,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            child_groups,
            group_index,
            children: Vec::new(),
        }))
    }

    pub fn apply_update(&mut self, update: VecDiff<impl Into<Element>>) {
        match update {
            VecDiff::Replace { values } => self.replace(values),
            VecDiff::InsertAt { index, value } => self.insert(index, value),
            VecDiff::UpdateAt { index, value } => self.set_at(index, value),
            VecDiff::RemoveAt { index } => {
                self.remove(index);
            }
            VecDiff::Move {
                old_index,
                new_index,
            } => self.relocate(old_index, new_index),
            VecDiff::Push { value } => self.push(value),
            VecDiff::Pop {} => self.pop(),
            VecDiff::Clear {} => self.clear(),
        }
    }

    pub fn replace(&mut self, new_children: Vec<impl Into<Element>>) {
        self.clear();
        self.children = new_children
            .into_iter()
            .map(Into::<Element>::into)
            .collect();

        let mut child_groups = self.child_groups.borrow_mut();

        if self.children.is_empty() {
            child_groups.clear_first_child(self.group_index);
            return;
        }

        let children = self.child_element_refs();
        child_groups.set_first_child(self.group_index, children.first().unwrap_throw().clone());
        let parent = self.parent.clone();
        let next_group_elem = child_groups.get_next_group_elem(self.group_index).cloned();

        queue_update(move || {
            for child in children {
                parent.insert_child_before_now(&child, next_group_elem.as_ref());
            }
        });
    }

    pub fn insert(&mut self, index: usize, new_child: impl Into<Element>) {
        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        let new_child = new_child.into();
        let new_child_node = new_child.0.clone_into_node();

        if index == 0 {
            self.child_groups
                .borrow_mut()
                .set_first_child(self.group_index, new_child_node.clone());
        }

        assert!(index < self.children.len());

        self.parent.insert_child_before(
            new_child_node,
            Some(self.children[index].0.clone_into_node()),
        );

        self.children.insert(index, new_child);
    }

    pub fn set_at(&mut self, index: usize, new_child: impl Into<Element>) {
        let new_child = new_child.into();
        let new_child_node = new_child.0.clone_into_node();

        if index == 0 {
            self.child_groups
                .borrow_mut()
                .set_first_child(self.group_index, new_child_node.clone());
        }

        let old_child = &mut self.children[index];

        self.parent
            .replace_child(new_child_node, old_child.0.clone_into_node());

        *old_child = new_child;
    }

    pub fn remove(&mut self, index: usize) -> Element {
        let old_child = self.children.remove(index);
        self.parent.remove_child(old_child.0.clone_into_node());

        let mut child_groups = self.child_groups.borrow_mut();

        match self.children.first() {
            None => child_groups.clear_first_child(self.group_index),
            Some(first) => {
                if index == 0 {
                    child_groups.set_first_child(self.group_index, first.0.clone_into_node())
                }
            }
        }

        old_child
    }

    pub fn relocate(&mut self, old_index: usize, new_index: usize) {
        let child = self.remove(old_index);
        self.insert(new_index, child);
    }

    pub fn push(&mut self, new_child: impl Into<Element>) {
        let new_child = new_child.into();
        let new_child_node = new_child.0.clone_into_node();
        let mut groups = self.child_groups.borrow_mut();

        if self.children.is_empty() {
            groups.insert_only_child(self.group_index, new_child_node);
        } else {
            groups.insert_last_child(self.group_index, new_child_node);
        }

        self.children.push(new_child);
    }

    pub fn pop(&mut self) {
        let removed_child = self.children.pop();

        if self.children.is_empty() {
            self.child_groups
                .borrow_mut()
                .clear_first_child(self.group_index);
        }

        if let Some(removed_child) = removed_child {
            self.parent.remove_child(removed_child.0.clone_into_node());
        }
    }

    pub fn clear(&mut self) {
        let existing_children = self.child_element_refs();
        self.children.clear();
        let mut child_groups = self.child_groups.borrow_mut();

        child_groups.clear_first_child(self.group_index);
        let is_only_group = child_groups.is_single_group();
        mem::drop(child_groups);

        if is_only_group {
            self.parent.clear_children();
        } else {
            let parent = self.parent.clone();

            queue_update(move || {
                for child in existing_children {
                    parent.remove_child_now(&child);
                }
            });
        }
    }

    fn child_element_refs(&self) -> Vec<StrictNodeBase> {
        self.children
            .iter()
            .map(|elem| elem.0.clone_into_node())
            .collect()
    }
}
