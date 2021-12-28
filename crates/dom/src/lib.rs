//! A reactive interface to the DOM.
// TODO: Split this file up
pub mod render;
use std::{cell::RefCell, collections::HashMap, future::Future};

use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
use render::queue_update;
use wasm_bindgen_futures::spawn_local;
use web_sys as dom;

pub mod macros;

mod element;

pub use element::{signal, Builder, DomElement, Element, ElementBuilder};

/// Mount an element on the document.
///
/// `id` is the html element id of the parent element. The element is added as
/// the last child of this element.
///
/// Mounting an `id` that is already mounted will remove that element.
///
/// An [`Element`] can only appear once in the document. Adding an [`Element`]
/// to the document a second time will move it. It will still require
/// unmounting from both places to free up any resources.
pub fn mount(id: &str, elem: impl Into<Element>) {
    unmount(id);
    let elem = elem.into();

    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("DOM node id = '{}' must exist", id))
        .append_child(elem.dom_element())
        .unwrap();
    APPS.with(|apps| apps.borrow_mut().insert(id.to_owned(), elem));
}

/// Unmount an element.
///
/// This is mostly useful for testing and checking for memory leaks
pub fn unmount(id: &str) {
    if let Some(elem) = APPS.with(|apps| apps.borrow_mut().remove(id)) {
        elem.dom_element().remove();
    }
}

pub fn window() -> dom::Window {
    dom::window().expect("Window must be available")
}

pub fn document() -> dom::Document {
    window().document().expect("Window must contain a document")
}

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: &str) -> ElementBuilder {
    ElementBuilder::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: &str, name: &str) -> ElementBuilder {
    ElementBuilder::new_in_namespace(namespace, name)
}

fn spawn_cancelable_future(
    future: impl 'static + Future<Output = ()>,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    spawn_local(cancelable_future);

    handle
}

pub trait AttributeValue {
    fn text(&self) -> String;
}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl AttributeValue for $typ {
                fn text(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);
define_attribute_values!(String);

/// A non-reactive attribute.
pub trait StaticAttribute {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();
        let value = self.text();

        queue_update(move || dom_element.set_attribute(&name, &value).unwrap());
    }
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();

        if *self {
            queue_update(move || {
                dom_element.set_attribute(&name, "").unwrap();
            });
        } else {
            queue_update(move || {
                dom_element.remove_attribute(&name).unwrap();
            });
        }
    }
}

/// Set the attribute, or remove it if the option is `None`.
///
/// Although this only really makes sense for attribute signals, we implement it
/// for `StaticAttribute`s because we fall foul of orphan rules if we try to
/// implement it for all signals of `AttributeValue`s.
impl<T: AttributeValue> StaticAttribute for Option<T> {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();

        match self {
            Some(value) => value.set_attribute(name, &dom_element),
            None => queue_update(move || {
                dom_element.remove_attribute(&name).unwrap();
            }),
        }
    }
}

/// A potentially reactive attribute.
pub trait Attribute<T> {
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder);
}

impl<T> Attribute<T> for T
where
    T: StaticAttribute,
{
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder) {
        StaticAttribute::set_attribute(&self, name, builder.dom_element());
    }
}

impl<'a> Attribute<String> for &'a str {
    fn set_attribute(self, name: impl Into<String>, builder: &mut ElementBuilder) {
        self.to_string().set_attribute(name, builder);
    }
}

// TODO: We probably want a better storage API.
// We want to be able to iterator over it like a map using Object::entries and
// Object::keys
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
pub fn local_storage() -> Option<dom::Storage> {
    // TODO: Under what circumstances can these fail?
    window().local_storage().unwrap()
}

/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
pub fn session_storage() -> Option<dom::Storage> {
    window().session_storage().unwrap()
}

thread_local!(
    static APPS: RefCell<HashMap<String, Element>> = RefCell::new(HashMap::new());
);
