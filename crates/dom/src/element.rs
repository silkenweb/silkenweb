#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::{self, cell::RefCell, future::Future, rc::Rc};

use discard::DiscardOnDrop;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
    CancelableFutureHandle,
};
use wasm_bindgen::{intern, JsCast, JsValue, UnwrapThrowExt};
use web_sys as dom;

use self::{child_groups::ChildGroups, child_vec::ChildVec, event::EventCallback};
use crate::{
    attribute::{Attribute, StaticAttribute},
    clone, document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

mod child_groups;
mod child_vec;
mod dom_children;
mod event;

/// Build an HTML element.
pub struct ElementBuilder {
    element: Element,
    child_groups: Rc<RefCell<ChildGroups>>,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
}

impl ElementBuilder {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document().create_element(tag).unwrap_throw())
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(
            document()
                .create_element_ns(Some(namespace), tag)
                .unwrap_throw(),
        )
    }

    fn new_element(dom_element: dom::Element) -> Self {
        Self {
            element: Element {
                dom_element: dom_element.clone(),
                event_callbacks: Vec::new(),
                futures: Vec::new(),
            },
            child_groups: Rc::new(RefCell::new(ChildGroups::new(dom_element))),
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        }
    }

    /// Set an attribute. Attribute values can be reactive.
    pub fn attribute<U, T: StaticAttribute<U>>(self, name: &str, value: impl Attribute<T>) -> Self {
        self.tagged_attribute(name, value)
    }

    pub fn tagged_attribute<T>(mut self, name: &str, value: impl Attribute<T>) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));

        value.set_attribute(name, &mut self);
        self
    }

    /// Add a child element after existing children.
    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();
        self.child_groups
            .borrow_mut()
            .append_new_group_sync(child.dom_element());
        self.element.event_callbacks.extend(child.event_callbacks);
        self.element.futures.extend(child.futures);

        self
    }

    pub fn child_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = impl Into<Element>>,
    ) -> Self {
        let group_index = self.child_groups.borrow_mut().new_group();
        let children = self.child_groups.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            let child = child.into();
            children
                .borrow_mut()
                .upsert_only_child(group_index, child.dom_element());
            _child_storage = Some(child);
            async {}
        });

        self.store_future(updater);

        self
    }

    pub fn optional_child_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = Option<impl Into<Element>>>,
    ) -> Self {
        let group_index = self.child_groups.borrow_mut().new_group();
        let children = self.child_groups.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            if let Some(child) = child {
                let child = child.into();
                children
                    .borrow_mut()
                    .upsert_only_child(group_index, child.dom_element());
                _child_storage = Some(child);
            } else {
                children.borrow_mut().remove_child(group_index);
                _child_storage = None;
            }

            async {}
        });

        self.store_future(updater);

        self
    }

    // TODO: Docs
    // TODO: tests
    pub fn children_signal(
        mut self,
        children: impl 'static + SignalVec<Item = impl Into<Element>>,
    ) -> Self {
        let group_index = self.child_groups.borrow_mut().new_group();
        let child_vec = ChildVec::new(
            self.dom_element().clone(),
            self.child_groups.clone(),
            group_index,
        );

        let updater = children.for_each(move |update| {
            child_vec.borrow_mut().apply_update(update);
            async {}
        });

        self.store_future(updater);
        self
    }

    /// Add a text node after existing children.
    pub fn text(self, child: &str) -> Self {
        let text_node = document().create_text_node(child);
        self.child_groups
            .borrow_mut()
            .append_new_group_sync(&text_node);
        self
    }

    pub fn text_signal(
        mut self,
        child_signal: impl 'static + Signal<Item = impl Into<String>>,
    ) -> Self {
        let text_node = document().create_text_node(intern(""));
        self.child_groups
            .borrow_mut()
            .append_new_group_sync(&text_node);

        let updater = child_signal.for_each({
            clone!(text_node);

            move |new_value| {
                queue_update({
                    clone!(text_node);
                    let new_value = new_value.into();
                    move || text_node.set_data(&new_value)
                });
                async {}
            }
        });

        self.store_future(updater);
        self
    }

    // TODO: Make this public? It might be useful if we have an expensive to compute
    // signal that we want to store in a mutable.
    fn store_future(&mut self, future: impl 'static + Future<Output = ()>) {
        self.element.futures.push(spawn_cancelable_future(future));
    }

    // TODO: Test
    /// Apply an effect after the next render. For example, to set the focus of
    /// an element:
    ///
    /// ```no_run
    /// # use silkenweb_dom::tag;
    /// # use web_sys::HtmlInputElement;
    /// # use wasm_bindgen::UnwrapThrowExt;
    /// # let element = tag("input");
    /// element.effect(|elem: &HtmlInputElement| elem.focus().unwrap_throw());
    /// ```
    ///
    /// Effects can be reactive. For example, to set the visibibilty of an item
    /// based on a `hidden` boolean signal:
    ///
    /// ```no_run
    /// # use silkenweb_dom::tag;
    /// # use futures_signals::signal::{Mutable, SignalExt};
    /// # use web_sys::HtmlInputElement;
    /// # let element = tag("input");
    /// let is_hidden = Mutable::new(false);
    ///
    /// element.effect_signal(is_hidden.signal(), move |elem: &HtmlInputElement, is_hidden| elem.set_hidden(is_hidden));
    /// ```
    pub fn effect<DomType: 'static + JsCast>(self, f: impl 'static + FnOnce(&DomType)) -> Self {
        let dom_element = self.dom_element().clone().dyn_into().unwrap_throw();
        after_render(move || f(&dom_element));

        self
    }

    // TODO: Test
    pub fn effect_signal<T, DomType>(
        mut self,
        sig: impl 'static + Signal<Item = T>,
        f: impl 'static + Clone + Fn(&DomType, T),
    ) -> Self
    where
        T: 'static,
        DomType: 'static + Clone + JsCast,
    {
        let dom_element: DomType = self.dom_element().clone().dyn_into().unwrap_throw();

        let future = sig.for_each(move |x| {
            clone!(dom_element, f);
            after_render(move || f(&dom_element, x));
            async {}
        });

        self.store_future(future);

        self
    }

    /// Register an event handler.
    ///
    /// `name` is the name of the event. See the [MDN Events] page for a list.
    ///
    /// `f` is the callback when the event fires and will be passed the
    /// javascript `Event` object.
    ///
    /// [MDN Events]: https://developer.mozilla.org/en-US/docs/Web/Events
    pub fn on(mut self, name: &'static str, f: impl 'static + FnMut(JsValue)) -> Self {
        {
            let dom_element = self.element.dom_element.clone();
            self.element
                .event_callbacks
                .push(EventCallback::new(dom_element, name, f));
        }

        self
    }
}

