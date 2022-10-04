pub use futures_signals::{signal::Signal, signal_vec::SignalVec};
pub use paste::paste;
pub use silkenweb_base::intern_str;
pub use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
pub use web_sys;

// TODO: Update docs for `_`/`-` conversion and explicit text names
// TODO: Convert _ to - in names

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
/// html_element!(my_html_element<web_sys::HtmlDivElement> {
///     attributes {
///         my_attribute: String
///     }
///
///     events {
///         my_event: web_sys::MouseEvent
///     }
///
///     custom_events {
///         my_custom_event: CustomEvent<web_sys::HtmlElement>,
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
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )?
        $($tail:tt)*
    ) => {
        $crate::dom_element!(
            name = $name $( ($text_name) )?,
            attributes = [$crate::elements::HtmlElement],
            events = [$crate::elements::HtmlElementEvents],
            doc = [$(#[$elem_meta])*],
            $($tail)*
        );
    }
}

macro_rules! svg_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )?
        $($tail:tt)*
    ) => {
        $crate::dom_element!(
            name = $name $( ($text_name) )?,
            namespace = Some("http://www.w3.org/2000/svg"),
            attributes = [$crate::elements::svg::attributes::Global],
            events = [],
            doc_macro = svg_element_doc,
            doc = [$(#[$elem_meta])*],
            $($tail)*
        );
    }
}

macro_rules! svg_element_doc {
    ($name:expr) => {
        concat!(
            "The SVG [",
            $name,
            "](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",
            $name,
            ") element"
        )
    };
}

// TODO: Text alternatives for event, custom_event

#[doc(hidden)]
#[macro_export]
macro_rules! dom_element {
    (
        name = $name:ident $( ($text_name: literal) )?,
        $($tail:tt)*
    ) => { $crate::macros::paste!{
        $crate::dom_element!(
            snake ( $name ),
            camel ( [< $name:camel >], [< $name:camel Builder >] ),
            text ( $crate::text_name!($name $( ($text_name) )?) ),
            $($tail)*
        );
    }};
    (
        // TODO: Make names/syntax better (snake_name = )
        snake ( $snake_name:ident ),
        camel ( $camel_name:ident, $camel_builder_name:ident ),
        text ( $text_name:expr ),
        $(namespace = $namespace:expr, )?
        attributes = [$($attribute_trait:ty),*],
        events = [$($event_trait:ty),*],
        $(doc_macro = $doc_macro:ident,)?
        doc = [$($docs:tt)*],
        < $elem_type:ty >
        {
            $(attributes { $(
                $(#[$attr_meta:meta])*
                $attr:ident $( ($text_attr:expr) )? : $typ:ty
            ),* $(,)? } )?

            $(events {
                $(
                    $(#[$event_meta:meta])*
                    $event:ident: $event_type:ty
                ),* $(,)?
            })?

            $(custom_events { $(
                    $(#[$custom_event_meta:meta])*
                    $custom_event:ident: $custom_event_type:ty
                ),* $(,)?
            })?
        }
    ) => {
        $(
            #[doc = $doc_macro!($text_name)]
            #[doc = ""]
        )?
        $($docs)*
        pub fn $snake_name() -> $camel_builder_name {
            $camel_builder_name { builder: $crate::create_element_fn!(
                $($namespace, )? $crate::macros::intern_str($text_name)
            ) }
        }

        pub struct $camel_builder_name {
            builder: $crate::node::element::ElementBuilderBase
        }

        impl $camel_builder_name {
            $crate::attributes![
                $($($(#[$attr_meta])* pub $attr $( ($text_attr) )?: $typ,)*)?
            ];

            $($crate::events!(
                $elem_type {
                    $(
                        $(#[$event_meta])*
                        pub $event: $event_type
                    ),*
                }
            ); )?

            $($crate::custom_events!(
                $elem_type {
                    $(
                        $(#[$custom_event_meta])*
                        $custom_event: $custom_event_type
                    ),*
                }
            ); )?
        }

        impl $crate::node::element::ElementBuilder for $camel_builder_name {
            type Target = $camel_name;
            type DomType = $elem_type;

            fn attribute<T: $crate::attribute::Attribute>(self, name: &str, value: T) -> Self {
                Self{ builder: self.builder.attribute(name, value) }
            }

            fn attribute_signal<T: $crate::attribute::Attribute + 'static>(
                self,
                name: &str,
                value: impl $crate::macros::Signal<Item = T> + 'static,
            ) -> Self {
                Self{ builder: $crate::node::element::ElementBuilder::attribute_signal(self.builder, name, value) }
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

            fn handle(&self) -> $crate::node::element::ElementHandle<Self::DomType> {
                self.builder.handle().cast()
            }

            fn spawn_future(self, future: impl ::std::future::Future<Output = ()> + 'static) -> Self {
                Self{ builder: self.builder.spawn_future(future) }
            }

            fn on(
                self,
                name: &'static str,
                f: impl FnMut($crate::macros::JsValue) + 'static
            ) -> Self {
                Self{ builder: $crate::node::element::ElementBuilder::on(self.builder, name, f) }
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

        impl From<$camel_builder_name> for $crate::node::element::Element {
            fn from(builder: $camel_builder_name) -> Self {
                $crate::node::element::ElementBuilder::build(builder).into()
            }
        }

        impl From<$camel_builder_name> for $crate::node::Node {
            fn from(builder: $camel_builder_name) -> Self {
                $crate::node::element::ElementBuilder::build(builder).into()
            }
        }

        pub struct $camel_name($crate::node::element::Element);

        impl From<$camel_name> for $crate::node::element::Element {
            fn from(html_elem: $camel_name) -> Self {
                html_elem.0
            }
        }

        impl From<$camel_name> for $crate::node::Node {
            fn from(html_elem: $camel_name) -> Self {
                html_elem.0.into()
            }
        }

        $(impl $attribute_trait for $camel_builder_name {})*

        $(
            impl $event_trait for $camel_builder_name {}
        )*

        impl $crate::elements::ElementEvents for $camel_builder_name {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! create_element_fn {
    ($text_name:expr) => {
        $crate::node::element::tag($text_name)
    };
    ($namespace:expr, $text_name:expr) => {
        $crate::node::element::tag_in_namespace($namespace, $text_name)
    };
}

/// Add `child` and `text` methods to an html element builder.
///
/// See [`html_element`] for a complete example of defining an html element.
#[macro_export]
macro_rules! parent_element {
    ($name:ident) => {$crate::macros::paste!{
        impl $crate::node::element::ParentBuilder for
            [< $name:camel Builder >]
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

            fn child(self, c: impl Into<$crate::node::Node>) -> Self
            {
                Self{ builder: self.builder.child(c) }
            }

            fn child_signal(
                self,
                children: impl $crate::macros::Signal<Item = impl Into<$crate::node::Node>> + 'static,
            ) -> Self {
                Self{ builder: self.builder.child_signal(children) }
            }

            fn optional_child_signal(
                self,
                children: impl $crate::macros::Signal<Item = ::std::option::Option<impl Into<$crate::node::Node>>> + 'static,
            ) -> Self {
                Self{ builder: self.builder.optional_child_signal(children) }
            }

            fn children_signal(
                self,
                children: impl $crate::macros::SignalVec<Item = impl Into<$crate::node::Node>> + 'static,
            ) -> Self::Target {
                [< $name:camel >] (self.builder.children_signal(children))
            }
        }
    }};
}

/// Implement `ShadowRootParentBuilder` for the HTML element
#[macro_export]
macro_rules! shadow_parent_element {
    ($name:ident) => {
        $crate::macros::paste! {
            impl $crate::node::element::ShadowRootParentBuilder for
                [< $name:camel Builder >]
            {
                fn attach_shadow_children(
                    self,
                    children: impl IntoIterator<Item = impl Into<$crate::node::Node>> + 'static
                ) -> Self::Target {
                    [< $name:camel >] (self.builder.attach_shadow_children(children))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! events {
    ($elem_type:ty {
        $(
            $(#[$event_meta:meta])*
            $visiblity:vis $name:ident: $event_type:ty
        ),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            $(#[$event_meta])*
            $visiblity fn [<on_ $name >] (
                self,
                mut f: impl FnMut($event_type, $elem_type) + 'static
            ) -> Self {
                $crate::node::element::ElementBuilder::on(
                    self,
                    $crate::text_name_intern!($name),
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
            $name:ident: $event_type:ty
        ),* $(,)?
    }) => { $crate::macros::paste!{
        $(
            $(#[$event_meta])*
            pub fn [<on_ $name>] (
                self,
                mut f: impl FnMut($event_type, $elem_type) + 'static
            ) -> Self {
                $crate::node::element::ElementBuilder::on(
                    self,
                    $crate::text_name_intern!($name),
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
        $visibility:vis $attr:ident $(($text_attr:expr))? : $typ:ty
    ),* $(,)? ) => {
        $(
            $crate::attribute!(
                $(#[$attr_meta])*
                $visibility $attr $(($text_attr))?: $typ
            );
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! attribute {
    (
        $(#[$attr_meta:meta])*
        $visibility:vis $attr:ident $( ($text_attr:expr) )?: $typ:ty
    ) => { $crate::macros::paste!{
        $crate::attribute!(
            $(#[$attr_meta])*
            $visibility ( $attr, [< $attr _signal >] ) ($crate::text_name_intern!($attr $( ($text_attr) )?)): $typ
        );
    }};
    (
        $(#[$attr_meta:meta])*
        $visibility:vis ( $attr:ident, $attr_signal:ident ) ($text_attr:expr): $typ:ty
    ) => {
        $(#[$attr_meta])*
        $visibility fn $attr(self, value: impl $crate::attribute::AsAttribute<$typ>) -> Self {
            $crate::node::element::ElementBuilder::attribute(self, $text_attr, value)
        }

        $(#[$attr_meta])*
        #[allow(clippy::wrong_self_convention)]
        #[allow(non_snake_case)]
        $visibility fn $attr_signal<T>(
            self,
            value: impl $crate::macros::Signal<Item = T> + 'static
        ) -> Self
        where
            T: $crate::attribute::AsAttribute<$typ> + 'static
        {
            $crate::node::element::ElementBuilder::attribute_signal(self, $text_attr, value)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name_intern {
    ($($name:tt)*) => {
        $crate::macros::intern_str($crate::text_name!($($name)*))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name {
    ($name:ident($text_name:literal)) => {
        $text_name
    };
    ($name:ident) => {
        $crate::stringify_raw!($name)
    };
}

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
