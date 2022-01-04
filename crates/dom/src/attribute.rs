use web_sys as dom;

use crate::{clone, render::queue_update, DomElement, ElementBuilder};

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

        queue_update(dom_element.is_connected(), move || {
            dom_element.set_attribute(&name, &value).unwrap()
        });
    }
}

impl StaticAttribute for bool {
    fn set_attribute(&self, name: impl Into<String>, dom_element: &dom::Element) {
        clone!(dom_element);
        let name = name.into();
        let value = *self;

        queue_update(dom_element.is_connected(), move || {
            if value {
                dom_element.set_attribute(&name, "").unwrap();
            } else {
                dom_element.remove_attribute(&name).unwrap();
            }
        });
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
            None => queue_update(dom_element.is_connected(), move || {
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