impl Builder for ElementBuilder {
    type Target = Element;

    fn build(mut self) -> Self::Target {
        self.element.futures.shrink_to_fit();
        self.element.event_callbacks.shrink_to_fit();
        self.child_groups.borrow_mut().shrink_to_fit();
        self.element
    }

    fn into_element(self) -> Element {
        self.build()
    }
}

impl DomElement for ElementBuilder {
    type Target = dom::Element;

    fn dom_element(&self) -> &Self::Target {
        &self.element.dom_element
    }
}

impl From<ElementBuilder> for Element {
    fn from(builder: ElementBuilder) -> Self {
        builder.build()
    }
}

/// An HTML element.
///
/// Elements can only appear once in the document. If an element is added again,
/// it will be moved.
pub struct Element {
    dom_element: dom::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<SignalHandle>,
}

impl DomElement for Element {
    type Target = dom::Element;

    fn dom_element(&self) -> &Self::Target {
        &self.dom_element
    }
}

impl Builder for Element {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }

    fn into_element(self) -> Element {
        self
    }
}

/// Get a raw Javascript, non-reactive DOM element.
pub trait DomElement {
    type Target: Into<dom::Element> + AsRef<dom::Element> + Clone;

    fn dom_element(&self) -> &Self::Target;
}

/// An HTML element builder.
pub trait Builder {
    type Target;

    fn build(self) -> Self::Target;

    fn into_element(self) -> Element;
}

type SignalHandle = DiscardOnDrop<CancelableFutureHandle>;

impl<Sig, Attr, T> Attribute<T> for SignalType<Sig>
where
    Attr: 'static + Clone + StaticAttribute<T>,
    Sig: 'static + Signal<Item = Attr>,
{
    fn set_attribute(self, name: &str, builder: &mut ElementBuilder) {
        let dom_element = builder.dom_element().clone();

        let updater = self.0.for_each({
            let name = name.to_owned();

            move |new_value| {
                clone!(name, dom_element, new_value);

                queue_update(move || {
                    StaticAttribute::set_attribute(&new_value, &name, &dom_element);
                });

                async {}
            }
        });

        builder.store_future(updater);
    }
}

pub struct SignalType<T>(T);

/// Create a newtype wrapper around a signal.
pub fn signal<Sig: Signal<Item = T>, T>(sig: Sig) -> SignalType<Sig> {
    SignalType(sig)
}

impl<Sig, Attr, T> Attribute<T> for OptionalSignalType<Sig>
where
    Attr: 'static + Clone + StaticAttribute<T>,
    Sig: 'static + Signal<Item = Option<Attr>>,
{
    fn set_attribute(self, name: &str, builder: &mut ElementBuilder) {
        let dom_element = builder.dom_element().clone();

        let updater = self.0.for_each({
            let name = name.to_owned();

            move |new_value| {
                clone!(name, dom_element, new_value);

                queue_update(move || match new_value {
                    Some(value) => StaticAttribute::set_attribute(&value, &name, &dom_element),
                    None => dom_element.remove_attribute(&name).unwrap_throw(),
                });

                async {}
            }
        });

        builder.store_future(updater);
    }
}

pub struct OptionalSignalType<T>(T);

/// Create a newtype wrapper around an optional signal.
pub fn optional_signal<Sig: Signal<Item = T>, T>(sig: Sig) -> OptionalSignalType<Sig> {
    OptionalSignalType(sig)
}
