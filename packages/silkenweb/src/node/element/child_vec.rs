use std::mem;

use futures_signals::signal_vec::VecDiff;

use crate::{
    dom::{Dom, DomElement},
    node::Node,
};

pub struct ChildVec<D: Dom> {
    parent: D::Element,
    children: Vec<Node<D>>,
    has_preceding_children: bool,
}

impl<D: Dom> ChildVec<D> {
    pub fn new(parent: D::Element, has_preceding_children: bool) -> Self {
        Self {
            parent,
            children: Vec::new(),
            has_preceding_children,
        }
    }

    pub fn apply_update(&mut self, update: VecDiff<impl Into<Node<D>>>) {
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

    pub fn replace(&mut self, new_children: Vec<impl Into<Node<D>>>) {
        self.clear();
        self.children = new_children
            .into_iter()
            .map(Into::<Node<D>>::into)
            .collect();

        let mut parent = self.parent.clone();

        for child in &self.children {
            parent.append_child(child.as_node());
        }
    }

    pub fn insert(&mut self, index: usize, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();

        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        assert!(index < self.children.len());

        self.parent
            .insert_child_before(new_child.as_node(), Some(self.children[index].as_node()));

        self.children.insert(index, new_child);
    }

    pub fn set_at(&mut self, index: usize, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();
        let old_child = &mut self.children[index];

        self.parent
            .replace_child(new_child.as_node(), old_child.as_node());

        *old_child = new_child;
    }

    pub fn remove(&mut self, index: usize) -> Node<D> {
        let old_child = self.children.remove(index);
        self.parent.remove_child(old_child.as_node());

        old_child
    }

    pub fn relocate(&mut self, old_index: usize, new_index: usize) {
        let child = self.remove(old_index);
        self.insert(new_index, child);
    }

    pub fn push(&mut self, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();
        self.parent.append_child(new_child.as_node());
        self.children.push(new_child);
    }

    pub fn pop(&mut self) {
        let removed_child = self.children.pop();

        if let Some(removed_child) = removed_child {
            self.parent.remove_child(removed_child.as_node());
        }
    }

    pub fn clear(&mut self) {
        if self.has_preceding_children {
            let mut parent = self.parent.clone();
            let children = mem::take(&mut self.children);

            for child in children {
                parent.remove_child(child.as_node());
            }
        } else {
            self.children.clear();
            self.parent.clear_children();
        }
    }
}
