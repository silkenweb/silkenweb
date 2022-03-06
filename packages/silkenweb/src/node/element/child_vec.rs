use std::{
    cell::{RefCell, RefMut},
    mem,
    rc::Rc,
};

use futures_signals::signal_vec::VecDiff;

use crate::node::{private::ElementImpl, Node, NodeImpl};

pub struct ChildVec<Impl: NodeImpl> {
    parent: Rc<RefCell<Impl::Element>>,
    children: Vec<Node<Impl>>,
    has_preceding_children: bool,
}

impl<Impl: NodeImpl> ChildVec<Impl> {
    pub fn new(parent: Rc<RefCell<Impl::Element>>, has_preceding_children: bool) -> Self {
        Self {
            parent,
            children: Vec::new(),
            has_preceding_children,
        }
    }

    pub fn apply_update(&mut self, update: VecDiff<impl Into<Node<Impl>>>) {
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

    pub fn replace(&mut self, new_children: Vec<impl Into<Node<Impl>>>) {
        self.clear();
        self.children = new_children
            .into_iter()
            .map(Into::<Node<Impl>>::into)
            .collect();

        for child in &self.children {
            self.parent.borrow_mut().append_child(child);
        }
    }

    pub fn insert(&mut self, index: usize, new_child: impl Into<Node<Impl>>) {
        let new_child = new_child.into();

        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        assert!(index < self.children.len());

        self.parent
            .borrow_mut()
            .insert_child_before(&new_child, Some(&self.children[index]));

        self.children.insert(index, new_child);
    }

    pub fn set_at(&mut self, index: usize, new_child: impl Into<Node<Impl>>) {
        let new_child = new_child.into();
        let old_child = &mut self.children[index];

        self.parent
            .borrow_mut()
            .replace_child(&new_child, &old_child);

        *old_child = new_child;
    }

    pub fn remove(&mut self, index: usize) -> Node<Impl> {
        let old_child = self.children.remove(index);
        self.parent_mut().remove_child(&old_child);

        old_child
    }

    pub fn relocate(&mut self, old_index: usize, new_index: usize) {
        let child = self.remove(old_index);
        self.insert(new_index, child);
    }

    pub fn push(&mut self, new_child: impl Into<Node<Impl>>) {
        let new_child = new_child.into();
        self.parent_mut().append_child(&new_child);
        self.children.push(new_child);
    }

    pub fn pop(&mut self) {
        let removed_child = self.children.pop();

        if let Some(removed_child) = removed_child {
            self.parent_mut().remove_child(&removed_child);
        }
    }

    pub fn clear(&mut self) {
        if self.has_preceding_children {
            let children = mem::take(&mut self.children);

            for child in children {
                self.parent_mut().remove_child(&child);
            }
        } else {
            self.children.clear();
            self.parent_mut().clear_children();
        }
    }

    fn parent_mut(&mut self) -> RefMut<Impl::Element> {
        self.parent.borrow_mut()
    }
}
