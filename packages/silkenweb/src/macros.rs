pub use futures_signals::{signal::Signal, signal_vec::SignalVec};
pub use paste::paste;
pub use silkenweb_base::intern_str;
pub use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
pub use web_sys;

pub use crate::{
    attribute::{AsAttribute, Attribute},
    hydration::Wet,
    node::{
        element::{
            tag, tag_in_namespace, Element, ElementBuilder, ElementBuilderBase, OptionalChildren,
            ParentBuilder,
        },
        Node, NodeImpl,
    },
};

/// Define an html element.
///
/// This will define a builder struct for an html element, with a method for
/// each attribute. It will also define a struct for the built element. Dashes
/// are allowed in element, attribute, and event names. They will be converted
/// to underscores when generating rust identifiers. For example:
///
/// ```no_run
/// # use silkenweb::html_element;
/// use silkenweb::elements::CustomEvent;
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
            attributes = [$crate::elements::HtmlElement],
            events = [$crate::elements::HtmlElementEvents],
            $($t)*
        );
    }
}

macro_rules! svg_element {
    ($($t:tt)*) => {
        $crate::dom_element!(
            namespace = Some("http://www.w3.org/2000/svg"),
            attributes = [],
            events = [],
            $($t)*
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! dom_element {
    (
        $(namespace = $namespace:expr, )?
        attributes = [$($attribute_trait:ty),*],
        events = [$($event_trait:ty),*],
        $(#[$elem_meta:meta])*
        $name:ident $(- $name_tail:tt)*
        < $elem_type:ty >
        {
            $(attributes {
                $(
                    $(#[$attr_meta:meta])*
                    $attr:ident $(- $attr_tail:tt)*: $typ:ty
                ),* $(,)?
            })?

            $(events {
                $(#[$event_meta:meta])*
                $($event:ident $(- $event_tail:tt)*: $event_type:ty),* $(,)?
            })?

            $(custom_events {
                $(#[$custom_event_meta:meta])*
                $($custom_event:ident $(- $custom_event_tail:tt)*: $custom_event_type:ty),* $(,)?
            })?
        }
    ) => { $crate::macros::paste!{
        $crate::dom_element!(
            $(namespace = $namespace, )?
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
                    $(#[$event_meta])*
                    $($event $(- $event_tail)*: $event_type),*
                })?

                $(custom_events {
                    $(#[$custom_event_meta])*
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
                $(
                    $(#[$event_meta:meta])*
                    $event:ident $(- $event_tail:tt)*: $event_type:ty
                ),* $(,)?
            })?

            $(custom_events { $(
                    $(#[$custom_event_meta:meta])*
                    $custom_event:ident $(- $custom_event_tail:tt)*: $custom_event_type:ty
                ),* $(,)?
            })?
        }
    ) => {
        $(#[$elem_meta])*
        pub fn $snake_name() -> $camel_builder_name {
            $camel_builder_name { builder: $crate::create_element_fn!(
                $($namespace, )? $text_name
            ) }
        }

        pub struct $camel_builder_name<Impl: $crate::macros::NodeImpl = $crate::macros::Wet> {
            builder: $crate::macros::ElementBuilderBase<Impl>
        }

        impl $camel_builder_name {
            $crate::attributes![
                $($($(#[$attr_meta])* pub $attr $($attr_tail)* ($text_attr): $typ,)*)?
            ];

            $($crate::events!(
                $elem_type {
                    $(
                        $(#[$event_meta])*
                        pub $event $(- $event_tail)*: $event_type
                    ),*
                }
            ); )?

            $($crate::custom_events!(
                $elem_type {
                    $(
                        $(#[$custom_event_meta])*
                        $custom_event $(- $custom_event_tail)*: $custom_event_type
                    ),*
                }
            ); )?
        }

        impl<Impl: $crate::macros::NodeImpl> $crate::macros::ElementBuilder for $camel_builder_name<Impl> {
            type Target = $camel_name<Impl>;
            type DomType = $elem_type;

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

            fn effect(self, f: impl ::std::ops::FnOnce(&Self::DomType) + 'static) -> Self {
                Self {
                    builder: self.builder.effect(|elem| {
                        f($crate::macros::UnwrapThrowExt::unwrap_throw($crate::macros::JsCast::dyn_ref(elem)))
                    })
                }
            }

            fn effect_signal<T: 'static>(
                self,
                sig: impl $crate::macros::Signal<Item = T> + 'static,
                f: impl Fn(&Self::DomType, T) + Clone + 'static,
            ) -> Self {
                Self{
                    builder: self.builder.effect_signal(
                        sig,
                        move |elem, signal| {
                            f(
                                $crate::macros::UnwrapThrowExt::unwrap_throw($crate::macros::JsCast::dyn_ref(elem)),
                                signal,
                            )
                        }
                    )
                }
            }

            fn spawn_future(self, future: impl ::std::future::Future<Output = ()> + 'static) -> Self {
                Self{ builder: self.builder.spawn_future(future) }
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
        }

        impl ::std::fmt::Display for $camel_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        // TODO: Macro code formatting
        impl<Impl: $crate::macros::NodeImpl> From<$camel_builder_name<Impl>> for $crate::macros::Element<Impl> {
            fn from(builder: $camel_builder_name<Impl>) -> Self {
                $crate::macros::ElementBuilder::build(builder).into()
            }
        }

        impl<Impl: $crate::macros::NodeImpl> From<$camel_builder_name<Impl>> for $crate::macros::Node<Impl> {
            fn from(builder: $camel_builder_name<Impl>) -> Self {
                $crate::macros::ElementBuilder::build(builder).into()
            }
        }

        pub struct $camel_name<Impl: $crate::macros::NodeImpl = $crate::macros::Wet>($crate::macros::Element<Impl>);

        impl<Impl: $crate::macros::NodeImpl> From<$camel_name<Impl>> for $crate::macros::Element<Impl> {
            fn from(html_elem: $camel_name<Impl>) -> Self {
                html_elem.0
            }
        }

        impl<Impl: $crate::macros::NodeImpl>  From<$camel_name<Impl>> for $crate::macros::Node<Impl> {
            fn from(html_elem: $camel_name<Impl>) -> Self {
                html_elem.0.into()
            }
        }

        $(impl<Impl: $crate::macros::NodeImpl>  $attribute_trait for $camel_builder_name<Impl> {})*

        $(
            impl<Impl: $crate::macros::NodeImpl>  $event_trait for $camel_builder_name<Impl> {
                type EventTarget = $elem_type;
            }
        )*

        impl $crate::elements::ElementEvents for $camel_builder_name {
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
    ($name:ident $(- $name_tail:ident)*) => {$crate::macros::paste!{
        impl<Impl: $crate::macros::NodeImpl> $crate::macros::ParentBuilder<Impl> for
            [< $name:camel $($name_tail:camel)* Builder >]<Impl>
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

            fn child(self, c: impl Into<$crate::macros::Node<Impl>>) -> Self
            {
                Self{ builder: self.builder.child(c) }
            }

            fn child_signal(
                self,
                children: impl $crate::macros::Signal<Item = impl Into<$crate::macros::Node<Impl>>> + 'static,
            ) -> Self::Target {
                [< $name:camel $($name_tail:camel)* >] (self.builder.child_signal(children))
            }

            fn children_signal(
                self,
                children: impl $crate::macros::SignalVec<Item = impl Into<$crate::macros::Node<Impl>>> + 'static,
            ) -> Self::Target {
                [< $name:camel $($name_tail:camel)* >] (self.builder.children_signal(children))
            }

            fn optional_children(self, children: $crate::macros::OptionalChildren<Impl>) -> Self::Target {
                [< $name:camel $($name_tail:camel)* >] (self.builder.optional_children(children))
            }
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! events {
    ($elem_type:ty {
        $(
            $(#[$event_meta:meta])*
            $visiblity:vis $name:ident $(- $name_tail:tt)*: $event_type:ty
        ),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            $(#[$event_meta])*
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
        $(
            $(#[$event_meta:meta])*
            $name:ident $(- $name_tail:tt)*: $event_type:ty
        ),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            $(#[$event_meta])*
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
            $visibility fn $attr(self, value: impl $crate::macros::AsAttribute<$typ>) -> Self {
                $crate::macros::ElementBuilder::attribute(self, $text_attr, value)
            }

            $(#[$attr_meta])*
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
    (current_time) => {
        $crate::text_name!(currentTime)
    };
    ($($name:tt)*) => {
        $crate::text_name!($($name)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name{
    ($name:ident $(- $name_tail:tt)*) => {
        $crate::macros::intern_str($crate::naked_text_name!($name $( - $name_tail)*))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! naked_text_name{
    ($name:ident $(- $name_tail:tt)*) => {
        concat!($crate::stringify_raw!($name) $(, "-", $crate::stringify_raw!($name_tail))*)
    }
}

// TODO: Tests for raw identifiers
#[doc(hidden)]
#[macro_export]
macro_rules! stringify_raw {
    (r#as) => {
        "as"
    };
    (r#async) => {
        "async"
    };
    (r#await) => {
        "await"
    };
    (r#break) => {
        "break"
    };
    (r#const) => {
        "const"
    };
    (r#continue) => {
        "continue"
    };
    (r#dyn) => {
        "dyn"
    };
    (r#else) => {
        "else"
    };
    (r#enum) => {
        "enum"
    };
    (r#extern) => {
        "extern"
    };
    (r#false) => {
        "false"
    };
    (r#fn) => {
        "fn"
    };
    (r#for) => {
        "for"
    };
    (r#if) => {
        "if"
    };
    (r#impl) => {
        "impl"
    };
    (r#in) => {
        "in"
    };
    (r#let) => {
        "let"
    };
    (r#loop) => {
        "loop"
    };
    (r#match) => {
        "match"
    };
    (r#mod) => {
        "mod"
    };
    (r#move) => {
        "move"
    };
    (r#mut) => {
        "mut"
    };
    (r#pub) => {
        "pub"
    };
    (r#ref) => {
        "ref"
    };
    (r#return) => {
        "return"
    };
    (r#static) => {
        "static"
    };
    (r#struct) => {
        "struct"
    };
    (r#trait) => {
        "trait"
    };
    (r#true) => {
        "true"
    };
    (r#type) => {
        "type"
    };
    (r#union) => {
        "union"
    };
    (r#unsafe) => {
        "unsafe"
    };
    (r#use) => {
        "use"
    };
    (r#where) => {
        "where"
    };
    (r#while) => {
        "while"
    };
    ($name:tt) => {
        stringify!($name)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! camel_name{
    ($name:ident $($name_tail:tt)*) => { $crate::macros::paste!{
        [< $name:camel $($name_tail:camel)* >]
    }}
}
