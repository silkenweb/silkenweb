//! Traits and types for building elements.
#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::{
    self,
    cell::RefCell,
    fmt,
    future::Future,
    marker::PhantomData,
    pin::{pin, Pin},
    rc::Rc,
};

use discard::DiscardOnDrop;
use futures::StreamExt;
use futures_signals::{
    cancelable_future,
    signal::{Signal, SignalExt},
    signal_vec::{always, SignalVec, SignalVecExt},
    CancelableFutureHandle,
};
use silkenweb_base::{clone, document};
use silkenweb_signals_ext::value::{Executor, RefSignalOrValue, SignalOrValue, Value};
use wasm_bindgen::{JsCast, JsValue};

use self::child_vec::ChildVec;
use super::{ChildNode, Node, ResourceVec};
use crate::{
    attribute::Attribute,
    dom::{
        private::{DomElement, DomText, EventStore, InstantiableDomElement},
        DefaultDom, Dom, Hydro, InDom, InstantiableDom, Template, Wet,
    },
    empty_str,
    hydration::HydrationStats,
    intern_str,
    node::text,
    task,
};

mod child_vec;

/// A generic HTML element.
///
/// Where available, specific DOM elements from [`crate::elements::html`] should
/// be used in preference to this.
///
/// `Mutability` should be one of [`Mut`] or [`Const`].
pub struct GenericElement<D: Dom = DefaultDom, Mutability = Mut> {
    static_child_count: usize,
    child_vec: Option<Pin<Box<dyn SignalVec<Item = Node<D>>>>>,
    resources: ResourceVec,
    events: EventStore,
    element: D::Element,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
    phantom: PhantomData<Mutability>,
}

impl<D: Dom> GenericElement<D> {
    /// Construct an element with type `tag` in `namespace`.
    pub fn new(namespace: Namespace, tag: &str) -> Self {
        Self::from_dom(D::Element::new(namespace, tag), 0)
    }

    /// Make this element immutable.
    pub fn freeze(mut self) -> GenericElement<D, Const> {
        self.build();
        GenericElement {
            static_child_count: self.static_child_count,
            child_vec: self.child_vec,
            resources: self.resources,
            events: self.events,
            element: self.element,
            #[cfg(debug_assertions)]
            attributes: self.attributes,
            phantom: PhantomData,
        }
    }

    pub(crate) fn from_dom(element: D::Element, static_child_count: usize) -> Self {
        Self {
            static_child_count,
            child_vec: None,
            resources: Vec::new(),
            events: EventStore::default(),
            element,
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
            phantom: PhantomData,
        }
    }

    pub(crate) fn store_child(&mut self, mut child: Self) {
        child.build();
        self.resources.append(&mut child.resources);
        self.events.combine(child.events);
    }

    fn check_attribute_unique(&mut self, name: &str) {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));
        let _ = name;
    }

    async fn class_signal<T>(mut element: D::Element, class: impl Signal<Item = T>)
    where
        T: AsRef<str>,
    {
        let mut class = pin!(class.to_stream());

        if let Some(new_value) = class.next().await {
            element.add_class(intern_str(new_value.as_ref()));
            let mut previous_value = new_value;

            while let Some(new_value) = class.next().await {
                element.remove_class(previous_value.as_ref());
                element.add_class(intern_str(new_value.as_ref()));
                previous_value = new_value;
            }
        }
    }

    async fn classes_signal<T>(
        mut element: D::Element,
        classes: impl Signal<Item = impl IntoIterator<Item = T>>,
    ) where
        T: AsRef<str>,
    {
        let mut classes = pin!(classes.to_stream());
        let mut previous_classes: Vec<T> = Vec::new();

        while let Some(new_classes) = classes.next().await {
            for to_remove in previous_classes.drain(..) {
                element.remove_class(to_remove.as_ref());
            }

            for to_add in new_classes {
                element.add_class(intern_str(to_add.as_ref()));
                previous_classes.push(to_add);
            }
        }
    }
}

impl<D: Dom, Mutability> GenericElement<D, Mutability> {
    fn build(&mut self) {
        if let Some(children) = self.child_vec.take() {
            let child_vec = Rc::new(RefCell::new(ChildVec::new(
                self.element.clone(),
                self.static_child_count,
            )));

            let future = children.for_each({
                clone!(child_vec);
                move |update| {
                    child_vec.borrow_mut().apply_update(update);
                    async {}
                }
            });

            // `future` may finish if, for example, a `MutableVec` is dropped. So we need to
            // keep a hold of `child_vec`, as it may own signals that need updating.
            let resource = (child_vec, spawn_cancelable_future(future));
            self.resources.push(Box::new(resource));
        }

        // This improves memory usage, and doesn't detectably impact performance
        self.resources.shrink_to_fit();
    }
}

