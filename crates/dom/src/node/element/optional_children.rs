use std::{cell::RefCell, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::MutableVec, CancelableFutureHandle,
};

use crate::spawn_cancelable_future;

use super::Element;

pub struct OptionalChildren {
    pub(super) items: Rc<RefCell<MutableVec<Item>>>,
    pub(super) futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
    len: usize,
}

type Item = Rc<RefCell<Option<Element>>>;

impl OptionalChildren {
    pub fn child(mut self, elem: impl Into<Element>) -> Self{
        self.push(Some(elem));
        self
    }

    pub fn child_signal(self, elem: impl Signal<Item = impl Into<Element>> + 'static) -> Self {
        self.optional_child_signal(elem.map(|e| Some(e)))
    }

    pub fn optional_child(&mut self, elem: Option<impl Into<Element>>) {
        self.push(elem);
    }

    pub fn optional_child_signal(
        mut self,
        elem: impl Signal<Item = Option<impl Into<Element>>> + 'static,
    ) -> Self{
        let index = self.push(None as Option<Element>);
        let items = self.items.clone();

        let future = elem.for_each(move |elem| {
            items
                .borrow_mut()
                .lock_mut()
                .set_cloned(index, Rc::new(RefCell::new(elem.map(|e| e.into()))));
            async {}
        });
        self.futures.push(spawn_cancelable_future(future));
        self
    }

    /// Push an item and return it's index
    fn push(&mut self, elem: Option<impl Into<Element>>) -> usize {
        let index = self.len;
        self.items
            .borrow_mut()
            .lock_mut()
            .push_cloned(Rc::new(RefCell::new(elem.map(|e| e.into()))));
        self.len += 1;
        index
    }
}

pub fn optional_children() -> OptionalChildren {
    OptionalChildren {
        items: Rc::new(RefCell::new(MutableVec::new())),
        futures: Vec::new(),
        len: 0,
    }
}
