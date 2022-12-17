//! Element DOM types, and traits for building elements.
//!
//! The DOM element types are generic. Specific DOM elements from
//! [`crate::elements::html`] should be used in preference to these, where they
//! are available.
//!
//! The [`Element`] and [`ParentElement`] traits are implemented by
//! specific DOM elements as well as by [`GenericElement`]. See the [`div`]
//! element for example.
//!
//! [`div`]: crate::elements::html::div

#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::{self, cell::Cell, future::Future, pin::Pin, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    cancelable_future,
    signal::{Signal, SignalExt},
    signal_vec::{always, SignalVec, SignalVecExt},
    CancelableFutureHandle,
};
use silkenweb_base::{clone, empty_str, intern_str};
use silkenweb_signals_ext::value::{Executor, RefSignalOrValue, SignalOrValue, Value};
use wasm_bindgen::{JsCast, JsValue};

use self::{child_vec::ChildVec, template::Template};
use super::{Node, Resource};
use crate::{
    attribute::Attribute,
    dom::{
        private::{DomElement, DomText},
        DefaultDom, Dom, InstantiableDom,
    },
    node::text,
    task,
};

mod child_vec;

pub mod template;

/// Build an HTML element.
pub struct GenericElement<D: Dom = DefaultDom> {
    static_child_count: usize,
    child_vec: Option<Pin<Box<dyn SignalVec<Item = Node<D>>>>>,
    resources: Vec<Resource>,
    element: D::Element,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
}

impl<D: Dom> GenericElement<D> {
    pub fn new(tag: &str) -> Self {
        Self::new_in_namespace(Namespace::Html, tag)
    }

    pub fn new_in_namespace(namespace: Namespace, tag: &str) -> Self {
        Self {
            static_child_count: 0,
            child_vec: None,
            resources: Vec::new(),
            element: D::Element::new(namespace, tag),
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        }
    }

    fn check_attribute_unique(&mut self, name: &str) {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));
        let _ = name;
    }

    fn build(mut self) -> Self {
        if let Some(children) = self.child_vec.take() {
            let mut child_vec = ChildVec::new(self.element.clone(), self.static_child_count);

            self.spawn(children.for_each(move |update| {
                child_vec.apply_update(update);
                async {}
            }))
        }

        self.resources.shrink_to_fit();
        self
    }

    fn class_signal<T>(
        element: &mut D::Element,
        class: T,
        previous_value: &Rc<Cell<Option<T>>>,
    ) -> impl Future<Output = ()>
    where
        T: AsRef<str>,
    {
        if let Some(previous) = previous_value.replace(None) {
            element.remove_class(previous.as_ref());
        }

        element.add_class(intern_str(class.as_ref()));
        previous_value.set(Some(class));

        async {}
    }

    fn classes_signal<T>(
        element: &mut D::Element,
        classes: impl IntoIterator<Item = T>,
        previous_values: &Rc<Cell<Vec<T>>>,
    ) -> impl Future<Output = ()>
    where
        T: AsRef<str>,
    {
        let mut previous = previous_values.replace(Vec::new());

        for to_remove in &previous {
            element.remove_class(to_remove.as_ref());
        }

        previous.clear();

        for to_add in classes {
            element.add_class(intern_str(to_add.as_ref()));
            previous.push(to_add);
        }

        previous_values.set(previous);

        async {}
    }
}

impl<D, Param> GenericElement<Template<D, Param>>
where
    D: InstantiableDom,
    Param: 'static,
{
    pub fn instantiate(&self, param: &Param) -> GenericElement<D> {
        self.element.instantiate(param)
    }

    pub fn on_instantiate(
        mut self,
        f: impl 'static + Fn(GenericElement<D>, &Param) -> GenericElement<D>,
    ) -> Self {
        self.element.on_instantiate(f);
        self
    }
}

impl<D: Dom> ParentElement<D> for GenericElement<D> {
    fn text<'a, T>(mut self, child: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str> + Into<String>,
    {
        if self.child_vec.is_some() {
            return self.child(child.map(|child| text(child.as_ref())));
        }

        self.static_child_count += 1;

        child.for_each(
            |parent, child| {
                parent
                    .element
                    .append_child(&D::Text::new(child.as_ref()).into());
            },
            |parent| {
                let mut text_node = D::Text::new(empty_str());
                parent.element.append_child(&text_node.clone().into());

                move |new_value| {
                    text_node.set_text(new_value.as_ref());
                    async {}
                }
            },
            &mut self,
        );

        self
    }

    fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Value + Into<Node<D>> + 'static>>,
    ) -> Self {
        child.select(
            |mut parent, child| {
                if let Some(child) = child {
                    if parent.child_vec.is_some() {
                        return parent.children_signal(always(vec![child]));
                    }

                    parent.static_child_count += 1;
                    let child = child.into();

                    parent.element.append_child(child.as_node());
                    parent.resources.extend(child.resources);
                }

                parent
            },
            |parent, child| {
                let child_vec = child
                    .map(|child| child.into_iter().collect::<Vec<_>>())
                    .to_signal_vec();
                parent.children_signal(child_vec)
            },
            self,
        )
    }

    fn children(mut self, children: impl IntoIterator<Item = impl Into<Node<D>>>) -> Self {
        if self.child_vec.is_some() {
            let children = children
                .into_iter()
                .map(|node| node.into())
                .collect::<Vec<_>>();
            return self.children_signal(always(children));
        }

        for child in children {
            self = self.child(child.into());
        }

        self
    }

    fn children_signal(
        mut self,
        children: impl SignalVec<Item = impl Into<Node<D>>> + 'static,
    ) -> Self {
        let new_children = children.map(|child| child.into());

        let boxed_children = if let Some(child_vec) = self.child_vec.take() {
            child_vec.chain(new_children).boxed_local()
        } else {
            new_children.boxed_local()
        };

        self.child_vec = Some(boxed_children);

        self
    }
}

impl<D: Dom> ShadowRootParent<D> for GenericElement<D> {
    /// Currently, there's no way to send the shadow root as plain HTML, until
    /// we get [Declarative Shadow Root](`<template shadowroot="open">...`).
    ///
    /// [Declarative Shadow Root]: https://caniuse.com/?search=template%20shadowroot
    fn attach_shadow_children(
        self,
        children: impl IntoIterator<Item = impl Into<Node<D>>> + 'static,
    ) -> Self {
        self.element
            .attach_shadow_children(children.into_iter().map(|child| child.into().into_node()));
        self
    }
}

impl<D: Dom> Element for GenericElement<D> {
    type DomType = web_sys::Element;

    fn class<'a, T>(mut self, class: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str>,
    {
        class.for_each(
            |elem, class| elem.element.add_class(intern_str(class.as_ref())),
            |elem| {
                let mut element = elem.element.clone();
                let previous_value = Rc::new(Cell::new(None));

                move |class: T| Self::class_signal(&mut element, class, &previous_value)
            },
            &mut self,
        );

        self
    }

    fn classes<'a, T, Iter>(mut self, classes: impl RefSignalOrValue<'a, Item = Iter>) -> Self
    where
        T: 'a + AsRef<str>,
        Iter: 'a + IntoIterator<Item = T>,
    {
        classes.for_each(
            |elem, classes| {
                for class in classes {
                    elem.element.add_class(intern_str(class.as_ref()));
                }
            },
            |elem| {
                let mut element = elem.element.clone();
                let previous_values = Rc::new(Cell::new(Vec::<T>::new()));

                move |classes| Self::classes_signal(&mut element, classes, &previous_values)
            },
            &mut self,
        );

        self
    }

    fn attribute<'a>(
        mut self,
        name: &str,
        value: impl RefSignalOrValue<'a, Item = impl Attribute>,
    ) -> Self {
        self.check_attribute_unique(name);

        value.for_each(
            |elem, value| elem.element.attribute(name, value),
            |elem| {
                let name = name.to_owned();
                let mut element = elem.element.clone();

                move |new_value| {
                    element.attribute(&name, new_value);

                    async {}
                }
            },
            &mut self,
        );

        self
    }

    fn effect(mut self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
        self.element.effect(f);
        self
    }

    fn effect_signal<T>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Clone + Fn(&Self::DomType, T) + 'static,
    ) -> Self
    where
        T: 'static,
    {
        let mut element = self.element.clone();

        let future = sig.for_each(move |x| {
            clone!(f);
            element.effect(move |elem| f(elem, x));
            async {}
        });

        self.spawn_future(future)
    }

    fn handle(&self) -> ElementHandle<Self::DomType> {
        ElementHandle(self.element.try_dom_element()).cast()
    }

    fn spawn_future(mut self, future: impl Future<Output = ()> + 'static) -> Self {
        self.spawn(future);
        self
    }

    fn on(mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self {
        self.element.on(name, f);
        self
    }
}

