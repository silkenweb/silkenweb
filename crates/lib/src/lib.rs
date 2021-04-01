use std::{cell::Cell, rc::Rc};

use web_sys as dom;

pub fn tag(name: impl AsRef<str>) -> ElementBuilder {
    ElementBuilder::new(name)
}

pub struct ElementBuilder(Element);

impl ElementBuilder {
    pub fn new(tag: impl AsRef<str>) -> Self {
        ElementBuilder(Element {
            dom_element: DomElement(DOCUMENT.with(|doc| doc.create_element(tag.as_ref()).unwrap())),
            states: Vec::new(),
        })
    }

    pub fn attribute(self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.0
            .dom_element
            .0
            .set_attribute(name.as_ref(), value.as_ref())
            .unwrap();
        self
    }

    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();
        self.0
            .dom_element
            .0
            .append_child(&child.dom_element.0)
            .unwrap();
        self.0.states.extend(child.states);
        self
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        DOCUMENT.with(|doc| {
            self.0
                .dom_element
                .0
                .append_child(&doc.create_text_node(child.as_ref()))
                .unwrap()
        });
        self
    }

    pub fn build(self) -> Element {
        self.0
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

pub struct Element {
    dom_element: DomElement,
    states: Vec<Box<dyn AnyState>>,
}

impl Element {
    pub fn append_to_body(&self) {
        DOCUMENT.with(|doc| {
            doc.body()
                .expect("Document must contain a `body`")
                .append_child(&self.dom_element.0)
                .unwrap()
        });
    }

    // TODO: Don't allow
    #[allow(dead_code)]
    fn update(&self) {
        for state in &self.states {
            state.update()
        }
    }
}

pub fn state<T, E>(init: T, generate: impl 'static + Fn(T, StateSetter<T>) -> E) -> Element
where
    E: Into<Element>,
    T: 'static,
{
    let setter = StateSetter::default();
    let element = generate(init, setter.clone()).into();
    let dom_element = element.dom_element;

    Element {
        dom_element: dom_element.clone(),
        states: vec![Box::new(State {
            _dom_element: dom_element,
            generate: move |value, setter| generate(value, setter).into(),
            setter,
        })],
    }
}

pub struct StateSetter<T>(Rc<Cell<Option<T>>>);

impl<T> Clone for StateSetter<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for StateSetter<T> {
    fn default() -> Self {
        Self(Rc::new(Cell::new(None)))
    }
}

impl<T> StateSetter<T> {
    pub fn set(&self, new_value: T) {
        self.0.replace(Some(new_value));
    }

    fn take(&self) -> Option<T> {
        self.0.replace(None)
    }
}

trait AnyState {
    fn update(&self);
}

struct State<T, F> {
    _dom_element: DomElement,
    generate: F,
    setter: StateSetter<T>,
}

impl<T, F> AnyState for State<T, F>
where
    F: Fn(T, StateSetter<T>) -> Element,
{
    fn update(&self) {
        if let Some(new_value) = self.setter.take() {
            (self.generate)(new_value, self.setter.clone());
            println!("Replacing node");
        }
    }
}

#[derive(Clone)]
struct DomElement(dom::Element);

thread_local!(
    static DOCUMENT: dom::Document = dom::window()
        .expect("Window must be available")
        .document()
        .expect("Window must contain a document");
);

#[cfg(test)]
mod tests {
    use crate::{state, tag, ElementBuilder};

    fn parent() -> ElementBuilder {
        tag("tag_name").attribute("attr_name", "attr_value")
    }

    fn child(i: i32) -> ElementBuilder {
        tag("child_tag_name").attribute("child_attr_name", format!("{}", i))
    }

    #[test]
    fn simple() {
        parent().child(child(0)).build();
    }

    #[test]
    fn state_unchanged() {
        let element = parent().child(state(0, |i, _set_i| child(i))).build();
        println!("Updating");
        element.update();
    }

    #[test]
    fn state_changed() {
        let element = parent()
            .child(state(0, |i, set_i| {
                if i < 3 {
                    set_i.set(i + 1);
                }
                child(i)
            }))
            .build();
        println!("Updating");
        element.update();
        println!("Updating again");
        element.update();
        println!("Update 3");
        element.update();
        println!("Update 4");
        element.update();
    }
}
