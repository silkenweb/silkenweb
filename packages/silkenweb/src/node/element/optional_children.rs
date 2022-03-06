use std::{cell::RefCell, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::MutableVec,
    CancelableFutureHandle,
};

use super::{spawn_cancelable_future, Node};
use crate::node::NodeImpl;

/// Helper to build children when some are optional and dependant on signal
/// vlaues.
///
/// See [`ParentBuilder::optional_children`] for an example.
///
/// [`ParentBuilder::optional_children`]: super::ParentBuilder::optional_children
pub struct OptionalChildren<Impl: NodeImpl> {
    pub(super) items: Rc<RefCell<MutableVec<Item<Impl>>>>,
    pub(super) futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    len: usize,
}

impl<Impl: NodeImpl> Default for OptionalChildren<Impl> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            futures: Default::default(),
            len: Default::default(),
        }
    }
}

type Item<Impl> = Rc<RefCell<Option<Node<Impl>>>>;

impl<Impl: NodeImpl> OptionalChildren<Impl> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, node: impl Into<Node<Impl>>) -> Self {
        self.push(Some(node));
        self
    }

    pub fn child_signal(self, node: impl Signal<Item = impl Into<Node<Impl>>> + 'static) -> Self {
        self.optional_child_signal(node.map(|e| Some(e)))
    }

    pub fn optional_child(&mut self, node: Option<impl Into<Node<Impl>>>) {
        self.push(node);
    }

    pub fn optional_child_signal(
        mut self,
        node: impl Signal<Item = Option<impl Into<Node<Impl>>>> + 'static,
    ) -> Self {
        let index = self.push(None as Option<Node<Impl>>);
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
    fn push(&mut self, node: Option<impl Into<Node<Impl>>>) -> usize {
        let index = self.len;
        self.items
            .borrow_mut()
            .lock_mut()
            .push_cloned(Rc::new(RefCell::new(node.map(|e| e.into()))));
        self.len += 1;
        index
    }
}
