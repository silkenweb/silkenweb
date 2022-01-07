use wasm_bindgen::UnwrapThrowExt;
use web_sys as dom;

pub trait AttributeValue: Clone {
    fn text(self) -> String;
}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl AttributeValue for $typ {
                fn text(self) -> String {
                    format!("{}", self)
                }
            }
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);

impl AttributeValue for String {
    fn text(self) -> String {
        self
    }
}

impl<'a> AttributeValue for &'a str {
    fn text(self) -> String {
        self.to_owned()
    }
}

/// A non-reactive attribute.
pub trait StaticAttribute {
    fn set_attribute(self, name: &str, dom_element: &dom::Element);
}

impl<T: AttributeValue> StaticAttribute for T {
    fn set_attribute(self, name: &str, dom_element: &dom::Element) {
        dom_element.set_attribute(name, &self.text()).unwrap_throw();
    }
}

impl<T: StaticAttribute> StaticAttribute for Option<T> {
    fn set_attribute(self, name: &str, dom_element: &dom::Element) {
        if let Some(value) = self {
            value.set_attribute(name, dom_element);
        } else {
            dom_element.remove_attribute(name).unwrap_throw();
        }
    }
}

impl StaticAttribute for bool {
    fn set_attribute(self, name: &str, dom_element: &dom::Element) {
        if self {
            dom_element.set_attribute(name, "").unwrap_throw();
        } else {
            dom_element.remove_attribute(name).unwrap_throw();
        }
    }
}
