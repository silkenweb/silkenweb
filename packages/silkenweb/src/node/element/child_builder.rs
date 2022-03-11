use std::{cell::RefCell, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::MutableVec,
    CancelableFutureHandle,
};

use super::{spawn_cancelable_future, Node};

/// Helper to build more complex sets of children.
///
/// Some children may be signals and optional. See
/// [`ParentBuilder::child_builder`] for an example.
///
/// [`ParentBuilder::child_builder`]: super::ParentBuilder::child_builder
#[derive(Default)]
pub struct ChildBuilder {
    pub(super) items: Rc<RefCell<MutableVec<Item>>>,
    pub(super) futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    len: usize,
}

type Item = Rc<RefCell<Option<Node>>>;

impl ChildBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, node: impl Into<Node>) -> Self {
        self.push(Some(node));
        self
    }

    pub fn child_signal(self, node: impl Signal<Item = impl Into<Node>> + 'static) -> Self {
        self.optional_child_signal(node.map(|e| Some(e)))
    }

    pub fn optional_child(&mut self, node: Option<impl Into<Node>>) {
        self.push(node);
    }

    pub fn optional_child_signal(
        mut self,
        node: impl Signal<Item = Option<impl Into<Node>>> + 'static,
    ) -> Self {
        let index = self.push(None as Option<Node>);
        let items = self.items.clone();

        let future = node.for_each(move |node| {
            items
                .borrow_mut()
                .lock_mut()
                .set_cloned(index, Rc::new(RefCell::new(node.map(|e| e.into()))));
            async {}
        });
        self.futures.push(spawn_cancelable_future(future));
        self
    }

    /// Push an item and return it's index
    fn push(&mut self, node: Option<impl Into<Node>>) -> usize {
        let index = self.len;
        self.items
            .borrow_mut()
            .lock_mut()
            .push_cloned(Rc::new(RefCell::new(node.map(|e| e.into()))));
        self.len += 1;
        index
    }
}
