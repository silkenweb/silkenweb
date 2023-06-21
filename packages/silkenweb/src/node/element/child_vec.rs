use std::mem;

use futures_signals::signal_vec::VecDiff;

use crate::{
    dom::{private::DomElement, Dom},
    node::Node,
};

pub struct ChildVec<D: Dom> {
    parent: D::Element,
    children: Vec<Node<D>>,
    static_child_count: usize,
}

impl<D: Dom> ChildVec<D> {
    pub fn new(parent: D::Element, static_child_count: usize) -> Self {
        Self {
            parent,
            children: Vec::new(),
            static_child_count,
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

    fn replace(&mut self, new_children: Vec<impl Into<Node<D>>>) {
        self.clear();
        self.children = new_children
            .into_iter()
            .map(Into::<Node<D>>::into)
            .collect();

        let mut parent = self.parent.clone();

        for child in &self.children {
            parent.append_child(&child.node);
        }
    }

    fn insert(&mut self, index: usize, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();

        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        assert!(index < self.children.len());

        self.parent.insert_child_before(
            index + self.static_child_count,
            &new_child.node,
            Some(&self.children[index].node),
        );

        self.children.insert(index, new_child);
    }

    fn set_at(&mut self, index: usize, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();
        let old_child = &mut self.children[index];

        self.parent.replace_child(
            index + self.static_child_count,
            &new_child.node,
            &old_child.node,
        );

        *old_child = new_child;
    }

    fn remove(&mut self, index: usize) -> Node<D> {
        let old_child = self.children.remove(index);
        self.parent
            .remove_child(index + self.static_child_count, &old_child.node);

        old_child
    }

    fn relocate(&mut self, old_index: usize, new_index: usize) {
        let child = self.remove(old_index);
        self.insert(new_index, child);
    }

    fn push(&mut self, new_child: impl Into<Node<D>>) {
        let new_child = new_child.into();
        self.parent.append_child(&new_child.node);
        self.children.push(new_child);
    }

    fn pop(&mut self) {
        let removed_child = self.children.pop();

        if let Some(removed_child) = removed_child {
            self.parent.remove_child(
                self.children.len() + self.static_child_count,
                &removed_child.node,
            );
        }
    }

    fn clear(&mut self) {
        if self.static_child_count > 0 {
            let mut parent = self.parent.clone();
            let children = mem::take(&mut self.children);

            for (index, child) in children.into_iter().enumerate().rev() {
                parent.remove_child(index + self.static_child_count, &child.node);
            }
        } else {
            self.children.clear();
            self.parent.clear_children();
        }
    }
}