impl<D: Dom> Executor for GenericElement<D> {
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.resources.push(spawn_cancelable_future(future));
    }
}

impl<D: Dom> Value for GenericElement<D> {}

impl<D: Dom> From<GenericElement<D>> for Node<D> {
    fn from(elem: GenericElement<D>) -> Self {
        let elem = elem.build();

        Self {
            node: elem.element.into(),
            resources: elem.resources,
        }
    }
}

/// An HTML element builder.
pub trait Element: Sized {
    type DomType: JsCast + 'static;

    // TODO: Doc
    fn class<'a, T>(self, class: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str>;

    // TODO: Doc
    // Adds or removes class names on the element
    //
    // Each time the signal updates, the previous value's classes will be
    // removed and the current classes will be added to the element.
    //
    // # Panics
    //
    // If a value contains whitespace, this will panic.

    /// Set the classes on an element
    ///
    /// All items in `value` must not contain spaces. This method can be called
    /// multiple times and will add to existing classes.
    ///
    /// # Panics
    ///
    /// Panics if any of the items in `value` contain whitespace.
    fn classes<'a, T, Iter>(self, classes: impl RefSignalOrValue<'a, Item = Iter>) -> Self
    where
        T: 'a + AsRef<str>,
        Iter: 'a + IntoIterator<Item = T>;

    /// Set an attribute
    ///
    /// The attribute can either be a value or a signal. Signals should be
    /// wrapped in the [`Sig`] newtype.`Option<impl Attribute>` can be used to
    /// add/remove an attribute based on a signal.
    ///
    /// [`Sig`]: crate::value::Sig
    fn attribute<'a>(
        self,
        name: &str,
        value: impl RefSignalOrValue<'a, Item = impl Attribute>,
    ) -> Self;

    /// Apply an effect after the next render.
    ///
    /// Effects give you access to the underlying DOM element.
    ///
    /// # Example
    ///
    /// Set the focus to an `HTMLInputElement`.
    ///
    /// ```no_run
    /// # use web_sys::HtmlInputElement;
    /// # use silkenweb::{elements::html::input, node::element::Element};
    /// input().effect(|elem: &HtmlInputElement| elem.focus().unwrap());
    /// ```
    fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self;

    /// Apply an effect after the next render each time a singal yields a new
    /// value.
    fn effect_signal<T: 'static>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Fn(&Self::DomType, T) + Clone + 'static,
    ) -> Self;

    /// Get a handle to the element.
    ///
    /// Handles can be cloned and used within click handlers, for example.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::Mutable;
    /// # use silkenweb::{
    /// #     elements::html::*,
    /// #     node::element::Element,
    /// #     prelude::*,
    /// #     value::Sig
    /// # };
    /// let text = Mutable::new("".to_string());
    /// let input = input();
    /// let input_handle = input.handle();
    /// let app = div()
    ///     .child(input)
    ///     .child(button().text("Read Input").on_click({
    ///         clone!(text);
    ///         move |_, _| text.set(input_handle.dom_element().value())
    ///     }))
    ///     .text(Sig(text.signal_cloned()));
    /// ```
    fn handle(&self) -> ElementHandle<Self::DomType>;

    /// Spawn a future on the element.
    ///
    /// The future will be dropped when this element is dropped.
    fn spawn_future(self, future: impl Future<Output = ()> + 'static) -> Self;

    /// Register an event handler.
    ///
    /// `name` is the name of the event. See the [MDN Events] page for a list.
    ///
    /// `f` is the callback when the event fires and will be passed the
    /// javascript `Event` object.
    ///
    /// [MDN Events]: https://developer.mozilla.org/en-US/docs/Web/Events
    fn on(self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self;
}

/// An element that is allowed to have children.
pub trait ParentElement<D: Dom = DefaultDom>: Element {
    // TODO: Docs for signal variant
    /// Add a text child to this element
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use silkenweb::{elements::html::div, node::element::ParentElement};
    /// div().text("Hello, world!");
    /// ```
    fn text<'a, T>(self, child: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str> + Into<String>;

    /// Add a child to the element.
    ///
    /// The child will update when the signal changes.
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::div,
    /// #     node::element::{
    /// #         ParentElement,
    /// #         Element
    /// #     },
    /// #     value::Sig
    /// # };
    /// let text = Mutable::new("hello");
    ///
    /// div().child(Sig(text.signal().map(|text| div().text(text))));
    /// ```
    // TODO: Doc for signal variant ^^^^
    /// Add a child node to the element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::{div, p},
    /// #     node::element::ParentElement,
    /// # };
    /// div().child(p().text("Hello,")).child(p().text("world!"));
    /// ```
    fn child(self, child: impl SignalOrValue<Item = impl Value + Into<Node<D>> + 'static>) -> Self {
        self.optional_child(child.map(|child| Some(child)))
    }

    // TODO: Doc for non signal
    /// Add an optional child to the element.
    ///
    /// The child will update when the signal changes to `Some(..)`, and will be
    /// removed when the signal changes to `None`.
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{elements::html::div, node::element::{ParentElement, Element}, value::Sig};
    /// let text = Mutable::new("hello");
    ///
    /// div().optional_child(Sig(text.signal().map(|text| {
    ///     if text.is_empty() {
    ///         None
    ///     } else {
    ///         Some(div().text(text))
    ///     }
    /// })));
    /// ```
    fn optional_child(
        self,
        child: impl SignalOrValue<Item = Option<impl Value + Into<Node<D>> + 'static>>,
    ) -> Self;

    /// Add children to the element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::{div, p},
    /// #     node::element::ParentElement,
    /// # };
    /// div().children([p().text("Hello,"), p().text("world!")]);
    /// ```
    fn children(self, children: impl IntoIterator<Item = impl Into<Node<D>>>) -> Self;

    /// Add children from a [`SignalVec`] to the element.
    ///
    /// See [counter_list](https://github.com/silkenweb/silkenweb/tree/main/examples/counter-list/src/main.rs)
    /// for an example
    fn children_signal(self, children: impl SignalVec<Item = impl Into<Node<D>>> + 'static)
        -> Self;
}

/// An element that is allowed to have a shadow root
pub trait ShadowRootParent<D: Dom = DefaultDom>: Element {
    /// Attach an open shadow root to `self` and add `children` to it.
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)
    fn attach_shadow_children(
        self,
        children: impl IntoIterator<Item = impl Into<Node<D>>> + 'static,
    ) -> Self;
}

fn spawn_cancelable_future(
    future: impl Future<Output = ()> + 'static,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    task::spawn_local(cancelable_future);

    handle
}

/// A handle to an element in the DOM.
///
/// This acts as a weak reference to the element. See [`Element::handle`]
/// for an example.
#[derive(Clone)]
pub struct ElementHandle<DomType>(Option<DomType>);

// TODO: Docs
impl<DomType: JsCast + Clone> ElementHandle<DomType> {
    /// Get the associated DOM element, if the referenced element is still live.
    ///
    /// If the referenced element is neither in the DOM, or referenced from the
    /// stack, this will return [`None`].
    pub fn try_dom_element(&self) -> Option<DomType> {
        self.0.clone()
    }

    /// Get the associated DOM element, or panic.
    ///
    /// # Panics
    ///
    /// This will panic if [`Self::try_dom_element`] would return [`None`].
    pub fn dom_element(&self) -> DomType {
        self.try_dom_element()
            .expect("Dom type doesn't support element handles")
    }
}

impl ElementHandle<web_sys::Element> {
    /// Cast the dom type of an [`ElementHandle`].
    ///
    /// It is the clients responsibility to ensure the new type is correct.
    pub fn cast<T: JsCast>(self) -> ElementHandle<T> {
        ElementHandle(self.0.map(|elem| elem.unchecked_into()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Namespace {
    /// New elements in the `Html` namespace are created with `create_element`,
    /// thus avoiding converting the namespace to a javascript string.
    Html,
    Svg,
    MathML,
    Other(&'static str),
}

impl Namespace {
    pub fn as_str(&self) -> &str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::Svg => "http://www.w3.org/2000/svg",
            Namespace::MathML => "http://www.w3.org/1998/Math/MathML",
            Namespace::Other(ns) => ns,
        }
    }
}
