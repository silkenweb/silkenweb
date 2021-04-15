use crate::{DomElement, Element, ElementBuilder};

pub struct ElementList<T> {
    root: ElementBuilder,
    children: Vec<Element>,
    generate_child: Box<dyn Fn(&T) -> Element>,
}

impl<T: 'static> ElementList<T> {
    // TODO: Accept other element builder types (maybe take a function like `div`)
    // How would we set attributes? Could take a Builder type and build it.
    pub fn new<'a, GenerateChild, Elem>(
        root: ElementBuilder,
        generate_child: GenerateChild,
        initial: impl Iterator<Item = &'a T>,
    ) -> Self
    where
        Elem: Into<Element>,
        GenerateChild: 'static + Fn(&T) -> Elem,
    {
        let mut new = Self {
            root,
            children: Vec::new(),
            generate_child: Box::new(move |e| generate_child(e).into()),
        };

        for elem in initial {
            new.push(elem);
        }

        new
    }

    pub fn push(&mut self, new_elem: &T) {
        let child = (self.generate_child)(&new_elem);

        self.root.append_child(&child.dom_element());
        self.children.push(child);
    }

    pub fn pop(&mut self) {
        let child = self.children.pop().expect("List must be non-empty");
        self.root.remove_child(&child.dom_element());
    }
}

impl<T> DomElement for ElementList<T> {
    fn dom_element(&self) -> web_sys::Element {
        self.root.dom_element()
    }
}
