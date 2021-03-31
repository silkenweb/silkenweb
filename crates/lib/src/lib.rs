use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub struct Attribute {
    name: String,
    value: String,
}

// TODO: Make private
pub struct ConcreteElement {
    tag: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
}

pub struct IsUpdated(bool);

pub trait AnyState {
    fn eval(&self) -> Element;

    fn update(&mut self) -> IsUpdated;
}

pub enum Element {
    // TODO: Should this be a DOM element?
    Concrete(ConcreteElement),
    NewState(Box<dyn AnyState>),
}

pub struct ElementBuilder {
    tag: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
}

impl ElementBuilder {
    pub fn new(tag: impl ToString) -> Self {
        Self {
            tag: tag.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attribute(mut self, name: impl ToString, value: impl ToString) -> Self {
        self.attributes.push(Attribute {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        self.children.push(child.into());
        self
    }
}

// TODO: Name? This handles updates as well
pub struct StateReader<T> {
    current: T,
    next: StateWriter<T>,
}

struct WithState<T, F> {
    reader: StateReader<T>,
    f: F,
}

impl<T, F> AnyState for WithState<T, F>
where
    F: Fn(&StateReader<T>, StateWriter<T>) -> Element,
{
    fn eval(&self) -> Element {
        (self.f)(&self.reader, self.reader.next.clone())
    }

    fn update(&mut self) -> IsUpdated {
        let next = self.reader.next.0.as_ref().replace(None);

        IsUpdated(if let Some(new) = next {
            self.reader.current = new;
            true
        } else {
            false
        })
    }
}

pub struct StateWriter<T>(Rc<RefCell<Option<T>>>);

impl<T> Clone for StateWriter<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for StateWriter<T> {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

pub fn state<T: 'static>(init: T, f: impl 'static + Fn(&StateReader<T>, StateWriter<T>) -> Element) -> Element
{
    let writer = StateWriter::default();
    let reader = StateReader {
        current: init,
        next: writer,
    };

    Element::NewState(Box::new(WithState {
        reader,
        f,
    }))
}

pub struct Dom {}
