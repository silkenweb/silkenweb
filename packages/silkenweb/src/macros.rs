pub use futures_signals::{signal::Signal, signal_vec::SignalVec};
pub use paste::paste;
pub use silkenweb_base::intern_str;
pub use silkenweb_macros::rust_to_html_ident;
pub use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
pub use web_sys;

/// Define a custom html element.
///
/// This will define a builder struct for an html element, with a method for
/// each attribute. It will also define a struct for the built element.
/// Underscores are converted to dashes in element, attribute, and event names,
/// and raw identifiers have their `r#` prefix stripped.
///
/// The html identifier can be explicitly specified in brackets after the
/// element or attribute name. See `my_explicitly_named_attribute` in the
/// example.
///
/// # Example
///
/// ```no_run
/// # use silkenweb::custom_html_element;
/// use silkenweb::elements::CustomEvent;
///
/// // The types of the dom element and event carry through to the event handler.
/// custom_html_element!(my_html_element = {
///     dom_type: web_sys::HtmlDivElement;
///     attributes {
///         my_attribute: String,
///         my_explicitly_named_attribute("MyExplicitlyNamedAttribute"): String
///     };
///
///     events {
///         my_event: web_sys::MouseEvent
///     };
///
///     custom_events {
///         my_custom_event: CustomEvent<web_sys::HtmlElement>,
///     };
/// });
///
/// let elem = my_html_element()
///     .my_attribute("attribute-value")
///     .on_my_event(|event: web_sys::MouseEvent, target: web_sys::HtmlDivElement| {})
///     .on_my_custom_event(|event: CustomEvent<web_sys::HtmlElement>, target: web_sys::HtmlDivElement| {});
/// ```
#[macro_export]
macro_rules! custom_html_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )? = {
            $($tail:tt)*
        }
    ) => {
        $crate::dom_element!(
            $(#[$elem_meta])*
            $name $( ($text_name) )? = {
                common_attributes = [$crate::elements::HtmlElement, $crate::elements::AriaElement];
                common_events = [$crate::elements::HtmlElementEvents];
                $($tail)*
            }
        );
    }
}

macro_rules! html_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )? = {
            $($tail:tt)*
        }
    ) => {
        $crate::dom_element!(
            $(#[$elem_meta])*
            $name $( ($text_name) )? = {
                common_attributes = [$crate::elements::HtmlElement, $crate::elements::AriaElement];
                common_events = [$crate::elements::HtmlElementEvents];
                doc_macro = html_element_doc;
                attribute_doc_macro = html_attribute_doc;
                $($tail)*
            }
        );
    }
}

macro_rules! html_element_doc {
    ($name:expr) => {
        concat!(
            "The HTML [",
            $name,
            "](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",
            $name,
            ") element"
        )
    };
}

macro_rules! html_attribute_doc {
    ($element:expr, $name:expr) => {
        concat!(
            "The [`",
            $name,
            "`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",
            $element,
            "#attr-",
            $name,
            ") attribute"
        )
    };
}

macro_rules! svg_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )? = {
            $($tail:tt)*
        }
    ) => {
        $crate::dom_element!(
            $(#[$elem_meta])*
            $name $( ($text_name) )? = {
                common_attributes = [$crate::elements::svg::attributes::Global, $crate::elements::AriaElement];
                common_events = [];
                doc_macro = svg_element_doc;
                attribute_doc_macro = svg_attribute_doc;
                namespace = Some("http://www.w3.org/2000/svg");
                $($tail)*
            }
        );
    }
}

macro_rules! svg_element_doc {
    ($name:expr) => {
        concat!(
            "The SVG [`",
            $name,
            "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",
            $name,
            ") element"
        )
    };
}

