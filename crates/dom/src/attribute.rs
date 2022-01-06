use wasm_bindgen::UnwrapThrowExt;
use web_sys as dom;

use crate::{DomElement, ElementBuilder};

pub trait AttributeValue: Clone {
    type Text: AsRef<str>;

    fn text(&self) -> Self::Text;
}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl AttributeValue for $typ {
                type Text = String;

                fn text(&self) -> Self::Text {
                    format!("{}", self)
                }
            }
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);

/// A non-reactive attribute.
pub trait StaticAttribute: Clone {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element) {
        dom_element
            .set_attribute(name, self.text().as_ref())
            .unwrap_throw();
    }
}

impl StaticAttribute for String {
    fn set_attribute(&self, name: &str, dom_element: &dom::Element) {
        dom_element.set_attribute(name, self).unwrap_throw();
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
        builder
            .dom_element()
            .set_attribute(name, self)
            .unwrap_throw();
    }
}
