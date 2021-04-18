#![allow(clippy::must_use_candidate)]
use surfinia_core::{
    hooks::{list_state::ElementList, state::ReadSignal},
    tag,
    AttributeValue,
    Builder,
    DomElement,
    Element,
    ElementBuilder,
    Text,
};
use wasm_bindgen::JsCast;
use web_sys as dom;

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
            pub fn $attr(self, value: impl AttributeValue<$typ>) -> Self {
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
            blur: dom::FocusEvent,
            click: dom::MouseEvent,
            dblclick: dom::MouseEvent,
            focusout: dom::FocusEvent,
            input: dom::InputEvent,
            keydown: dom::KeyboardEvent,
            keyup: dom::KeyboardEvent,
        });
    };
}

macro_rules! html_element {
    ($name:ident <$elem_type:ty> { $($attr:ident : $typ:ty),* $(,)? }) => {
        paste::item! {
            pub fn $name() -> [<$name:camel Builder>] {
                [<$name: camel Builder>](tag(stringify!($name)))
            }

            pub struct [<$name:camel Builder>](ElementBuilder);

            impl [<$name:camel Builder>] {
                attributes![id: String, class: String, $($attr: $typ, )*];
                html_events!($elem_type);

                pub fn child<Child: Parent<[<$name:camel>]>>(self, c: Child) -> Self {
                    Self(self.0.child(c.into()))
                }
            }

            impl Builder for [<$name:camel Builder>] {
                type Target = [<$name:camel>];

                fn build(self) -> Self::Target {
                    [<$name:camel>](self.0.build())
                }
            }

            impl DomElement for [<$name:camel Builder>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    self.0.dom_element().unchecked_into()
                }
            }

            impl From<[<$name:camel Builder>]> for Element {
                fn from(builder: [<$name:camel Builder>]) -> Self {
                    builder.build().into()
                }
            }

            impl From<[<$name:camel Builder>]> for ElementBuilder {
                fn from(builder: [<$name:camel Builder>]) -> Self {
                    builder.0
                }
            }

            #[derive(Clone)]
            pub struct [<$name:camel>](Element);

            impl Builder for [<$name:camel>] {
                type Target = Self;

                fn build(self) -> Self::Target {
                    self
                }
            }

            impl From<[<$name:camel>]> for Element {
                fn from(html_elem: [<$name:camel>]) -> Self {
                    html_elem.0
                }
            }

            impl DomElement for [<$name:camel>] {
                type Target = $elem_type;

                fn dom_element(&self) -> Self::Target {
                    self.0.dom_element().unchecked_into()
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

macro_rules! categories {
    ($name:ident [$($category:path),* $(,)?] ) => {
        paste::item! {
            // We get better error messages if we implement these traits directly for
            // builder as well as target, rather than via a blanket trait.
            $(
                impl content_category::$category for [<$name:camel>] {}
                impl content_category::$category for [<$name:camel Builder>] {}

                // TODO: Fix this
                impl content_category::$category for ReadSignal<[<$name:camel>]> {}
                impl content_category::$category for ReadSignal<[<$name:camel Builder>]> {}
            )*
        }
    }
}

macro_rules! child_categories {
    ($name:ident [$($category:ident),* $(,)?] ) => {
        paste::item! {
            $(
                impl<Child: content_category::$category> ParentCategory<[<$name:camel>]> for Child {}
            )*
        }
    }
}

pub fn element_list<'a, T, GenerateChild, ChildElem, ParentElem>(
    root: ParentElem,
    generate_child: GenerateChild,
    initial: impl Iterator<Item = &'a T>,
) -> ElementList<T>
where
    T: 'static,
    ChildElem: Into<Element> + Parent<ParentElem::Target>,
    ParentElem: Into<ElementBuilder> + Builder,
    GenerateChild: 'static + Fn(&T) -> ChildElem,
{
    ElementList::new(root.into(), move |c| generate_child(c).into(), initial)
}

// TODO: Set correct dom elements
html_element!(div <dom::Element> {});
text_parent!(div);
categories!(div[Flow, Palpable]);
child_categories!(div[Flow]);

html_element!(button <dom::Element> {});
text_parent!(button);
categories!(button[Flow, Palpable]);
child_categories!(button[Flow]);

html_element!(section <dom::Element> {});
categories!(section[Flow, Sectioning, Palpable]);
child_categories!(section[Flow]);

html_element!(header <dom::Element> {});
categories!(header[Flow, Palpable]);
child_categories!(header[Flow]);

html_element!(h1 <dom::Element> {});
text_parent!(h1);
categories!(h1[Flow, Heading, Palpable]);
child_categories!(h1[Phrasing]);

html_element!(input <dom::HtmlInputElement> {
    type_: String,
    placeholder: String,
    value: String,
    autofocus: bool,
    checked: bool,
});
categories!(input[Flow, Listed, Submittable, form_associated::Resettable, Phrasing]);

html_element!(label <dom::Element> { for_: String });
text_parent!(label);
categories!(label[Flow, Phrasing, Interactive, FormAssociated]);
child_categories!(label[Phrasing]);

html_element!(ul <dom::Element> {});
text_parent!(ul);
categories!(ul[Flow, Palpable]);
child_categories!(ul[Flow]); // TODO: Allowed child tags.

html_element!(li <dom::Element> {});
text_parent!(li);
categories!(li[Flow]);
child_categories!(li[Flow]);

pub trait Parent<T>: Into<Element> {}

impl<Child, T> Parent<Child> for T where T: ParentCategory<Child> + Into<Element> {}

pub trait ParentCategory<T> {}

// TODO: Fix this
impl<T> content_category::Flow for ReadSignal<ElementList<T>> {}

pub mod content_category {
    macro_rules! content_categories {
        ($($name:ident),* $(,)?) => { $(pub trait $name {})* }
    }

    content_categories![
        Flow,
        Heading,
        Phrasing,
        Interactive,
        Listed,
        Labelable,
        Submittable,
        Palpable,
        Sectioning,
        FormAssociated,
    ];

    pub mod form_associated {
        content_categories![Resettable];
    }
}
