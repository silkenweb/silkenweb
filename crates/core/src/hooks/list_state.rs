use std::{cell::RefCell, collections::BTreeMap, mem, rc::Rc};

use web_sys as dom;

use super::state::{ReadSignal, Signal};
use crate::{DomElement, Element, ElementBuilder};

type SharedItem<T> = Rc<T>;

struct StoredItem<T> {
    item: SharedItem<T>,
    updater: ReadSignal<()>,
}

struct Storage<T> {
    generate_child: Box<dyn Fn(&T) -> Element>,
    root: RefCell<ElementBuilder>,
    items: RefCell<BTreeMap<usize, StoredItem<T>>>,
    visible_items: RefCell<BTreeMap<usize, Element>>,
}

impl<T> Storage<T> {
    pub fn is_empty(&self) -> bool {
        self.items.borrow().is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn pop(&self) {
        let mut items = self.items.borrow_mut();

        if let Some((&key, _)) = items.iter().next_back() {
            items.remove(&key);

            if self.visible_items.borrow_mut().remove(&key).is_some() {
                self.root.borrow_mut().remove_last();
            }
        }
    }

    pub fn remove(&self, key: usize) {
        if self.items.borrow_mut().remove(&key).is_some() {
            if let Some(element) = self.visible_items.borrow_mut().remove(&key)
            {
                self.root.borrow_mut().remove_child(&element.dom_element());
            }
        }
    }
}

// TODO: Parameterize on key type
// TODO: Parameterize on storage type
pub struct ElementList<T> {
    storage: Rc<Storage<T>>,
    filter: Box<dyn Fn(&T) -> ReadSignal<bool>>,
}

impl<T: 'static> ElementList<T> {
    // TODO: Assert builders children empty.
    // How would we set attributes? Could take a Builder type and build it.
    pub fn new<GenerateChild>(
        root: ElementBuilder,
        generate_child: GenerateChild,
        initial: impl Iterator<Item = (usize, T)>,
    ) -> Self
    where
        GenerateChild: 'static + Fn(&T) -> Element,
    {
        let mut new = Self {
            storage: Rc::new(Storage {
                generate_child: Box::new(generate_child),
                root: RefCell::new(root),
                items: RefCell::new(BTreeMap::new()),
                visible_items: RefCell::new(BTreeMap::new()),
            }),
            filter: Box::new(|_| Signal::new(true).read()),
        };

        for (key, elem) in initial {
            new.insert(key, elem);
        }

        new
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn insert(&mut self, key: usize, item: T) {
        let item = Rc::new(item);
        let updater = self.updater(key, &item);

        self.storage
            .items
            .borrow_mut()
            .insert(key, StoredItem { item, updater });
    }

    pub fn pop(&mut self) {
        self.storage.pop()
    }

    pub fn remove(&mut self, key: usize) {
        self.storage.remove(key)
    }

    pub fn filter(&mut self, f: impl 'static + Fn(&T) -> ReadSignal<bool>) {
        let old_items = self.storage.items.take();
        self.filter = Box::new(f);
        let mut items = self.storage.items.borrow_mut();

        for (key, StoredItem { item, updater }) in old_items {
            mem::drop(updater);
            let updater = self.updater(key, &item);
            items.insert(key, StoredItem { item, updater });
        }
    }

    fn updater(&self, key: usize, item: &Rc<T>) -> ReadSignal<()> {
        (self.filter)(&item).map({
            let storage = self.storage.clone();
            let item = item.clone();

            move |&visible| {
                if visible {
                    let item = item.clone();

                    storage
                        .visible_items
                        .borrow_mut()
                        .entry(key)
                        .or_insert_with(|| {
                            let element = (storage.generate_child)(&item);

                            // TODO: Insert child in correct place.
                            storage
                                .root
                                .borrow_mut()
                                .append_child(&element.dom_element());

                            element
                        });
                } else if let Some(element) =
                    storage.visible_items.borrow_mut().remove(&key)
                {
                    storage
                        .root
                        .borrow_mut()
                        .remove_child(&element.dom_element())
                }
            }
        })
    }
}

impl<T> DomElement for ElementList<T> {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.storage.root.borrow().dom_element()
    }
}
