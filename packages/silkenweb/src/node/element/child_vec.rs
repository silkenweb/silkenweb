use std::{cell::RefCell, marker::PhantomData, mem, rc::Rc};

use clonelet::clone;
use discard::DiscardOnDrop;
use futures_signals::{
    signal_vec::{SignalVec, SignalVecExt, VecDiff},
    CancelableFutureHandle,
};

use super::spawn_cancelable_future;
use crate::{
    dom::{private::DomElement, Dom},
    node::Node,
};

pub struct ChildVec<D: Dom, PO> {
    parent: D::Element,
    children: Vec<Node<D>>,
    static_child_count: usize,
    phantom: PhantomData<PO>,
}

#[must_use]
pub struct ChildVecHandle<D: Dom, PO> {
    child_vec: Rc<RefCell<ChildVec<D, PO>>>,
    _future_handle: DiscardOnDrop<CancelableFutureHandle>,
}

impl<D, PO> ChildVecHandle<D, PO>
where
    D: Dom,
    ChildVec<D, PO>: ParentOwner,
{
    pub fn inner_html(&self) -> String {
        let mut html = String::new();

        for elem in self.child_vec.borrow().children.iter() {
            html.push_str(&elem.to_string());
        }

        html
    }

    pub fn clear(self) {
        self.child_vec.borrow_mut().clear();
    }
}

pub trait ParentOwner {
    fn clear(&mut self);
}

pub struct ParentUnique;

pub struct ParentShared;

impl<D: Dom, PO> ChildVec<D, PO>
where
    PO: 'static,
    Self: ParentOwner,
{
    pub fn new(parent: D::Element, static_child_count: usize) -> Self {
        Self {
            parent,
            children: Vec::new(),
            static_child_count,
            phantom: PhantomData,
        }
    }

    pub fn run(self, children: impl SignalVec<Item = Node<D>> + 'static) -> ChildVecHandle<D, PO> {
        let child_vec = Rc::new(RefCell::new(self));
        let future = children.for_each({
            clone!(child_vec);
            move |update| {
                child_vec.borrow_mut().apply_update(update);
                async {}
            }
        });

        // `future` may finish if, for example, a `MutableVec` is dropped. So we need to
        // keep a hold of `child_vec`, as it may own signals that need updating.
        ChildVecHandle {
            child_vec,
            _future_handle: spawn_cancelable_future(future),
        }
    }

    fn apply_update(&mut self, update: VecDiff<impl Into<Node<D>>>) {
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

        clone!(mut self.parent);

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

    fn elementwise_clear(&mut self) {
        let children = mem::take(&mut self.children);

        for (index, child) in children.into_iter().enumerate().rev() {
            self.parent
                .remove_child(index + self.static_child_count, &child.node);
        }
    }
}

impl<D: Dom> ParentOwner for ChildVec<D, ParentUnique> {
    fn clear(&mut self) {
        if self.static_child_count > 0 {
            self.elementwise_clear()
        } else {
            self.children.clear();
            self.parent.clear_children();
        }
    }
}

impl<D: Dom> ParentOwner for ChildVec<D, ParentShared> {
    fn clear(&mut self) {
        self.elementwise_clear()
    }
}
