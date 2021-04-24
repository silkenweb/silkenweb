use std::{
    cell::{Ref, RefCell},
    collections::{BTreeMap, BTreeSet},
    mem,
    ops::Bound::{Excluded, Unbounded},
    rc::Rc,
};

use silkenweb_reactive::signal::{ReadSignal, Signal};
use web_sys as dom;

use crate::{Builder, DomElement, Element, ElementBuilder};

type SharedItem<T> = Rc<RefCell<T>>;

struct StoredItem<T> {
    item: SharedItem<T>,
    updater: ReadSignal<()>,
}

struct OrderedElementList<Key> {
    root: ElementBuilder,
    items: BTreeMap<Key, Element>,
}

impl<Key> OrderedElementList<Key>
where
    Key: Ord + Eq,
{
    /// # Panic
    ///
    /// Panics if `root` has already had children added to it.
    pub fn new(root: ElementBuilder) -> Self {
        assert!(root.element.children.is_empty());

        Self {
            root,
            items: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Key, element: Element) {
        // TODO(testing): Add a test to make sure a reactive element gives us the
        // correct dom_element.
        let dom_element = element.dom_element();

        if let Some((_key, next_elem)) = self.items.range((Excluded(&key), Unbounded)).next() {
            self.root
                .insert_child_before(&dom_element, &next_elem.dom_element());
        } else {
            self.root.append_child(&dom_element);
        }

        if let Some(existing_elem) = self.items.insert(key, element) {
            self.root.remove_child(&existing_elem.dom_element());
        }
    }

    pub fn remove(&mut self, key: &Key) {
        if let Some(element) = self.items.remove(key) {
            self.root.remove_child(&element.dom_element());
        }
    }

    pub fn clear(&mut self) {
        for element in self.items.values() {
            self.root.remove_child(&element.dom_element());
        }

        self.items.clear();
    }
}

pub struct ElementList<Key, Value> {
    visible_items: Rc<RefCell<OrderedElementList<Key>>>,
    generate_child: Rc<dyn Fn(&Value) -> Element>,
    items: BTreeMap<Key, StoredItem<Value>>,
    filter: Box<dyn Fn(&Value) -> ReadSignal<bool>>,
}

impl<Key, Value> ElementList<Key, Value>
where
    Key: 'static + Clone + Ord + Eq,
    Value: 'static,
{
    /// # Panic
    ///
    /// Panics if `root` has already had children added to it.
    pub fn new<GenerateChild, ChildElem, ParentElem>(
        root: ParentElem,
        generate_child: GenerateChild,
        initial: impl Iterator<Item = (Key, Value)>,
    ) -> Self
    where
        ChildElem: Into<Element>,
        ParentElem: Into<ElementBuilder> + Builder,
        GenerateChild: 'static + Fn(&Value) -> ChildElem,
    {
        let mut new = Self {
            visible_items: Rc::new(RefCell::new(OrderedElementList::new(root.into()))),
            generate_child: Rc::new(move |c| generate_child(c).into()),
            items: BTreeMap::new(),
            filter: Box::new(|_| Signal::new(true).read()),
        };

        for (key, elem) in initial {
            new.insert(key, elem);
        }

        new
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn insert(&mut self, key: Key, item: Value) {
        let item = Rc::new(RefCell::new(item));
        let updater = self.updater(&key, &item);

        self.items.insert(key, StoredItem { item, updater });
    }

    pub fn pop(&mut self) {
        if let Some((key, _)) = self.items.iter().next_back() {
            // FEATURE(btree_pop_last): Don't clone the key and just pop last
            let key = key.clone();
            self.items.remove(&key);
            self.visible_items.borrow_mut().remove(&key);
        }
    }

    pub fn remove(&mut self, key: &Key) {
        if self.items.remove(key).is_some() {
            self.visible_items.borrow_mut().remove(key)
        }
    }

    pub fn filter(&mut self, f: impl 'static + Fn(&Value) -> ReadSignal<bool>) {
        let old_items = mem::take(&mut self.items);
        self.filter = Box::new(f);

        for (key, StoredItem { item, updater }) in old_items {
            mem::drop(updater);
            let updater = self.updater(&key, &item);
            self.items.insert(key, StoredItem { item, updater });
        }
    }

    pub fn retain(&mut self, f: impl Fn(&Value) -> bool) {
        // FEATURE(btree_map_retain): Use retain
        let mut to_remove = BTreeSet::new();

        for (key, value) in &self.items {
            if !f(&value.item.borrow()) {
                to_remove.insert(key.clone());
            }
        }

        for key in to_remove {
            self.remove(&key);
        }
    }

    pub fn values(&mut self) -> impl Iterator<Item = Ref<Value>> {
        self.items.values_mut().map(|stored| stored.item.borrow())
    }

    pub fn clear(&mut self) {
        self.visible_items.borrow_mut().clear();
        self.items.clear();
    }

    fn updater(&self, key: &Key, item: &Rc<RefCell<Value>>) -> ReadSignal<()> {
        (self.filter)(&item.borrow()).map({
            let storage = self.visible_items.clone();
            let item = item.clone();
            let generate_child = self.generate_child.clone();
            let key = key.clone();

            move |&visible| {
                if visible {
                    storage
                        .borrow_mut()
                        .insert(key.clone(), generate_child(&item.borrow()));
                } else {
                    storage.borrow_mut().remove(&key);
                }
            }
        })
    }
}

impl<Key, T> DomElement for ElementList<Key, T> {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.visible_items.borrow_mut().root.dom_element()
    }
}
