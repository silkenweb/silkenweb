//! Element DOM types, and traits for building elements.
//!
//! The DOM element types are generic. Specific DOM elements from
//! [`crate::elements::html`] should be used in preference to these, where they
//! are available.
//!
//! The [`ElementBuilder`] and [`ParentBuilder`] traits are implemented by
//! specific DOM elements as well as by [`ElementBuilderBase`]. See the [`div`]
//! element for example.
//!
//! [`div`]: crate::elements::html::div

#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::{
    self,
    borrow::BorrowMut,
    fmt::{self, Display},
    future::Future,
    mem,
};

use discard::DiscardOnDrop;
use futures_signals::{
    cancelable_future,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
    CancelableFutureHandle,
};
use silkenweb_base::{clone, intern_str};
use wasm_bindgen::{JsCast, JsValue};

use self::child_vec::ChildVec;
use super::Node;
use crate::{
    attribute::Attribute,
    hydration::{
        node::{DryNode, HydrationElement, HydrationText, Namespace},
        HydrationStats,
    },
    task,
};

mod child_vec;
mod optional_children;

pub use optional_children::OptionalChildren;

/// Build an HTML element.
pub struct ElementBuilderBase {
    element: Element,
    has_preceding_children: bool,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
}

/// An HTML element tag.
///
/// For example: `tag("div")`
pub fn tag(name: &str) -> ElementBuilderBase {
    ElementBuilderBase::new(name)
}

/// An HTML element tag in a namespace.
///
/// For example: `tag_in_namespace("http://www.w3.org/2000/svg", "svg")`
pub fn tag_in_namespace(namespace: Option<&'static str>, name: &str) -> ElementBuilderBase {
    ElementBuilderBase::new_in_namespace(namespace, name)
}

impl ElementBuilderBase {
    fn new(tag: &str) -> Self {
        Self::new_element(HydrationElement::new(Namespace::Html, tag))
    }

    fn new_in_namespace(namespace: Option<&'static str>, tag: &str) -> Self {
        Self::new_element(HydrationElement::new(Namespace::Other(namespace), tag))
    }

    fn new_element(hydro_elem: HydrationElement) -> Self {
        Self {
            element: Element {
                hydro_elem,
                futures: Vec::new(),
            },
            has_preceding_children: false,
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        }
    }

    fn check_attribute_unique(&mut self, name: &str) {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));
        let _ = name;
    }
}

impl ParentBuilder for ElementBuilderBase {
    /// Add a child element after existing children.
    fn child(mut self, child: impl Into<Node>) -> Self {
        self.has_preceding_children = true;

        let mut child = child.into();
        self.element.futures.extend(child.take_futures());

        self.element.hydro_elem.append_child_now(&child);
        self.element.hydro_elem.store_child(child.into_hydro());

        self
    }

    fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<Node>> + 'static,
    ) -> Self::Target {
        let mut child_vec =
            ChildVec::new(self.element.hydro_elem.clone(), self.has_preceding_children);

        let updater = children.for_each(move |update| {
            child_vec.apply_update(update);
            async {}
        });

        self.spawn_future(updater).build()
    }

    /// Add a text node after existing children.
    fn text(mut self, child: &str) -> Self {
        self.has_preceding_children = true;
        self.element
            .hydro_elem
            .append_child_now(HydrationText::new(child));
        self
    }

    fn text_signal(
        mut self,
        child_signal: impl Signal<Item = impl Into<String>> + 'static,
    ) -> Self {
        self.has_preceding_children = true;

        let mut text_node = HydrationText::new(intern_str(""));
        self.element.hydro_elem.append_child_now(text_node.clone());

        let updater = child_signal.for_each({
            move |new_value| {
                text_node.set_text(new_value.into());
                async {}
            }
        });

        self.spawn_future(updater)
    }

    fn optional_children(mut self, mut children: OptionalChildren) -> Self::Target {
        self.element.futures.append(&mut children.futures);
        self.children_signal(
            children
                .items
                .borrow()
                .signal_vec_cloned()
                .filter_map(|mut e| e.borrow_mut().take()),
        )
    }
}

impl ElementBuilder for ElementBuilderBase {
    type DomType = web_sys::Element;
    type Target = Element;

    fn attribute<T: Attribute>(mut self, name: &str, value: T) -> Self {
        self.check_attribute_unique(name);

        self.element.hydro_elem.attribute_now(name, value);
        self
    }

    fn attribute_signal<T: Attribute + 'static>(
        mut self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self {
        self.check_attribute_unique(name);
        let mut element = self.element.hydro_elem.clone();

        let updater = value.for_each({
            let name = name.to_owned();

            move |new_value| {
                element.attribute(&name, new_value);

                async {}
            }
        });

        self.spawn_future(updater)
    }