macro_rules! svg_attribute_doc {
    ($element:expr, $name:expr) => {
        concat!(
            "The SVG [`",
            $name,
            "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/",
            $name,
            ") attribute"
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! dom_element {
    (
        $(#[$elem_meta:meta])*
        $snake_name:ident ($text_name:expr) = {
            camel_name = $camel_name:ident;
            camel_builder_name = $camel_builder_name:ident;
            common_attributes = [$($attribute_trait:ty),*];
            common_events = [$($event_trait:ty),*];
            $(doc_macro = $doc_macro:ident;)?
            $(attribute_doc_macro = $attr_doc_macro:ident;)?
            $(namespace = $namespace:expr; )?
            dom_type: $elem_type:ty;

            $(attributes { $(
                $(#[$attr_meta:meta])*
                $attr:ident $( ($text_attr:expr) )? : $typ:ty
            ),* $(,)? }; )?

            $(events {
                $(
                    $(#[$event_meta:meta])*
                    $event:ident: $event_type:ty
                ),* $(,)?
            };)?

            $(custom_events { $(
                    $(#[$custom_event_meta:meta])*
                    $custom_event:ident: $custom_event_type:ty
                ),* $(,)?
            };)?
        }
    ) => {
        $(
            #[doc = $doc_macro!($text_name)]
            #[doc = ""]
        )?
        $(#[$elem_meta])*
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
                $([
                        attribute_parent = $text_name,
                        attribute_doc_macro = $attr_doc_macro
                ])?
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

            fn class<'a, T>(self, class: impl $crate::node::element::SignalOrValue<'a, Item = T>) -> Self
            where
                T: 'a + AsRef<str> {
                        Self { builder: self.builder.class(class) }
            }

            fn classes(self, value: impl $crate::node::element::UpdateClasses) -> Self {
                Self { builder: self.builder.classes(value) }
            }

            fn attribute<T: $crate::node::element::SetAttribute>(self, name: &str, value: T) -> Self {
                Self{ builder: self.builder.attribute(name, value) }
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

        impl From<$camel_builder_name> for $crate::node::element::ElementBuilderBase {
            fn from(builder: $camel_builder_name) -> Self {
                builder.builder
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

        impl $camel_name {
            pub fn handle(&self) -> $crate::node::element::ElementHandle<$elem_type> {
                self.0.handle().cast()
            }
        }

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
    (
        $(#[$elem_meta:meta])*
        $name:ident $( ($text_name: literal) )? = {
            common_attributes $($tail:tt)*
        }
    ) => { $crate::macros::paste!{
        $crate::dom_element!(
            $(#[$elem_meta])*
            $name($crate::text_name!($name $( ($text_name) )?)) = {
                camel_name = [< $name:camel >];
                camel_builder_name = [< $name:camel Builder >];
                common_attributes $($tail)*
            }
        );
    }};
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
/// See [`custom_html_element`] for a complete example of defining an html
/// element.
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
    (
        [
            attribute_parent = $element:expr,
            attribute_doc_macro = $attr_doc_macro:ident
        ]
        $(
            $(#[$attr_meta:meta])*
            $visibility:vis $attr:ident $(($text_attr:expr))? : $typ:ty
        ),* $(,)?
     ) => {
        $(
            $crate::attribute!(
                [
                    attribute_parent = $element,
                    attribute_doc_macro = $attr_doc_macro
                ]
                $(#[$attr_meta])*
                $visibility $attr $(($text_attr))?: $typ
            );
        )*
    };
    (
        $(
            $(#[$attr_meta:meta])*
            $visibility:vis $attr:ident $(($text_attr:expr))? : $typ:ty
        ),* $(,)?
     ) => {
        $(
            $crate::attribute!(
                []
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
        [
            $(
                attribute_parent = $element:expr,
                attribute_doc_macro = $attr_doc_macro:ident
            )?
        ]
        $(#[$attr_meta:meta])*
        $visibility:vis $attr:ident : $typ:ty
    ) => {
        $crate::attribute!(
            [
                $(
                    attribute_parent = $element,
                    attribute_doc_macro = $attr_doc_macro
                )?
            ]
                $(#[$attr_meta])*
            $visibility $attr ($crate::macros::rust_to_html_ident!($attr)): $typ
        );
    };
    (
        [
            $(
                attribute_parent = $element:expr,
                attribute_doc_macro = $attr_doc_macro:ident
            )?
        ]
        $(#[$attr_meta:meta])*
        $visibility:vis $attr:ident ($text_attr:expr): $typ:ty
    ) => {
        $(
            #[doc = $attr_doc_macro!($element, $text_attr)]
            #[doc = ""]
        )?
        $(#[$attr_meta])*
        $visibility fn $attr<T>(self, value: T) -> Self
        where
            T: $crate::node::element::SetAttribute,
            T::Type: $crate::attribute::AsAttribute<$typ>
        {
            $crate::node::element::ElementBuilder::attribute(self, $crate::macros::intern_str($text_attr), value)
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
        $crate::macros::rust_to_html_ident!($name)
    };
}
