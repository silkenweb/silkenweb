//! Traits for defining attribute types.
//!
//! [`Attribute`] defines how an attribute is rendered and [`AsAttribute`] is a
//! marker trait to say where the attribute can be used.
//!
//! For example, both `String` and `&str` can be used as `String` attributes
//! because `&str` implements `AsAttribute<String>`.
//!
//! Once you've implemented [`Attribute`] and [`AsAttribute`] for your type, you
//! can use it with [`Element::attribute`], or define attributes on your
//! own html element using the [`custom_html_element`] macro.
//!
//! [`Element::attribute`]: crate::node::element::Element::attribute

/// A type that can be used as the value of an attribute.
///
/// See [module-level documentation](self) for more details.
pub trait Attribute {
    type Text<'a>: 'a + AsRef<str> + Into<String> + ToString
    where
        Self: 'a;

    /// The attribute value text.
    ///
    /// Return `Some(text)` to set the attribute, or `None` to unset the
    /// attribute. For example, `bool` attributes set the attribute if `true`,
    /// or unset the attribute if `false`.
    fn text(&self) -> Option<Self::Text<'_>>;
}

/// Define where an attribute type can be used.
///
/// See [module-level documentation](self) for more details.
pub trait AsAttribute<T>: Attribute {}

macro_rules! define_attribute_values{
    ($($typ:ty),* $(,)?) => {
        $(
            impl Attribute for $typ {
                type Text<'a> = String;

                fn text(&self) -> Option<Self::Text<'_>> {
                    Some(self.to_string())
                }
            }

            impl AsAttribute<$typ> for $typ {}
        )*
    }
}

define_attribute_values!(i8, i16, i32, i64);
define_attribute_values!(u8, u16, u32, u64);
define_attribute_values!(f32, f64);
define_attribute_values!(usize);

impl Attribute for String {
    type Text<'a> = &'a str;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(self)
    }
}

impl AsAttribute<String> for String {}

impl<T: Attribute> Attribute for Option<T> {
    type Text<'a>
        = T::Text<'a>
    where
        T: 'a;

    fn text(&self) -> Option<Self::Text<'_>> {
        self.as_ref()?.text()
    }
}

impl<U: Attribute, T: AsAttribute<U>> AsAttribute<U> for Option<T> {}

impl Attribute for bool {
    type Text<'a> = &'static str;

    fn text(&self) -> Option<Self::Text<'_>> {
        if *self {
            Some("")
        } else {
            None
        }
    }
}

impl AsAttribute<bool> for bool {}

impl<'a> Attribute for &'a str {
    type Text<'b>
        = &'b str
    where
        'a: 'b;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(*self)
    }
}

impl AsAttribute<String> for &str {}

impl<'a> Attribute for &'a String {
    type Text<'b>
        = &'b str
    where
        'a: 'b;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(self.as_str())
    }
}

impl AsAttribute<String> for &String {}
