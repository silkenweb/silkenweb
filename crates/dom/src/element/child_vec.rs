use std::{cell::RefCell, rc::Rc};

use futures_signals::signal_vec::VecDiff;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as dom;

use super::{
    child_groups::ChildGroups,
    dom_children::{remove_child, replace_child},
    Element,
};
use crate::{
    element::{dom_children::insert_child_before, DomElement},
    render::queue_update,
};

pub struct ChildVec {
    parent: dom::Element,
    child_groups: Rc<RefCell<ChildGroups>>,
    group_index: usize,
    children: Vec<Element>,
}

impl ChildVec {
    pub fn new(
        parent: dom::Element,
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

        let children = self.child_dom_elements();
        child_groups.set_first_child(self.group_index, children.first().unwrap_throw());
        let parent = self.parent.clone();
        let next_group_elem = child_groups.get_next_group_elem(self.group_index).cloned();

        queue_update(parent.is_connected(), move || {
            for child in children {
                parent
                    .insert_before(&child, next_group_elem.as_ref())
                    .unwrap_throw();
            }
        });
    }

    pub fn insert(&mut self, index: usize, new_child: impl Into<Element>) {
        if index >= self.children.len() {
            self.push(new_child);
            return;
        }

        let new_child = new_child.into();
        let new_dom_elem = new_child.dom_element();

        if index == 0 {
            self.child_groups
                .borrow_mut()
                .set_first_child(self.group_index, new_dom_elem);
        }

        assert!(index < self.children.len());

        insert_child_before(
            &self.parent,
            new_dom_elem,
            self.children[index].dom_element(),
        );

        self.children.insert(index, new_child);
    }

    pub fn set_at(&mut self, index: usize, new_child: impl Into<Element>) {
        let new_child = new_child.into();

        if index == 0 {
            self.child_groups
                .borrow_mut()
                .set_first_child(self.group_index, new_child.dom_element());
        }

        let old_child = &mut self.children[index];

        replace_child(
            &self.parent,
            new_child.dom_element(),
            old_child.dom_element(),
        );

        *old_child = new_child;
    }

    pub fn remove(&mut self, index: usize) -> Element {
        let old_child = self.children.remove(index);
        remove_child(&self.parent, old_child.dom_element());

        let mut child_groups = self.child_groups.borrow_mut();

        match self.children.first() {
            None => child_groups.clear_first_child(self.group_index),
            Some(first) => {
                if index == 0 {
                    child_groups.set_first_child(self.group_index, first.dom_element())
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
        let new_child_dom = new_child.dom_element();
        let mut groups = self.child_groups.borrow_mut();

        if self.children.is_empty() {
            groups.insert_only_child(self.group_index, new_child_dom);
        } else {
            groups.insert_last_child(self.group_index, new_child_dom);
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
            remove_child(&self.parent, removed_child.dom_element());
        }
    }

    pub fn clear(&mut self) {
        let existing_children = self.child_dom_elements();
        self.children.clear();
        let parent = self.parent.clone();

        queue_update(parent.is_connected(), move || {
            for child in existing_children {
                parent.remove_child(&child).unwrap_throw();
            }
        });

        self.child_groups
            .borrow_mut()
            .clear_first_child(self.group_index);
    }

    fn child_dom_elements(&self) -> Vec<dom::Element> {
        self.children
            .iter()
            .map(Element::dom_element)
            .cloned()
            .collect()
    }
}
