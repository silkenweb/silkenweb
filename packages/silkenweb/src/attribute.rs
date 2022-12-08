//! Traits for defining attribute types.
//!
//! Once you've implemented [`Attribute`] and [`AsAttribute`] for your type, you
//! can use it with [`Element::attribute`], or define attributes on your
//! own html element using the [`custom_html_element`] macro.
//!
//! [`Element::attribute`]: crate::node::element::Element::attribute
use std::borrow::Cow;

/// A type that can be used as the value of an attribute.
pub trait Attribute {
    /// The attribute value text.
    ///
    /// Return `Some(text)` to set the attribute, or `None` to unset the
    /// attribute. For example, `bool` attributes set the attribute if `true`,
    /// or unset the attribute if `false`.
    fn text(&self) -> Option<Cow<str>>;
}

/// Define where an attribute type can be used.
///
/// For example, both `String` and `&str` can be used as `String` attributes
/// because `&str` implements `AsAttribute<String>`.
pub trait AsAttribute<T>: Attribute {}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl Attribute for $typ {
                fn text(&self) -> Option<Cow<str>> {
                    Some(Cow::from(format!("{}", self)))
                }
            }

            impl AsAttribute<$typ> for $typ {}
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);

impl Attribute for String {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

impl AsAttribute<String> for String {}

impl<T: Attribute> Attribute for Option<T> {
    fn text(&self) -> Option<Cow<str>> {
        self.as_ref().and_then(|attr| attr.text())
    }
}

impl<U: Attribute, T: AsAttribute<U>> AsAttribute<U> for Option<T> {}

impl Attribute for bool {
    fn text(&self) -> Option<Cow<str>> {
        if *self {
            Some(Cow::from(""))
        } else {
            None
        }
    }
}

impl AsAttribute<bool> for bool {}

impl<'a> Attribute for &'a str {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(*self))
    }
}

impl<'a> AsAttribute<String> for &'a str {}

impl<'a> Attribute for &'a String {
    fn text(&self) -> Option<Cow<str>> {
        Some(Cow::from(*self))
    }
}

impl<'a> AsAttribute<String> for &'a String {}
