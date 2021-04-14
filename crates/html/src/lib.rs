#![allow(clippy::must_use_candidate)]
use surfinia_core::{tag, Builder, Element, ElementBuilder};

macro_rules! html_element {
    ($name:ident { $($attr:ident : $typ:ty),* $(,)? }) => {
        paste::item! {
            pub fn $name() -> [<$name:camel Builder>] {
                [<$name: camel Builder>](HtmlElementBuilder::new(stringify!($name)))
            }

            pub struct [<$name:camel Builder>](HtmlElementBuilder);

            impl [<$name:camel Builder>] {
                pub fn id(self, value: impl AsRef<str>) -> Self {
                    Self(self.0.id(value))
                }

                pub fn child<Child: Parent<[<$name:camel>]>>(self, c: Child) -> Self {
                    Self(self.0.child(c.into()))
                }

                pub fn on_click(self, f: impl 'static + FnMut()) -> Self {
                    Self(self.0.on_click(f))
                }
            }

            impl Builder for [<$name:camel Builder>] {
                type Target = [<$name:camel>];

                fn build(self) -> Self::Target {
                    [<$name:camel>](self.0.build())
                }
            }

            impl From<[<$name:camel Builder>]> for Element {
                fn from(div: [<$name:camel Builder>]) -> Self {
                    div.build().into()
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
                fn from(div: [<$name:camel>]) -> Self {
                    div.0
                }
            }

            impl From<Element> for [<$name:camel>] {
                fn from(elem: Element) -> Self {
                    [<$name:camel>](elem)
                }
            }
        }
    };
}

macro_rules! text_parent {
    ($name:ident) => {
        paste::item! {
            impl [<$name:camel Builder>] {
                pub fn text(self, child: impl AsRef<str>) -> Self {
                    Self(self.0.text(child))
                }
            }
        }
    };
}

macro_rules! categories {
    ($name:ident [$($category:ident),* $(,)?] ) => {
        paste::item! {
            // We get better error messages if we implement these traits directly for
            // builder as well as target, rather than via a blanket trait.
            $(
                impl content_category::$category for [<$name:camel>] {}
                impl content_category::$category for [<$name:camel Builder>] {}
            )*
        }
    }
}

macro_rules! child_categories {
    ($name:ident [$($category:ident),* $(,)?] ) => {
        paste::item! {
            // We get better error messages if we implement these traits directly for
            // builder as well as target, rather than via a blanket trait.
            $(
                impl<Child: content_category::$category> ParentCategory<[<$name:camel>]> for Child {}
            )*
        }
    }
}

html_element!(div {});
text_parent!(div);
categories!(div[Flow, Palpable]);
child_categories!(div[Flow]);

html_element!(button {});
text_parent!(button);
categories!(button[Flow, Palpable]);
child_categories!(button[Flow]);

struct HtmlElementBuilder(ElementBuilder);

impl HtmlElementBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(tag(name))
    }

    pub fn id(self, value: impl AsRef<str>) -> Self {
        Self(self.0.attribute("id", value))
    }

    pub fn on_click(self, mut f: impl 'static + FnMut()) -> Self {
        Self(self.0.on("click", move |_| f()))
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        Self(self.0.text(child))
    }

    pub fn child(self, c: impl Into<Element>) -> Self {
        Self(self.0.child(c.into()))
    }
}

impl Builder for HtmlElementBuilder {
    type Target = Element;

    fn build(self) -> Self::Target {
        self.0.build()
    }
}

pub trait Parent<T>: Into<Element> {}

impl<Child, T: ParentCategory<Child> + Into<Element>> Parent<Child> for T {}

pub trait ParentCategory<T> {}

pub mod content_category {
    macro_rules! content_categories {
        ($($name:ident),* $(,)?) => { $(pub trait $name {})* }
    }

    content_categories![
        Flow,
        Phrasing,
        Interactive,
        Listed,
        Labelable,
        Submittable,
        Palpable,
    ];
}
