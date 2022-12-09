use std::{cell::RefCell, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{signal_vec::MutableVec, CancelableFutureHandle};
use silkenweb_signals_ext::value::{Executor, SignalOrValue, Value};

use super::spawn_cancelable_future;
use crate::{dom::Dom, node::Node};

/// Helper to build more complex sets of children.
///
/// Some children may be signals and optional. See
/// [`ParentElement::child_builder`] for an example.
///
/// [`ParentElement::child_builder`]: super::ParentElement::child_builder
pub struct ChildBuilder<D: Dom> {
    pub(super) items: Rc<RefCell<MutableVec<Item<D>>>>,
    pub(super) futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

impl<D: Dom> Default for ChildBuilder<D> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            futures: Default::default(),
        }
    }
}

type Item<D> = Rc<RefCell<Option<Node<D>>>>;

impl<D: Dom> ChildBuilder<D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(&mut self, node: impl SignalOrValue<Item = impl Value + Into<Node<D>> + 'static>) {
        self.optional_child(node.map(|e| Some(e)));
    }

    pub fn optional_child(
        &mut self,
        node: impl SignalOrValue<Item = Option<impl Value + Into<Node<D>> + 'static>>,
    ) {
        node.for_each(
            |builder, node| {
                if node.is_some() {
                    builder.push(node);
                }
            },
            |builder| {
                let index = builder.push(None as Option<Node<D>>);
                let items = builder.items.clone();

                move |node| {
                    items
                        .borrow_mut()
                        .lock_mut()
                        .set_cloned(index, Rc::new(RefCell::new(node.map(|e| e.into()))));
                    async {}
                }
            },
            self,
        );
    }

    /// Push an item and return it's index
    fn push(&mut self, node: Option<impl Into<Node<D>>>) -> usize {
        let items_mut = self.items.borrow_mut();
        let mut items_mut = items_mut.lock_mut();
        let index = items_mut.len();
        items_mut.push_cloned(Rc::new(RefCell::new(node.map(|e| e.into()))));
        index
    }
}

impl<D: Dom> Executor for ChildBuilder<D> {
    fn spawn(&mut self, future: impl futures::Future<Output = ()> + 'static) {
        self.futures.push(spawn_cancelable_future(future));
    }
}
