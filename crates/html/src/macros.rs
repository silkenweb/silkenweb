pub use silkenweb_dom::{
    tag, AttributeValue, Builder, DomElement, Effect, Element, ElementBuilder, Text,
};
pub use wasm_bindgen::JsCast;

#[macro_export]
macro_rules! html_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident $(- $name_tail:ident)*
        {
            $(
                $(#[$attr_meta:meta])*
                $attr:ident $(- $attr_tail:ident)*: $typ:ty
            ),* $(,)?
        }
    ) => { paste::item!{
        html_element!(
            $(#[$elem_meta])*
            snake ( [< $name $(_ $name_tail)* >] ),
            camel ( [< $name:camel $($name_tail:camel)* >] ),
            text ( text_name!($name $(- $name_tail)*) ),
            {
                $(
                    $(#[$attr_meta])*
                    [< $attr $(_ $attr_tail)*>] ( text_name!($attr $(- $attr_tail)*) ) : $typ
                ),*
            }
        );
    }};
    (
        $(#[$elem_meta:meta])*
        snake ( $snake_name:ident ),
        camel ( $camel_name:ident ),
        text ( $text_name:expr ) $(,)?
        {
            $(
                $(#[$attr_meta:meta])*
                $attr:ident ($text_attr:expr) : $typ:ty
            ),* $(,)?
        }
    ) => {
        paste::item! {
            $(#[$elem_meta])*
            pub fn $snake_name() -> [<$camel_name Builder>] {
                [<$camel_name Builder>]($crate::macros::tag(stringify!($text_name)))
            }

            pub struct [<$camel_name Builder>]($crate::macros::ElementBuilder);

            impl [<$camel_name Builder>] {
                attributes![
                    // TODO: Add all global attrs.
                    id: String,
                    class: String,
                    style: String,
                    $($(#[$attr_meta])* $attr: $typ,)*
                ];
            }

            impl $crate::macros::Builder for [<$camel_name Builder>] {
                type Target = $camel_name;

                fn build(self) -> Self::Target {
                    $camel_name(self.0.build())
                }

                fn into_element(self) -> $crate::macros::Element {
                    self.build().into()
                }
            }

            impl From<[<$camel_name Builder>]> for $crate::macros::Element {
                fn from(builder: [<$camel_name Builder>]) -> Self {
                    use $crate::macros::Builder;
                    builder.build().into()
                }
            }

            impl From<[<$camel_name Builder>]> for $crate::macros::ElementBuilder {
                fn from(builder: [<$camel_name Builder>]) -> Self {
                    builder.0
                }
            }

            #[derive(Clone)]
            pub struct $camel_name($crate::macros::Element);

            impl $crate::macros::Builder for $camel_name {
                type Target = Self;

                fn build(self) -> Self::Target {
                    self
                }

                fn into_element(self) -> $crate::macros::Element {
                    self.build().into()
                }
            }

            impl From<$camel_name> for $crate::macros::Element {
                fn from(html_elem: $camel_name) -> Self {
                    html_elem.0
                }
            }
        }
    };
}

#[macro_export]
macro_rules! dom_type {
    ($name:ident $(- $name_tail:ident)* < $elem_type:ty > $( { $($events:tt)* } )? ) => {
        paste::item! {
            dom_type!(
                camel([<$name:camel $(- $name_tail:camel)* >])
                < $elem_type >
                $( { $($events)* } )?
            );
        }
    };
    (camel($name_camel:ident) < $elem_type:ty > $( { $($events:tt)* } )?) => {
        paste::item! {
            impl [<$name_camel Builder>] {
                html_element_events!($elem_type);
                element_events!($elem_type);

                $( events!($elem_type { $($events)* }); )?

                pub fn effect(self, f: impl $crate::macros::Effect<$elem_type>) -> Self {
                    Self(self.0.effect(f))
                }
            }

            impl $crate::macros::DomElement for [<$name_camel Builder>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    use $crate::macros::JsCast;
                    self.0.dom_element().unchecked_into()
                }
            }

            impl $crate::macros::DomElement for [<$name_camel>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    use $crate::macros::JsCast;
                    self.0.dom_element().unchecked_into()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! children_allowed {
    ($name:ident $(- $name_tail:ident)*) => {
        paste::item! {
            impl [<$name:camel $($name_tail:camel)* Builder>] {
                pub fn text(self, child: impl $crate::macros::Text) -> Self {
                    Self(self.0.text(child))
                }

                pub fn child<Child>(self, c: Child) -> Self
                where
                    Child: Into<$crate::macros::Element>
                {
                    Self(self.0.child(c.into()))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! html_element_events {
    ($elem_type:ty) => {
        events!($elem_type {
            animationend: web_sys::AnimationEvent,
            animationiteration: web_sys::AnimationEvent,
            animationstart: web_sys::AnimationEvent,
            beforeinput: web_sys::InputEvent,
            change: web_sys::Event,
            gotpointercapture: web_sys::PointerEvent,
            input: web_sys::InputEvent,
            lostpointercapture: web_sys::PointerEvent,
            pointercancel: web_sys::PointerEvent,
            pointerdown: web_sys::PointerEvent,
            pointerenter: web_sys::PointerEvent,
            pointerleave: web_sys::PointerEvent,
            pointermove: web_sys::PointerEvent,
            pointerout: web_sys::PointerEvent,
            pointerover: web_sys::PointerEvent,
            pointerup: web_sys::PointerEvent,
            transitionend: web_sys::TransitionEvent,
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! element_events {
    ($elem_type:ty) => {
        events!($elem_type {
            auxclick: web_sys::MouseEvent,
            blur: web_sys::FocusEvent,
            click: web_sys::MouseEvent,
            compositionend: web_sys::CompositionEvent,
            compositionstart: web_sys::CompositionEvent,
            compositionupdate: web_sys::CompositionEvent,
            contextmenu: web_sys::MouseEvent,
            dblclick: web_sys::MouseEvent,
            error: web_sys::Event,
            focusin: web_sys::FocusEvent,
            focusout: web_sys::FocusEvent,
            focus: web_sys::FocusEvent,
            fullscreenchange: web_sys::Event,
            fullscreenerror: web_sys::Event,
            keydown: web_sys::KeyboardEvent,
            keyup: web_sys::KeyboardEvent,
            mousedown: web_sys::MouseEvent,
            mouseenter: web_sys::MouseEvent,
            mouseleave: web_sys::MouseEvent,
            mousemove: web_sys::MouseEvent,
            mouseout: web_sys::MouseEvent,
            mouseover: web_sys::MouseEvent,
            mouseup: web_sys::MouseEvent,
            scroll: web_sys::Event,
            select: web_sys::Event,
            touchcancel: web_sys::TouchEvent,
            touchend: web_sys::TouchEvent,
            touchmove: web_sys::TouchEvent,
            touchstart: web_sys::TouchEvent,
            wheel: web_sys::WheelEvent,
            /* The events are currently marked as unstable in web_sys:
             *
             * copy: web_sys::ClipboardEvent,
             * cut: web_sys::ClipboardEvent,
             * paste: web_sys::ClipboardEvent, */
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! events {
    ($elem_type:ty {
        $($name:ident: $event_type:ty),* $(,)?
    }) => {
        paste::item!{
            $(
                pub fn [<on_ $name >] (
                    self,
                    mut f: impl 'static + FnMut($event_type, $elem_type)
                ) -> Self {
                    Self(self.0.on(stringify!($name), move |js_ev| {
                        use $crate::macros::JsCast;
                        // I *think* it's safe to assume event and event.current_target aren't null
                        let event: $event_type = js_ev.unchecked_into();
                        let target: $elem_type = event.current_target().unwrap().unchecked_into();
                        f(event, target);
                    }))
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
        $attr:ident : $typ:ty
    ),* $(,)? ) => {
        $(
            $(#[$attr_meta])*
            pub fn $attr(self, value: impl $crate::macros::AttributeValue<$typ>) -> Self {
                Self(self.0.attribute(attr_name!($attr), value))
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! attr_name {
    (accept_charset) => {
        "accept-charset"
    };
    (as_) => {
        "as"
    };
    (async_) => {
        "async"
    };
    (for_) => {
        "for"
    };
    (http_equiv) => {
        "http-equiv"
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
    ($name:ident) => {
        stringify!($name)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! text_name{
    ($name:ident $(- $name_tail:ident)*) => {
        concat!(stringify!($name) $(, "-", stringify!($name_tail))*)
    }
}
