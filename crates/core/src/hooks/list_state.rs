use indexmap::IndexMap;
use web_sys as dom;

use crate::{DomElement, Element, ElementBuilder};

use super::state::ReadSignal;

struct Item<T> {
    item: T,
    element: Element,
}

pub struct ElementList<T> {
    root: ElementBuilder,
    items: IndexMap<usize, Item<T>>,
    generate_child: Box<dyn Fn(&T) -> Element>,
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
            root,
            items: IndexMap::new(),
            generate_child: Box::new(generate_child),
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

    pub fn filter(&mut self, f: impl Fn(&T) -> ReadSignal<bool>) {}

    pub fn insert(&mut self, key: usize, new_elem: T) {
        let child = (self.generate_child)(&new_elem);
        self.items.insert(
            key,
            Item {
                item: new_elem,
                element: child.clone(),
            },
        );
        self.root.append_child(&child.dom_element());
    }

    pub fn remove(&mut self, key: usize) {
        if let Some(item) = self.items.shift_remove(&key) {
            self.root.remove_child(&item.element.dom_element());
        }
    }

    pub fn pop(&mut self) {
        self.root.remove_last();
    }
}

impl<T> DomElement for ElementList<T> {
    type Target = dom::Element;

    fn dom_element(&self) -> Self::Target {
        self.root.dom_element()
    }
}
