use wasm_bindgen::{intern, UnwrapThrowExt};

pub trait AttributeValue {
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

/// A non-reactive attribute.
pub trait Attribute {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element);

    fn text(self) -> Option<String>;
}

pub trait AsAttribute<T>: Attribute {}

impl<T: AttributeValue> Attribute for T {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element) {
        dom_element.set_attribute(name, &self.text()).unwrap_throw();
    }

    fn text(self) -> Option<String> {
        Some(AttributeValue::text(self))
    }
}

impl<T: AttributeValue> AsAttribute<T> for T {}

impl<T: Attribute> Attribute for Option<T> {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element) {
        if let Some(value) = self {
            value.set_attribute(name, dom_element);
        } else {
            dom_element.remove_attribute(name).unwrap_throw();
        }
    }

    fn text(self) -> Option<String> {
        self.and_then(Attribute::text)
    }
}

impl<U: Attribute, T: AsAttribute<U>> AsAttribute<U> for Option<T> {}

impl Attribute for bool {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element) {
        if self {
            dom_element.set_attribute(name, intern("")).unwrap_throw();
        } else {
            dom_element.remove_attribute(name).unwrap_throw();
        }
    }

    fn text(self) -> Option<String> {
        self.then(|| "".to_owned())
    }
}

impl AsAttribute<bool> for bool {}

impl<'a> Attribute for &'a str {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element) {
        dom_element.set_attribute(name, self).unwrap_throw();
    }

    fn text(self) -> Option<String> {
        Some(self.to_owned())
    }
}

impl<'a> AsAttribute<String> for &'a str {}

impl<'a> Attribute for &'a String {
    fn set_attribute(self, name: &str, dom_element: &web_sys::Element) {
        dom_element.set_attribute(name, self).unwrap_throw();
    }

    fn text(self) -> Option<String> {
        Some(self.to_owned())
    }
}

impl<'a> AsAttribute<String> for &'a String {}
