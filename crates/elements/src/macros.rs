pub use futures_signals::{signal::Signal, signal_vec::SignalVec};
pub use paste::paste;
pub use silkenweb_dom::{
    attribute::{AsAttribute, Attribute},
    element::{Element, ElementBuilder, GenericElementBuilder},
    tag, tag_in_namespace,
};
pub use wasm_bindgen::{intern, JsCast, JsValue, UnwrapThrowExt};
pub use web_sys;

/// Define an html element.
///
/// This will define a builder struct for an html element, with a method for
/// each attribute. It will also define a struct for the built element. Dashes
/// are allowed in element, attribute, and event names. They will be converted
/// to underscores when generating rust identifiers. For example:
///
/// ```no_run
/// # use silkenweb_elements::html_element;
/// use silkenweb_elements::CustomEvent;
///
/// // The types of the dom element and event carry through to the event handler.
/// html_element!(my-html-element<web_sys::HtmlDivElement> {
///     attributes {
///         my-attribute: String
///     }
///
///     events {
///         my-event: web_sys::MouseEvent
///     }
///
///     custom_events {
///         my-custom-event: CustomEvent<web_sys::HtmlElement>,
///     }
/// });
///
/// let elem = my_html_element()
///     .my_attribute("attribute-value")
///     .on_my_event(|event: web_sys::MouseEvent, target: web_sys::HtmlDivElement| {})
///     .on_my_custom_event(|event: CustomEvent<web_sys::HtmlElement>, target: web_sys::HtmlDivElement| {});
/// ```
#[macro_export]
macro_rules! html_element {
    ($($t:tt)*) => {
        $crate::dom_element!(
            attributes = [$crate::HtmlElement],
            events = [$crate::HtmlElementEvents],
            $($t)*
        );
    }
}

