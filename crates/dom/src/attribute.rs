use wasm_bindgen::UnwrapThrowExt;
use web_sys as dom;

use crate::{DomElement, ElementBuilder};

pub trait AttributeValue: Clone {
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
pub trait StaticAttribute: Clone {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element) {
        let value = self.text();

        dom_element.set_attribute(name, &value).unwrap_throw();
    }
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element) {
        if *self {
            dom_element.set_attribute(name, "").unwrap_throw();
        } else {
            dom_element.remove_attribute(name).unwrap_throw();
        }
    }
}

/// Set the attribute, or remove it if the option is `None`.
///
/// Although this only really makes sense for attribute signals, we implement it
/// for `StaticAttribute`s because we fall foul of orphan rules if we try to
/// implement it for all signals of `AttributeValue`s.
impl<T: AttributeValue> StaticAttribute for Option<T> {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element) {
        match self {
            Some(value) => value.set_attribute(name, dom_element),
            None => dom_element.remove_attribute(name).unwrap_throw(),
        }
    }
}

/// A potentially reactive attribute.
pub trait Attribute<T> {
    fn set_attribute(self, name: &str, builder: &mut ElementBuilder);
}

impl<T> Attribute<T> for T
where
    T: StaticAttribute,
{
    fn set_attribute(self, name: &str, builder: &mut ElementBuilder) {
        StaticAttribute::set_attribute(&self, name, builder.dom_element());
    }
}

impl<'a> Attribute<String> for &'a str {
    fn set_attribute(self, name: &str, builder: &mut ElementBuilder) {
        self.to_string().set_attribute(name, builder);
    }
}
