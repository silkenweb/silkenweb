use std::{cell::RefCell, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle,
};

use super::{spawn_cancelable_future, Node};

/// Helper to build children when some are optional and dependant on signal
/// vlaues.
///
/// # Example
///
/// ```no_run
/// # use futures_signals::signal::{Mutable, SignalExt};
/// # use silkenweb::{
/// #     elements::html::div,
/// #     node::{
/// #         element::{OptionalChildren, ParentBuilder},
/// #         text,
/// #     },
/// # };
/// let include_child1 = Mutable::new(true);
/// let include_child2 = Mutable::new(true);
///
/// div().children_signal(
///     OptionalChildren::new()
///         .optional_child_signal(
///             include_child1
///                 .signal()
///                 .map(|child1| child1.then(|| text("This is child1"))),
///         )
///         .optional_child_signal(
///             include_child2
///                 .signal()
///                 .map(|child1| child1.then(|| text("This is child2"))),
///         )
///         .signal_vec(),
/// );
/// ```
#[derive(Default)]
pub struct OptionalChildren {
    pub(super) items: Rc<RefCell<MutableVec<Item>>>,
    pub(super) futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    len: usize,
}

type Item = Rc<RefCell<Option<Node>>>;

impl OptionalChildren {
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

    pub fn signal_vec(self) -> impl SignalVec<Item = Node> {
        self.items
            .borrow()
            .signal_vec_cloned()
            .filter_map(|e| e.borrow_mut().take())
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
