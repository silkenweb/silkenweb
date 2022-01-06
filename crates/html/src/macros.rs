#[doc(hidden)]
// Macro dependencies
pub mod private {
    pub use futures_signals::{signal::Signal, signal_vec::SignalVec};
    pub use paste::paste;
    pub use silkenweb_dom::{
        tag, Attribute, Builder, DomElement, Element, ElementBuilder, StaticAttribute,
    };
    pub use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
    pub use web_sys as dom;
}

/// Define an html element.
///
/// This will define a builder struct for an html element, with a method for
/// each attribute. It will also define a struct for the built element. Dashes
/// are allowed in element, attribute, and event names. They will be converted
/// to underscores when generating rust identifiers. For example:
///
/// ```no_run
/// # use silkenweb_html::html_element;
/// use silkenweb_html::CustomEvent;
/// use web_sys as dom;
///
/// // The types of the dom element and event carry through to the event handler.
/// html_element!(my-html-element<dom::HtmlDivElement> {
///     attributes {
///         my-attribute: String
///     }
///
///     events {
///         my-event: dom::MouseEvent
///     }
///
///     custom_events {
///         my-custom-event: CustomEvent<dom::HtmlElement>,
///     }
/// });
///
/// let elem = my_html_element()
///     .my_attribute("attribute-value")
///     .on_my_event(|event: dom::MouseEvent, target: dom::HtmlDivElement| {})
///     .on_my_custom_event(|event: CustomEvent<dom::HtmlElement>, target: dom::HtmlDivElement| {});
/// ```
#[macro_export]
macro_rules! html_element {
    (
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
    ) => { $crate::macros::private::paste!{
        html_element!(
            $(#[$elem_meta])*
            snake ( [< $name $(_ $name_tail)* >] ),
            camel ( [< $name:camel $($name_tail:camel)* >] ),
            text ( $crate::text_name!($name $(- $name_tail)*) )
            < $elem_type >
            {
                $(attributes { $(
                    $(#[$attr_meta])*
                    [< $attr $(_ $attr_tail)*>] ( $crate::text_attr!($attr $(- $attr_tail)*) ) : $typ
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
        $(#[$elem_meta:meta])*
        snake ( $snake_name:ident ),
        camel ( $camel_name:ident ),
        text ( $text_name:expr )
        < $elem_type:ty >
        {
            $(attributes { $(
                $(#[$attr_meta:meta])*
                $attr:ident ($text_attr:expr) : $typ:ty
            ),* $(,)? } )?

            $(events {
                $($event:ident $(- $event_tail:ident)*: $event_type:ty),* $(,)?
            })?

            $(custom_events {
                $($custom_event:ident $(- $custom_event_tail:ident)*: $custom_event_type:ty),* $(,)?
            })?
        }
    ) => {
        $crate::macros::private::paste! {
            $(#[$elem_meta])*
            pub fn $snake_name() -> [<$camel_name Builder>] {
                [<$camel_name Builder>]{ builder: $crate::macros::private::tag($text_name) }
            }

            pub struct [<$camel_name Builder>]{
                builder: $crate::macros::private::ElementBuilder
            }

            impl [<$camel_name Builder>] {
                $crate::attributes![
                    $($($(#[$attr_meta])* $attr ($text_attr): $typ,)*)?
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

            impl $crate::HtmlElementEvents for [<$camel_name Builder>] {
                type EventTarget = $elem_type;
            }

            impl $crate::ElementEvents for [<$camel_name Builder>] {
                type EventTarget = $elem_type;
            }

            impl $crate::Effects<$elem_type> for [<$camel_name Builder>] {
                fn effect(self, f: impl 'static + ::std::ops::FnOnce(&$elem_type)) -> Self {
                    Self{ builder: self.builder.effect(f) }
                }

                fn effect_signal<T: 'static>(
                    self,
                    sig: impl 'static + $crate::macros::private::Signal<Item = T>,
                    f: impl 'static + Clone + Fn(&$elem_type, T),
                ) -> Self {
                    Self{ builder: self.builder.effect_signal(sig, f) }
                }
            }

            impl $crate::macros::private::Builder for [<$camel_name Builder>] {
                type Target = $camel_name;

                fn attribute<T>(
                    self,
                    name: &str,
                    value: impl $crate::macros::private::Attribute<T>,
                ) -> Self {
                    Self{ builder: $crate::macros::private::Builder::attribute(self.builder, name, value) }
                }

                fn on(
                    self,
                    name: &'static str,
                    f: impl 'static + FnMut($crate::macros::private::JsValue)
                ) -> Self {
                    Self{ builder: $crate::macros::private::Builder::on(self.builder, name, f) }
                }

                fn build(self) -> Self::Target {
                    $camel_name(self.builder.build())
                }

                fn into_element(self) -> $crate::macros::private::Element {
                    self.build().into()
                }
            }

            impl $crate::macros::private::DomElement for [<$camel_name Builder>] {
                type Target = $elem_type;

                fn dom_element(&self) -> &Self::Target {
                    use $crate::macros::private::JsCast;
                    self.builder.dom_element().unchecked_ref()
                }
            }

            impl From<[<$camel_name Builder>]> for $crate::macros::private::Element {
                fn from(builder: [<$camel_name Builder>]) -> Self {
                    use $crate::macros::private::Builder;
                    builder.build().into()
                }
            }

            impl From<[<$camel_name Builder>]> for $crate::macros::private::ElementBuilder {
                fn from(builder: [<$camel_name Builder>]) -> Self {
                    builder.builder
                }
            }

            pub struct $camel_name($crate::macros::private::Element);

            impl $crate::HtmlElement for [<$camel_name Builder>] {}

            impl $crate::macros::private::DomElement for [<$camel_name>] {
                type Target = $elem_type;

                fn dom_element(&self) -> &Self::Target {
                    use $crate::macros::private::JsCast;
                    self.0.dom_element().unchecked_ref()
                }
            }

            impl From<$camel_name> for $crate::macros::private::Element {
                fn from(html_elem: $camel_name) -> Self {
                    html_elem.0
                }
            }
        }
    };
}

/// Add `child` and `text` methods to an html element builder.
///
/// See [`html_element`] for a complete example of defining an html element.
#[macro_export]
macro_rules! children_allowed {
    ($name:ident $(- $name_tail:ident)*) => {
        $crate::macros::private::paste! {
            impl $crate::ParentBuilder for [<$name:camel $($name_tail:camel)* Builder>] {
                fn text(self, child: &str) -> Self {
                    Self{ builder: self.builder.text(child) }
                }

                fn text_signal(self, child: impl 'static + $crate::macros::private::Signal<Item = impl Into<String>>) -> Self {
                    Self{ builder: self.builder.text_signal(child) }
                }

                fn child<Child>(self, c: Child) -> Self
                where
                    Child: Into<$crate::macros::private::Element>
                {
                    Self{ builder: self.builder.child(c.into()) }
                }

                fn child_signal(self, child: impl 'static + $crate::macros::private::Signal<Item = impl Into<$crate::macros::private::Element>>) -> Self {
                    Self{ builder: self.builder.child_signal(child) }
                }

                fn optional_child_signal(
                    self,
                    child: impl 'static + $crate::macros::private::Signal<Item = ::std::option::Option<impl Into<$crate::macros::private::Element>>>
                ) -> Self {
                    Self{ builder: self.builder.optional_child_signal(child) }
                }

                fn children_signal(
                    self,
                    children: impl 'static + $crate::macros::private::SignalVec<Item = impl Into<$crate::macros::private::Element>>,
                ) -> Self {
                    Self{ builder: self.builder.children_signal(children) }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! events {
    ($elem_type:ty {
        $($visiblity:vis $name:ident $(- $name_tail:ident)*: $event_type:ty),* $(,)?
    }) => {
        $crate::macros::private::paste!{
            $(
                $visiblity fn [<on_ $name $(_ $name_tail)* >] (
                    self,
                    mut f: impl 'static + FnMut($event_type, $elem_type)
                ) -> Self {
                    $crate::macros::private::Builder::on(
                        self,
                        $crate::text_name!($name $(- $name_tail)*),
                        move |js_ev| {
                            use $crate::macros::private::JsCast;
                            // I *think* we can assume event and event.current_target aren't null
                            let event: $event_type = js_ev.unchecked_into();
                            let target: $elem_type =
                                $crate::macros::private::UnwrapThrowExt::unwrap_throw(
                                    event.current_target()
                                )
                                .unchecked_into();
                            f(event, target);
                        }
                    )
                }
            )*
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! custom_events {
    ($elem_type:ty {
        $($name:ident $(- $name_tail:ident)*: $event_type:ty),* $(,)?
    }) => {
        $crate::macros::private::paste!{
            $(
                pub fn [<on_ $name $(_ $name_tail)* >] (
                    self,
                    mut f: impl 'static + FnMut($event_type, $elem_type)
                ) -> Self {
                    $crate::macros::private::Builder::on(
                        self,
                        $crate::text_name!($name $(- $name_tail)*),
                        move |js_ev| {
                            use $crate::macros::private::JsCast;
                            // I *think* it's safe to assume event and event.current_target aren't null
                            let event: $crate::macros::private::dom::CustomEvent =
                                js_ev.unchecked_into();
                            let target: $elem_type =
                                $crate::macros::private::UnwrapThrowExt::unwrap_throw(
                                    event.current_target()
                                )
                                .unchecked_into();
                            f(event.into(), target);
                        }
                    )
                }
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! attributes {
    ($(
        $(#[$attr_meta:meta])*
        $attr:ident ($text_attr:expr): $typ:ty
    ),* $(,)? ) => {
        $(
            $(#[$attr_meta])*
            pub fn $attr(self, value: impl $crate::macros::private::Attribute<$typ>) -> Self {
                Self{ builder: $crate::macros::private::Builder::attribute(self.builder, $text_attr, value) }
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_attr {
    (as_) => {
        "as"
    };
    (async_) => {
        "async"
    };
    (for_) => {
        "for"
    };
    (current_time) => {
        "currentTime"
    };
    (loop_) => {
        "loop"
    };
    (type_) => {
        "type"
    };
    ($($name:tt)*) => {
        $crate::text_name!($($name)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name{
    ($name:ident $(- $name_tail:ident)*) => {
        concat!(stringify!($name) $(, "-", stringify!($name_tail))*)
    }
}