macro_rules! svg_element {
    ($($t:tt)*) => {
        $crate::dom_element!(
            namespace = "http://www.w3.org/2000/svg",
            // TODO: Add default attributes
            attributes = [],
            // TODO: Add default events
            events = [],
            $($t)*
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! dom_element {
    (
        $(namespace = $namespace:literal, )?
        attributes = [$($attribute_trait:ty),*],
        events = [$($event_trait:ty),*],
        $(#[$elem_meta:meta])*
        $name:ident $(- $name_tail:ident)*
        < $elem_type:ty >
        {
            $(attributes {
                $(
                    $(#[$attr_meta:meta])*
                    $attr:ident $(- $attr_tail:ident)*: $typ:ty
                ),* $(,)?
            })?

            $(events {
                $($event:ident $(- $event_tail:ident)*: $event_type:ty),* $(,)?
            })?

            $(custom_events {
                $($custom_event:ident $(- $custom_event_tail:ident)*: $custom_event_type:ty),* $(,)?
            })?
        }
    ) => { $crate::macros::paste!{
        $crate::dom_element!(
            $(namespace = $crate::macros::intern($namespace), )?
            attributes = [$($attribute_trait),*],
            events = [$($event_trait),*],
            $(#[$elem_meta])*
            snake ( [< $name:snake $(_ $name_tail:snake)* >] ),
            camel ( [< $name:camel $($name_tail:camel)* >], [< $name:camel $($name_tail:camel)* Builder >] ),
            text ( $crate::text_name!($name $(- $name_tail)*) )
            < $elem_type >
            {
                $(attributes { $(
                    $(#[$attr_meta])*
                    // TODO: If [this](https://github.com/dtolnay/paste/issues/74#issue-1100247715) paste issue gets resolved,
                    // use `[< $attr:snake >]` and replace `text_attr!` usage with `text_name`.
                    $attr $($attr_tail)* ( $crate::text_attr!($attr $(- $attr_tail)*) ) : $typ
                ),*})?

                $(events {
                    $($event $(- $event_tail)*: $event_type),*
                })?

                $(custom_events {
                    $($custom_event $(- $custom_event_tail)*: $custom_event_type),*
                })?
            }
        );
    }};
    (
        $(namespace = $namespace:expr, )?
        attributes = [$($attribute_trait:ty),*],
        events = [$($event_trait:ty),*],
        $(#[$elem_meta:meta])*
        snake ( $snake_name:ident ),
        camel ( $camel_name:ident, $camel_builder_name:ident ),
        text ( $text_name:expr )
        < $elem_type:ty >
        {
            $(attributes { $(
                $(#[$attr_meta:meta])*
                $attr:ident $($attr_tail:ident)* ($text_attr:expr) : $typ:ty
            ),* $(,)? } )?

            $(events {
                $($event:ident $(- $event_tail:ident)*: $event_type:ty),* $(,)?
            })?

            $(custom_events {
                $($custom_event:ident $(- $custom_event_tail:ident)*: $custom_event_type:ty),* $(,)?
            })?
        }
    ) => {
        $(#[$elem_meta])*
        pub fn $snake_name() -> $camel_builder_name {
            $camel_builder_name { builder: $crate::create_element_fn!(
                $($namespace, )? $text_name
            ) }
        }

        pub struct $camel_builder_name {
            builder: $crate::macros::GenericElementBuilder
        }

        impl $camel_builder_name {
            $crate::attributes![
                $($($(#[$attr_meta])* pub $attr $($attr_tail)* ($text_attr): $typ,)*)?
            ];

            $($crate::events!(
                $elem_type {
                    $(pub $event $(- $event_tail)*: $event_type),*
                }
            ); )?

            $($crate::custom_events!(
                $elem_type {
                    $($custom_event $(- $custom_event_tail)*: $custom_event_type),*
                }
            ); )?
        }

        impl $crate::Effects<$elem_type> for $camel_builder_name {
            fn effect(self, f: impl ::std::ops::FnOnce(&$elem_type) + 'static) -> Self {
                Self{ builder: self.builder.effect(f) }
            }

            fn effect_signal<T: 'static>(
                self,
                sig: impl $crate::macros::Signal<Item = T> + 'static,
                f: impl Fn(&$elem_type, T) + Clone + 'static,
            ) -> Self {
                Self{ builder: self.builder.effect_signal(sig, f) }
            }
        }

        impl $crate::macros::ElementBuilder for $camel_builder_name {
            type Target = $camel_name;

            fn attribute<T: $crate::macros::Attribute>(self, name: &str, value: T) -> Self {
                Self{ builder: self.builder.attribute(name, value) }
            }

            fn attribute_signal<T: $crate::macros::Attribute + 'static>(
                self,
                name: &str,
                value: impl $crate::macros::Signal<Item = T> + 'static,
            ) -> Self {
                Self{ builder: $crate::macros::ElementBuilder::attribute_signal(self.builder, name, value) }
            }

            fn on(
                self,
                name: &'static str,
                f: impl FnMut($crate::macros::JsValue) + 'static
            ) -> Self {
                Self{ builder: $crate::macros::ElementBuilder::on(self.builder, name, f) }
            }

            fn build(self) -> Self::Target {
                $camel_name(self.builder.build())
            }

            fn into_element(self) -> $crate::macros::Element {
                self.build().into()
            }
        }

        impl From<$camel_builder_name> for $crate::macros::Element {
            fn from(builder: $camel_builder_name) -> Self {
                $crate::macros::ElementBuilder::build(builder).into()
            }
        }

        impl From<$camel_builder_name> for $crate::macros::GenericElementBuilder {
            fn from(builder: $camel_builder_name) -> Self {
                builder.builder
            }
        }

        pub struct $camel_name($crate::macros::Element);

        impl From<$camel_name> for $crate::macros::Element {
            fn from(html_elem: $camel_name) -> Self {
                html_elem.0
            }
        }

        $(impl $attribute_trait for $camel_builder_name {})*

        $(
            impl $event_trait for $camel_builder_name {
                type EventTarget = $elem_type;
            }
        )*

        impl $crate::ElementEvents for $camel_builder_name {
            type EventTarget = $elem_type;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! create_element_fn {
    ($text_name:expr) => {
        $crate::macros::tag($text_name)
    };
    ($namespace:expr, $text_name:expr) => {
        $crate::macros::tag_in_namespace($namespace, $text_name)
    };
}

/// Add `child` and `text` methods to an html element builder.
///
/// See [`html_element`] for a complete example of defining an html element.
#[macro_export]
macro_rules! parent_element {
    ($name:ident $(- $name_tail:ident)*) => {
        impl $crate::ParentBuilder for
            $crate::camel_name!{$name $($name_tail)* Builder}
        {
            fn text(self, child: &str) -> Self {
                Self{ builder: self.builder.text(child) }
            }

            fn text_signal(
                self,
                child: impl $crate::macros::Signal<Item = impl Into<String>> + 'static
            ) -> Self {
                Self{ builder: self.builder.text_signal(child) }
            }

            fn child<Child>(self, c: Child) -> Self
            where
                Child: Into<$crate::macros::Element>
            {
                Self{ builder: self.builder.child(c) }
            }

            fn child_signal(
                self,
                child: impl $crate::macros::Signal<Item = impl Into<$crate::macros::Element>> + 'static
            ) -> Self {
                Self{ builder: self.builder.child_signal(child) }
            }

            fn children_signal(
                self,
                children: impl $crate::macros::SignalVec<Item = impl Into<$crate::macros::Element>> + 'static,
            ) -> Self {
                Self{ builder: self.builder.children_signal(children) }
            }

            fn optional_child_signal(
                self,
                child: impl $crate::macros::Signal<Item = ::std::option::Option<impl Into<$crate::macros::Element>>> + 'static
            ) -> Self {
                Self{ builder: self.builder.optional_child_signal(child) }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! events {
    ($elem_type:ty {
        $($visiblity:vis $name:ident $(- $name_tail:ident)*: $event_type:ty),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            $visiblity fn [<on_ $name $(_ $name_tail)* >] (
                self,
                mut f: impl FnMut($event_type, $elem_type) + 'static
            ) -> Self {
                $crate::macros::ElementBuilder::on(
                    self,
                    $crate::text_name!($name $(- $name_tail)*),
                    move |js_ev| {
                        use $crate::macros::JsCast;
                        // I *think* we can assume event and event.current_target aren't null
                        let event: $event_type = js_ev.unchecked_into();
                        let target: $elem_type =
                            $crate::macros::UnwrapThrowExt::unwrap_throw(
                                event.current_target()
                            )
                            .unchecked_into();
                        f(event, target);
                    }
                )
            }
        )*
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! custom_events {
    ($elem_type:ty {
        $($name:ident $(- $name_tail:ident)*: $event_type:ty),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            pub fn [<on_ $name $(_ $name_tail)* >] (
                self,
                mut f: impl FnMut($event_type, $elem_type) + 'static
            ) -> Self {
                $crate::macros::ElementBuilder::on(
                    self,
                    $crate::text_name!($name $(- $name_tail)*),
                    move |js_ev| {
                        use $crate::macros::JsCast;
                        // I *think* it's safe to assume event and event.current_target aren't null
                        let event: $crate::macros::web_sys::CustomEvent =
                            js_ev.unchecked_into();
                        let target: $elem_type =
                            $crate::macros::UnwrapThrowExt::unwrap_throw(
                                event.current_target()
                            )
                            .unchecked_into();
                        f(event.into(), target);
                    }
                )
            }
        )*
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! attributes {
    ($(
        $(#[$attr_meta:meta])*
        $visibility:vis $attr:ident ($text_attr:expr): $typ:ty
    ),* $(,)? ) => { $crate::macros::paste!{
        $(
            $(#[$attr_meta])*
            #[doc = ""]
            #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-" $attr ")"]
            $visibility fn $attr(self, value: impl $crate::macros::AsAttribute<$typ>) -> Self {
                $crate::macros::ElementBuilder::attribute(self, $text_attr, value)
            }

            $(#[$attr_meta])*
            #[doc = ""]
            #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-" $attr ")"]
            #[allow(clippy::wrong_self_convention)]
            #[allow(non_snake_case)]
            $visibility fn [< $attr _signal >]<T>(
                self,
                value: impl $crate::macros::Signal<Item = T> + 'static
            ) -> Self
            where
                T: $crate::macros::AsAttribute<$typ> + 'static
            {
                $crate::macros::ElementBuilder::attribute_signal(self, $text_attr, value)
            }
        )*
    }};
    ($(
        $(#[$attr_meta:meta])*
        $visibility:vis $attr:ident $($attr_tail:ident)* ($text_attr:expr): $typ:ty
    ),* $(,)? ) => { $crate::macros::paste!{
        $crate::attributes!(
            $(
                $(#[$attr_meta])*
                $visibility [< $attr $(_ $attr_tail)* >] ($text_attr): $typ
            ),*
        );
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_attr {
    ($($name:tt)*) => {
        $crate::macros::intern($crate::naked_text_attr!($($name)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! naked_text_attr {
    (current_time) => {
        "currentTime"
    };
    ($($name:tt)*) => {
        $crate::naked_text_name!($($name)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name{
    ($name:ident $(- $name_tail:ident)*) => {
        $crate::macros::intern($crate::naked_text_name!($name $( - $name_tail)*))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! naked_text_name{
    ($name:ident $(- $name_tail:ident)*) => {
        concat!($crate::stringify_raw!($name) $(, "-", $crate::stringify_raw!($name_tail))*)
    }
}

// TODO: Tests for raw identifiers
#[doc(hidden)]
#[macro_export]
macro_rules! stringify_raw{
    (r#as) => { "as" };
    (r#async) => { "async" };
    (r#await) => { "await" };
    (r#break) => { "break" };
    (r#const) => { "const" };
    (r#continue) => { "continue" };
    (r#dyn) => { "dyn" };
    (r#else) => { "else" };
    (r#enum) => { "enum" };
    (r#extern) => { "extern" };
    (r#false) => { "false" };
    (r#fn) => { "fn" };
    (r#for) => { "for" };
    (r#if) => { "if" };
    (r#impl) => { "impl" };
    (r#in) => { "in" };
    (r#let) => { "let" };
    (r#loop) => { "loop" };
    (r#match) => { "match" };
    (r#mod) => { "mod" };
    (r#move) => { "move" };
    (r#mut) => { "mut" };
    (r#pub) => { "pub" };
    (r#ref) => { "ref" };
    (r#return) => { "return" };
    (r#static) => { "static" };
    (r#struct) => { "struct" };
    (r#trait) => { "trait" };
    (r#true) => { "true" };
    (r#type) => { "type" };
    (r#union) => { "union" };
    (r#unsafe) => { "unsafe" };
    (r#use) => { "use" };
    (r#where) => { "where" };
    (r#while) => { "while" };
    ($name:tt) => { stringify!($name) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! camel_name{
    ($name:ident $($name_tail:ident)*) => { $crate::macros::paste!{
        [< $name:camel $($name_tail:camel)* >]
    }}
}
