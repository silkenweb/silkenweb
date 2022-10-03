use futures_signals::signal::{Signal, SignalExt};
use paste::paste;
use silkenweb_base::intern_str;

use crate::{elements::class_attribute_text, node::element::ElementBuilder};

macro_rules! attributes {
    ($(
        $(#[$attr_meta:meta])*
        $attr:ident: $typ:ty
    ),* $(,)? ) => { paste!{
        $crate::attributes![
            $($(#[$attr_meta])*
            $attr ($crate::text_name!($attr)): $typ,)*
        ];
    }};
}

/// An SVG element
///
/// Methods for setting attributes specific to SVG elements
pub trait Global: ElementBuilder {
    fn class(self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.attribute(intern_str("class"), class_attribute_text(value))
    }

    fn class_signal<Iter: IntoIterator<Item = impl AsRef<str>>>(
        self,
        value: impl Signal<Item = Iter> + 'static,
    ) -> Self {
        self.attribute_signal(
            intern_str("class"),
            value.map(move |class| class_attribute_text(class)),
        )
    }

    attributes![
        /// Defines a unique identifier (ID) which must be unique in the whole
        /// document. Its purpose is to identify the element when linking (using
        /// a fragment identifier), scripting, or styling (with CSS). Value: Any
        /// valid ID string; Animatable: No.
        id: String,
        /// Participates in defining the language of the element, the language
        /// that non-editable elements are written in or the language that
        /// editable elements should be written in. The tag contains one single
        /// entry value in the format defined in
        /// [RFC 5646: Tags for Identifying Languages (also known as BCP 47)](https://datatracker.ietf.org/doc/html/rfc5646).
        lang: String,
        /// The tabindex SVG attribute allows you to control whether an element
        /// is focusable and to define the relative order of the element for the
        /// purposes of sequential focus navigation. Animatable: No
        tabindex: i32,
        /// It specifies style information for its element. It functions
        /// identically to the style attribute in HTML. Value: Any valid style
        /// string; Animatable: No
        style: String,
    ];
}
