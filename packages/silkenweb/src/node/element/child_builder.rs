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
}

type Item = Rc<RefCell<Option<Node>>>;

impl ChildBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(&mut self, node: impl Into<Node>) {
        self.push(Some(node));
    }

    pub fn child_signal(&mut self, node: impl Signal<Item = impl Into<Node>> + 'static) {
        self.optional_child_signal(node.map(|e| Some(e)));
    }

    pub fn optional_child_signal(
        &mut self,
        node: impl Signal<Item = Option<impl Into<Node>>> + 'static,
    ) {
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
    }

    /// Push an item and return it's index
    fn push(&mut self, node: Option<impl Into<Node>>) -> usize {
        let items_mut = self.items.borrow_mut();
        let mut items_mut = items_mut.lock_mut();
        let index = items_mut.len();
        items_mut.push_cloned(Rc::new(RefCell::new(node.map(|e| e.into()))));
        index
    }
}