    fn effect(mut self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
        self.element.hydro_elem.effect(f);
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
        let mut element = self.element.hydro_elem.clone();

        let future = sig.for_each(move |x| {
            clone!(f);
            element.effect(move |elem| f(elem, x));
            async {}
        });

        self.spawn_future(future)
    }

    fn spawn_future(mut self, future: impl Future<Output = ()> + 'static) -> Self {
        self.element.futures.push(spawn_cancelable_future(future));
        self
    }

    fn on(mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self {
        self.element.hydro_elem.on(name, f);
        self
    }

    fn build(mut self) -> Self::Target {
        self.element.futures.shrink_to_fit();
        self.element.hydro_elem.shrink_to_fit();
        self.element
    }
}

impl From<ElementBuilderBase> for Element {
    fn from(builder: ElementBuilderBase) -> Self {
        builder.build()
    }
}

impl From<ElementBuilderBase> for Node {
    fn from(builder: ElementBuilderBase) -> Self {
        builder.build().into()
    }
}

/// An HTML element.
pub struct Element {
    pub(super) hydro_elem: HydrationElement,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

impl Element {
    pub(crate) fn eval_dom_element(&self) -> web_sys::Element {
        self.hydro_elem.eval_dom_element()
    }

    pub(crate) fn hydrate_child(
        &self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> web_sys::Element {
        self.hydro_elem.hydrate_child(parent, child, tracker)
    }

    pub(super) fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        mem::take(&mut self.futures)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.hydro_elem.fmt(f)
    }
}

/// An HTML element builder.
pub trait ElementBuilder: Sized {
    type Target;
    type DomType: JsCast + 'static;

    /// Set an attribute
    fn attribute<T: Attribute>(self, name: &str, value: T) -> Self;

    /// Set an attribute based on a signal. `Option<impl Attribute>` can be used
    /// to add/remove an attribute based on a signal.
    fn attribute_signal<T: Attribute + 'static>(
        self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
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
    /// # use silkenweb::{elements::html::input, node::element::ElementBuilder};
    ///
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

    fn build(self) -> Self::Target;
}

/// An element that is allowed to have children.
pub trait ParentBuilder: ElementBuilder {
    /// Add a text child to this element
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use silkenweb::{elements::html::div, node::element::ParentBuilder};
    /// div().text("Hello, world!");
    /// ```
    fn text(self, child: &str) -> Self;

    /// Add a text signal child to this element
    ///
    /// The text will update when the signal is updated.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{elements::html::div, node::element::ParentBuilder};
    /// let hello = Mutable::new("Hello");
    ///
    /// div().text_signal(hello.signal());
    /// ```
    fn text_signal(self, child: impl Signal<Item = impl Into<String>> + 'static) -> Self;

    /// Add a child node to the element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::{div, p},
    /// #     node::element::ParentBuilder,
    /// # };
    ///
    /// div().child(p().text("Hello,")).child(p().text("world!"));
    /// ```
    fn child(self, c: impl Into<Node>) -> Self;

    /// Add a children node to the element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::{div, p},
    /// #     node::element::ParentBuilder,
    /// # };
    ///
    /// div().children([p().text("Hello,"), p().text("world!")]);
    /// ```
    fn children(mut self, children: impl IntoIterator<Item = impl Into<Node>>) -> Self {
        for child in children {
            self = self.child(child);
        }

        self
    }

    /// Add [`SignalVec`] children to the element.
    ///
    /// See [counter_list](https://github.com/silkenweb/silkenweb/tree/main/examples/counter-list/src/main.rs)
    /// for an example
    fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<Node>> + 'static,
    ) -> Self::Target;

    /// Add [`OptionalChildren`]
    ///
    /// Sometimes element children are optional, dependant on the value of a
    /// signal for example.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use silkenweb::{
    /// #     elements::html::div,
    /// #     node::{
    /// #         element::{OptionalChildren, ParentBuilder},
    /// #         text,
    /// #     },
    /// # };
    ///
    /// let include_child1 = Mutable::new(true);
    /// let include_child2 = Mutable::new(true);
    ///
    /// div().optional_children(
    ///     OptionalChildren::new()
    ///         .optional_child_signal(
    ///             include_child1
    ///                 .signal()
    ///                 .map(|child1| child1.then(|| text("This is child1"))),
    ///         )
    ///         .optional_child_signal(
    ///             include_child2
    ///                 .signal()
    ///                 .map(|child1| child1.then(|| text("This is child2"))),
    ///         ),
    /// );
    /// ```
    fn optional_children(self, children: OptionalChildren) -> Self::Target;
}

fn spawn_cancelable_future(
    future: impl Future<Output = ()> + 'static,
) -> DiscardOnDrop<CancelableFutureHandle> {
    let (handle, cancelable_future) = cancelable_future(future, || ());

    task::spawn_local(cancelable_future);

    handle
}