impl<Param, D> GenericElement<Template<Param, D>>
where
    Param: 'static,
    D: InstantiableDom,
{
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

        child.select_spawn(
            |parent, child| {
                parent
                    .element
                    .append_child(&D::Text::new(child.as_ref()).into());
            },
            |parent, child_signal| {
                let mut text_node = D::Text::new(empty_str());
                parent.element.append_child(&text_node.clone().into());

                child_signal.for_each(move |new_value| {
                    text_node.set_text(new_value.as_ref());
                    async {}
                })
            },
            &mut self,
        );

        self
    }

    fn optional_child(self, child: impl SignalOrValue<Item = Option<impl ChildNode<D>>>) -> Self {
        child.select(
            |mut parent, child| {
                if let Some(child) = child {
                    if parent.child_vec.is_some() {
                        return parent.children_signal(always(vec![child]));
                    }

                    parent.static_child_count += 1;
                    let child = child.into();

                    parent.element.append_child(&child.node);
                    parent.resources.extend(child.resources);
                    parent.events.combine(child.events);
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

    fn children<N>(mut self, children: impl IntoIterator<Item = N>) -> Self
    where
        N: Into<Node<D>>,
    {
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

    fn children_signal<N>(mut self, children: impl SignalVec<Item = N> + 'static) -> Self
    where
        N: Into<Node<D>>,
    {
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

impl<Mutability> GenericElement<Wet, Mutability> {
    pub(crate) fn dom_element(&self) -> web_sys::Element {
        self.element.dom_element()
    }
}

impl<Mutability> GenericElement<Hydro, Mutability> {
    pub(crate) fn hydrate(
        mut self,
        element: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) -> GenericElement<Wet, Const> {
        self.build();

        GenericElement {
            static_child_count: self.static_child_count,
            child_vec: None,
            resources: self.resources,
            events: self.events,
            element: self.element.hydrate(element, tracker),
            #[cfg(debug_assertions)]
            attributes: self.attributes,
            phantom: PhantomData,
        }
    }
}

impl<D: InstantiableDom> ShadowRootParent<D> for GenericElement<D> {
    fn attach_shadow_children<N>(mut self, children: impl IntoIterator<Item = N> + 'static) -> Self
    where
        N: Into<Node<D>>,
    {
        let children: Vec<_> = children
            .into_iter()
            .map(|child| {
                let mut child = child.into();
                let child_node = child.node;
                self.resources.append(&mut child.resources);
                self.events.combine(child.events);
                child_node
            })
            .collect();

        self.element.attach_shadow_children(children);
        self
    }
}

impl<D: Dom> Element for GenericElement<D> {
    type Dom = D;
    type DomElement = web_sys::Element;

    fn class<'a, T>(mut self, class: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str>,
    {
        class.select_spawn(
            |elem, class| elem.element.add_class(intern_str(class.as_ref())),
            |elem, class| Self::class_signal(elem.element.clone(), class),
            &mut self,
        );

        self
    }

    fn classes<'a, T, Iter>(mut self, classes: impl RefSignalOrValue<'a, Item = Iter>) -> Self
    where
        T: 'a + AsRef<str>,
        Iter: 'a + IntoIterator<Item = T>,
    {
        classes.select_spawn(
            |elem, classes| {
                for class in classes {
                    elem.element.add_class(intern_str(class.as_ref()));
                }
            },
            |elem, classes| Self::classes_signal(elem.element.clone(), classes),
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

        value.select_spawn(
            |elem, value| elem.element.attribute(name, value),
            |elem, value| {
                let name = name.to_owned();
                let mut element = elem.element.clone();

                value.for_each(move |new_value| {
                    element.attribute(&name, new_value);

                    async {}
                })
            },
            &mut self,
        );

        self
    }

    fn style_property<'a>(
        mut self,
        name: impl Into<String>,
        value: impl RefSignalOrValue<'a, Item = impl AsRef<str> + 'a>,
    ) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(!self.attributes.contains("style"));

        let name = name.into();

        value.select_spawn(
            |elem, value| elem.element.style_property(&name, value.as_ref()),
            |elem, value| {
                clone!(name);
                let mut element = elem.element.clone();

                value.for_each(move |new_value| {
                    element.style_property(&name, new_value.as_ref());

                    async {}
                })
            },
            &mut self,
        );

        self
    }

    fn effect(mut self, f: impl FnOnce(&Self::DomElement) + 'static) -> Self {
        self.element.effect(f);
        self
    }

    fn effect_signal<T>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Clone + Fn(&Self::DomElement, T) + 'static,
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

    fn handle(&self) -> ElementHandle<Self::Dom, Self::DomElement> {
        ElementHandle(self.element.clone(), PhantomData)
    }

    fn spawn_future(mut self, future: impl Future<Output = ()> + 'static) -> Self {
        self.spawn(future);
        self
    }

    fn on(mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self {
        self.element.on(name, f, &mut self.events);
        self
    }
}

impl<D: Dom> Executor for GenericElement<D> {
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.resources
            .push(Box::new(spawn_cancelable_future(future)));
    }
}

impl<D: Dom, Mutability> Value for GenericElement<D, Mutability> {}

impl<D: Dom, Mutability> InDom for GenericElement<D, Mutability> {
    type Dom = D;
}

impl<D: Dom, Mutability> From<GenericElement<D, Mutability>> for Node<D> {
    fn from(mut elem: GenericElement<D, Mutability>) -> Self {
        elem.build();

        Self {
            node: elem.element.into(),
            resources: elem.resources,
            events: elem.events,
        }
    }
}

/// Trait alias for elements that can be used as a child
pub trait ChildElement<D: Dom = DefaultDom>:
    Into<GenericElement<D, Const>> + Into<Node<D>> + Value + 'static
{
}

impl<D, T> ChildElement<D> for T
where
    D: Dom,
    T: Into<GenericElement<D, Const>> + Into<Node<D>> + Value + 'static,
{
}

/// An HTML element.
pub trait Element: Sized {
    type Dom: Dom;
    type DomElement: JsCast + 'static;

    /// Add a class to the element.
    ///
    /// `class` must not contain whitespace. This method can be called multiple
    /// times to add multiple classes.
    ///
    /// Classes must be unique across all
    /// invocations of this method and [`Self::classes`], otherwise the results
    /// are undefined. Any class signal values, past or present, must be unique
    /// w.r.t. other invocations.
    ///
    /// # Panics
    ///
    /// This panics if `class` contains whitespace.
    ///
    /// # Examples
    ///
    /// Add static class names:
    ///
    /// ```
    /// # use html::{div, Div};
    /// # use silkenweb::{dom::Dry, prelude::*};
    /// let app: Div<Dry> = div().class("my-class").class("my-other-class");
    /// assert_eq!(
    ///     app.freeze().to_string(),
    ///     r#"<div class="my-class my-other-class"></div>"#
    /// );
    /// ```
    ///
    /// Add dynamic class names:
    ///
    /// ```
    /// # use html::{div, Div};
    /// # use silkenweb::{dom::Dry, prelude::*, task::{render_now, server}};
    /// # server::block_on(server::scope(async {
    /// let my_class = Mutable::new("my-class");
    /// let my_other_class = Mutable::new("my-other-class");
    /// let app: Div<Dry> = div()
    ///     .class(Sig(my_class.signal()))
    ///     .class(Sig(my_other_class.signal()));
    /// let app = app.freeze();
    ///
    /// render_now().await;
    /// assert_eq!(
    ///     app.to_string(),
    ///     r#"<div class="my-class my-other-class"></div>"#
    /// );
    ///
    /// my_other_class.set("my-other-class-updated");
    ///
    /// render_now().await;
    /// assert_eq!(
    ///     app.to_string(),
    ///     r#"<div class="my-class my-other-class-updated"></div>"#
    /// );
    /// # }))
    /// ```
    fn class<'a, T>(self, class: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str>;

    /// Set the classes on an element
    ///
    /// All `classes` must not contain spaces. This method can be called
    /// multiple times and will add to existing classes.
    ///
    /// Classes must be unique across all invocations of this method and
    /// [`Self::class`], otherwise the results are undefined. Any class signal
    /// values, past or present, must be unique w.r.t. other invocations.
    ///
    /// # Panics
    ///
    /// Panics if any of the items in `classes` contain whitespace.
    /// # Examples
    ///
    /// Add static class names:
    ///
    /// ```
    /// # use html::{div, Div};
    /// # use silkenweb::{dom::Dry, prelude::*};
    /// let app: Div<Dry> = div().classes(["class0", "class1"]);
    /// assert_eq!(
    ///     app.freeze().to_string(),
    ///     r#"<div class="class0 class1"></div>"#
    /// );
    /// ```
    ///
    /// Add dynamic class names:
    ///
    /// ```
    /// # use html::{div, Div};
    /// # use silkenweb::{dom::Dry, prelude::*, task::{render_now, server}};
    /// # server::block_on(server::scope(async {
    /// let my_classes = Mutable::new(vec!["class0", "class1"]);
    /// let app: Div<Dry> = div().classes(Sig(my_classes.signal_cloned()));
    /// let app = app.freeze();
    ///
    /// render_now().await;
    /// assert_eq!(app.to_string(), r#"<div class="class0 class1"></div>"#);
    ///
    /// my_classes.set(vec![]);
    ///
    /// render_now().await;
    /// assert_eq!(app.to_string(), r#"<div class=""></div>"#);
    /// # }))
    /// ```
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

    /// Set an inline style property
    ///
    /// The property can be a value or a signal. Signals should be wrapped in
    /// the [`Sig`] newtype.
    ///
    /// [`Sig`]: crate::value::Sig
    fn style_property<'a>(
        self,
        name: impl Into<String>,
        value: impl RefSignalOrValue<'a, Item = impl AsRef<str> + 'a>,
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
    /// # use html::{input, Input};
    /// # use silkenweb::prelude::*;
    /// # let input: Input =
    /// input().effect(|elem: &HtmlInputElement| elem.focus().unwrap());
    /// ```
    fn effect(self, f: impl FnOnce(&Self::DomElement) + 'static) -> Self;

    /// Apply an effect after the next render each time a signal yields a new
    /// value.
    fn effect_signal<T: 'static>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Fn(&Self::DomElement, T) + Clone + 'static,
    ) -> Self;

    /// Get a handle to the element.
    ///
    /// Handles can be cloned and used within click handlers, for example.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use html::{button, div, input, Div};
    /// # use silkenweb::prelude::*;
    /// let text = Mutable::new("".to_string());
    /// let input = input();
    /// let input_handle = input.handle();
    /// let app: Div = div()
    ///     .child(input)
    ///     .child(button().text("Read Input").on_click({
    ///         clone!(text);
    ///         move |_, _| text.set(input_handle.dom_element().value())
    ///     }))
    ///     .text(Sig(text.signal_cloned()));
    /// ```
    fn handle(&self) -> ElementHandle<Self::Dom, Self::DomElement>;

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

/// An element that can have children.
pub trait ParentElement<D: Dom = DefaultDom>: Element {
    /// Add a text child to this element
    ///
    /// # Example
    ///
    /// Static text:
    ///
    /// ```no_run
    /// # use html::{div, Div};
    /// # use silkenweb::prelude::*;
    /// # let d: Div =
    /// div().text("Hello, world!");
    /// ```
    ///
    /// Dynamic text:
    ///
    /// ```no_run
    /// # use html::{div, Div};
    /// # use silkenweb::prelude::*;
    /// let text = Mutable::new("Hello, world!");
    /// # let d: Div =
    /// div().text(Sig(text.signal()));
    /// ```
    fn text<'a, T>(self, child: impl RefSignalOrValue<'a, Item = T>) -> Self
    where
        T: 'a + AsRef<str> + Into<String>;

    /// Add a child to the element.
    ///
    /// # Example
    ///
    /// Add static children:
    ///
    /// ```no_run
    /// # use html::{div, p, Div};
    /// # use silkenweb::prelude::*;
    /// # let div: Div =
    /// div().child(p().text("Hello,")).child(p().text("world!"));
    /// ```
    ///
    /// Add a dynamic child:
    ///
    /// ```no_run
    /// # use html::{div, Div};
    /// # use silkenweb::prelude::*;
    /// let text = Mutable::new("Hello, world!");
    ///
    /// # let d: Div =
    /// div().child(Sig(text.signal().map(|text| div().text(text))));
    /// ```
    fn child(self, child: impl SignalOrValue<Item = impl ChildNode<D>>) -> Self {
        self.optional_child(child.map(|child| Some(child)))
    }

    /// Add an optional child to the element.
    ///
    /// The child will update when the signal changes to `Some(..)`, and will be
    /// removed when the signal changes to `None`.
    ///
    /// # Example
    ///
    /// Add a static optional child:
    ///
    /// ```no_run
    /// # use html::{div, p, Div};
    /// # use silkenweb::prelude::*;
    /// let text = Mutable::new("hello");
    ///
    /// # let div: Div =
    /// div().optional_child(Some(p().text("Hello, world!")));
    /// ```
    ///
    /// Add a dynamic optional child:
    ///
    /// ```no_run
    /// # use html::{div, Div};
    /// # use silkenweb::prelude::*;
    /// let text = Mutable::new("hello");
    ///
    /// # let div: Div =
    /// div().optional_child(Sig(text.signal().map(|text| {
    ///     if text.is_empty() {
    ///         None
    ///     } else {
    ///         Some(div().text(text))
    ///     }
    /// })));
    /// ```
    fn optional_child(self, child: impl SignalOrValue<Item = Option<impl ChildNode<D>>>) -> Self;

    /// Add children to the element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use html::{div, p, Div};
    /// # use silkenweb::prelude::*;
    /// # let div: Div =
    /// div().children([p().text("Hello,"), p().text("world!")]);
    /// ```
    fn children<N>(self, children: impl IntoIterator<Item = N>) -> Self
    where
        N: Into<Node<D>>;

    /// Add children from a [`SignalVec`] to the element.
    ///
    /// See [counter_list](https://github.com/silkenweb/silkenweb/tree/main/examples/counter-list/src/main.rs)
    /// for an example
    fn children_signal<N>(self, children: impl SignalVec<Item = N> + 'static) -> Self
    where
        N: Into<Node<D>>;
}

/// An element that can be a shadow host.
pub trait ShadowRootParent<D: InstantiableDom = DefaultDom>: Element {
    /// Attach an open shadow root to `self` and add `children` to it.
    ///
    /// If there's already a shadow root, the `children` are appended to it.
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)
    fn attach_shadow_children<N>(self, children: impl IntoIterator<Item = N> + 'static) -> Self
    where
        N: Into<Node<D>>;
}

impl<D> fmt::Display for GenericElement<D, Const>
where
    D: Dom,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

impl<Param, D> GenericElement<Template<Param, D>, Const>
where
    D: InstantiableDom,
    Param: 'static,
{
    /// Instantiate a template with `param`.
    ///
    /// See [`Template`] for an example.
    pub fn instantiate(&self, param: &Param) -> GenericElement<D> {
        self.element.instantiate(param)
    }
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
/// The handle will only be valid for [`Wet`]  DOM elements, so the methods
/// should only be used inside event handlers and effects.
///
/// See [`Element::handle`] for an example.
#[derive(Clone)]
pub struct ElementHandle<D: Dom, DomElement>(D::Element, PhantomData<DomElement>);

impl<D: Dom, DomElement: JsCast + Clone> ElementHandle<D, DomElement> {
    /// Get the associated DOM element, if it is a [`Wet`] element.
    ///
    /// If the referenced element is not [`Wet`] or a hydrated [`Hydro`]
    /// element, this will return [`None`].
    pub fn try_dom_element(&self) -> Option<DomElement> {
        self.0
            .try_dom_element()
            .map(|elem| elem.dyn_into().unwrap())
    }

    /// Get the associated DOM element, or panic.
    ///
    /// # Panics
    ///
    /// This will panic if [`Self::try_dom_element`] would return [`None`], or
    /// `self` was created from an invalid [`ElementHandle::cast`].
    pub fn dom_element(&self) -> DomElement {
        self.try_dom_element()
            .expect("Dom type doesn't support element handles")
    }
}

impl<D: Dom> ElementHandle<D, web_sys::Element> {
    /// Cast the dom type of an [`ElementHandle`].
    ///
    /// It is the clients responsibility to ensure the new type is correct.
    pub fn cast<T: JsCast>(self) -> ElementHandle<D, T> {
        ElementHandle(self.0, PhantomData)
    }
}

/// The namespace of a DOM element.
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
    pub(crate) fn create_element(self, tag: &str) -> web_sys::Element {
        match self {
            Namespace::Html => document::create_element(tag),
            _ => document::create_element_ns(intern_str(self.as_str()), tag),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::Svg => "http://www.w3.org/2000/svg",
            Namespace::MathML => "http://www.w3.org/1998/Math/MathML",
            Namespace::Other(ns) => ns,
        }
    }
}

/// Marker type for mutable elements.
pub struct Mut;

/// Marker type for immutable elements.
pub struct Const;
