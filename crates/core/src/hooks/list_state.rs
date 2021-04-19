use web_sys as dom;

use crate::{DomElement, Element, ElementBuilder};

pub struct ElementList<T> {
    root: ElementBuilder,
    generate_child: Box<dyn Fn(&T) -> Element>,
}

impl<T: 'static> ElementList<T> {
    // TODO: Assert builders children empty.
    // How would we set attributes? Could take a Builder type and build it.
    pub fn new<'a, GenerateChild>(
        root: ElementBuilder,
        generate_child: GenerateChild,
        initial: impl Iterator<Item = &'a T>,
    ) -> Self
    where
        GenerateChild: 'static + Fn(&T) -> Element,
    {
        let mut new = Self {
            root,
            generate_child: Box::new(generate_child),
        };

        for elem in initial {
            new.push(elem);
        }

        new
    }

    pub fn push(&mut self, new_elem: &T) {
        let child = (self.generate_child)(&new_elem);
        self.root.append_child(&child.dom_element());
    }

    pub fn remove(&mut self, elem: &dom::Element) {
        self.root.remove_child(elem);
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
