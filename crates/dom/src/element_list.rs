//! Manage reactive lists of DOM elements.
use std::{
    cell::{Ref, RefCell},
    collections::{BTreeMap, BTreeSet},
    mem,
    ops::Bound::{Excluded, Unbounded},
    rc::Rc,
};

use silkenweb_reactive::{
    clone,
    signal::{ReadSignal, Signal},
};
use web_sys as dom;

use crate::{DomElement, Element, ElementBuilder};

/// A filterable, ordered element list.
///
/// This owns the data to create child elements, and will manage adding/removing
/// them from the DOM as the filter requires.
///
/// The list is ordered by `Key`.
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
    /// Create a new [`ElementList`].
    ///
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
        ParentElem: Into<ElementBuilder>,
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

    /// `true` iff the list, without a filter applied, is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The length of the list without any filter applied.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Insert an item into the list. If an item exists at `key`, it is
    /// replaced.
    pub fn insert(&mut self, key: Key, item: Value) {
        let item = Rc::new(RefCell::new(item));
        let updater = self.updater(&key, &item);

        self.items.insert(key, StoredItem { item, updater });
    }

    /// Pop the last element from the list. If the list is empty, this has no
    /// effect.
    pub fn pop(&mut self) {
        if let Some((key, _)) = self.items.iter().next_back() {
            // RUSTC(btree_pop_last): Don't clone the key and just pop last
            clone!(key);
            self.items.remove(&key);
            self.visible_items.borrow_mut().remove(&key);
        }
    }

    /// Remove the item corresponding to `key`. If the item is not in the list,
    /// this has no effect.
    pub fn remove(&mut self, key: &Key) {
        if self.items.remove(key).is_some() {
            self.visible_items.borrow_mut().remove(key)
        }
    }

    /// Apply a filter to the list, replacing any existing filter.
    pub fn filter(&mut self, f: impl 'static + Fn(&Value) -> ReadSignal<bool>) {
        let old_items = mem::take(&mut self.items);
        self.filter = Box::new(f);

        for (key, StoredItem { item, updater }) in old_items {
            mem::drop(updater);
            let updater = self.updater(&key, &item);
            self.items.insert(key, StoredItem { item, updater });
        }
    }

    /// Remove all items for which `f` returns `false`. Matching items that are
    /// currently filtered out will still be removed.
    pub fn retain(&mut self, f: impl Fn(&Value) -> bool) {
        // RUSTC(btree_map_retain): Use retain
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

    /// An iterator over all values in the list, including hidden items. If
    /// `Value` is interiorly mutable and reactivity with the filter
    /// is correctly set up, it's safe to mutate the items.
    pub fn values(&mut self) -> impl Iterator<Item = Ref<Value>> {
        self.items.values_mut().map(|stored| stored.item.borrow())
    }

    /// Clear all the items from the list, including filtered items.
    pub fn clear(&mut self) {
        self.visible_items.borrow_mut().clear();
        self.items.clear();
    }

    fn updater(&self, key: &Key, item: &Rc<RefCell<Value>>) -> ReadSignal<()> {
        (self.filter)(&item.borrow()).map({
            let storage = self.visible_items.clone();
            clone!(item, key);
            let generate_child = self.generate_child.clone();

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
        self.visible_items.borrow().dom_element()
    }
}

/// A list ordered by `Key`.
pub struct OrderedElementList<Key> {
    root: ElementBuilder,
    items: BTreeMap<Key, Element>,
}

impl<Key> OrderedElementList<Key>
where
    Key: Ord + Eq,
{
    /// Create a new [`OrderedElementList`].
    ///
    /// # Panic
    ///
    /// Panics if `root` has already had children added to it.
    pub fn new<ParentElem>(root: ParentElem) -> Self
    where
        ParentElem: Into<ElementBuilder>,
    {
        let root = root.into();
        assert!(root.element.children.is_empty());

        Self {
            root,
            items: BTreeMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Insert an element. If the element exists, it will be replaced.
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

    /// Remove an item from the list. If no item exists for `key`, this has no
    /// effect.
    pub fn remove(&mut self, key: &Key) {
        if let Some(element) = self.items.remove(key) {
            self.root.remove_child(&element.dom_element());
        }
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        for element in self.items.values() {
            self.root.remove_child(&element.dom_element());
        }

        self.items.clear();
    }
}

impl<Key> DomElement for OrderedElementList<Key> {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.root.dom_element()
    }
}

struct StoredItem<T> {
    item: SharedItem<T>,
    updater: ReadSignal<()>,
}

type SharedItem<T> = Rc<RefCell<T>>;
