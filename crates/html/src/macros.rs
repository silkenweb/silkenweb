pub use surfinia_core::{tag, AttributeValue, Builder, DomElement, Element, ElementBuilder};
pub use wasm_bindgen::JsCast;

macro_rules! attr_name {
    (for_) => {
        "for"
    };
    (type_) => {
        "type"
    };
    ($name:ident) => {
        stringify!($name)
    };
}

macro_rules! attributes {
    ($($attr:ident : $typ:ty),* $(,)?) => {
        $(
            pub fn $attr(self, value: impl $crate::macros::AttributeValue<$typ>) -> Self {
                Self(self.0.attribute(attr_name!($attr), value))
            }
        )*
    };
}

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
                        // TODO: Export from crate
                        use $crate::macros::JsCast;
                        let event: $event_type = js_ev.unchecked_into();
                        // TODO: Is it safe to unwrap here?
                        let target: $elem_type = event.target().unwrap().unchecked_into();
                        f(event, target);
                    }))
                }
            )*
        }
    };
}

macro_rules! html_events {
    ($elem_type:ty) => {
        events!($elem_type {
            blur: web_sys::FocusEvent,
            click: web_sys::MouseEvent,
            dblclick: web_sys::MouseEvent,
            focusout: web_sys::FocusEvent,
            input: web_sys::InputEvent,
            keydown: web_sys::KeyboardEvent,
            keyup: web_sys::KeyboardEvent,
        });
    };
}

macro_rules! dom_type {
    ($name:ident < $elem_type:ty >) => {
        paste::item! {
            impl [<$name:camel Builder>] {
                html_events!($elem_type);
            }

            impl $crate::macros::DomElement for [<$name:camel Builder>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    // TODO: Export from crate
                    use $crate::macros::JsCast;
                    self.0.dom_element().unchecked_into()
                }
            }

            impl $crate::macros::DomElement for [<$name:camel>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    // TODO: Export from crate
                    use $crate::macros::JsCast;
                    self.0.dom_element().unchecked_into()
                }
            }
        }
    };
}

macro_rules! html_element {
    (
        $(#[$elem_meta:meta])*
        $name:ident {
            $(
                $(#[$attr_meta:meta])*
                $attr:ident : $typ:ty
            ),* $(,)?
        }
    ) => {
        // TODO: Use meta stuff
        paste::item! {
            pub fn $name() -> [<$name:camel Builder>] {
                [<$name: camel Builder>]($crate::macros::tag(stringify!($name)))
            }

            pub struct [<$name:camel Builder>]($crate::macros::ElementBuilder);

            impl [<$name:camel Builder>] {
                attributes![id: String, class: String, $($attr: $typ, )*];

                pub fn child<Child>(self, c: Child) -> Self
                where
                    Child: Into<$crate::macros::Element>
                {
                    Self(self.0.child(c.into()))
                }
            }

            impl $crate::macros::Builder for [<$name:camel Builder>] {
                type Target = [<$name:camel>];

                fn build(self) -> Self::Target {
                    [<$name:camel>](self.0.build())
                }
            }

            impl From<[<$name:camel Builder>]> for $crate::macros::Element {
                fn from(builder: [<$name:camel Builder>]) -> Self {
                    use $crate::macros::Builder;
                    builder.build().into()
                }
            }

            impl From<[<$name:camel Builder>]> for $crate::macros::ElementBuilder {
                fn from(builder: [<$name:camel Builder>]) -> Self {
                    builder.0
                }
            }

            #[derive(Clone)]
            pub struct [<$name:camel>]($crate::macros::Element);

            impl $crate::macros::Builder for [<$name:camel>] {
                type Target = Self;

                fn build(self) -> Self::Target {
                    self
                }
            }

            impl From<[<$name:camel>]> for $crate::macros::Element {
                fn from(html_elem: [<$name:camel>]) -> Self {
                    html_elem.0
                }
            }
        }
    };
}

macro_rules! text_parent {
    ($name:ident) => {
        paste::item! {
            impl [<$name:camel Builder>] {
                pub fn text(self, child: impl Text) -> Self {
                    Self(self.0.text(child))
                }
            }
        }
    };
}
