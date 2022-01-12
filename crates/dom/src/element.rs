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

use self::{child_groups::ChildGroups, child_vec::ChildVec, event::EventCallback};
use crate::{
    attribute::Attribute,
    clone,
    global::document,
    render::{after_render, queue_update},
    spawn_cancelable_future,
};

mod child_groups;
mod child_vec;
mod dom_children;
mod event;

/// Build an HTML element.
pub struct GenericElementBuilder {
    element: Element,
    child_groups: Rc<RefCell<ChildGroups>>,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
}

impl GenericElementBuilder {
    pub fn new(tag: &str) -> Self {
        Self::new_element(document::create_element(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(document::create_element_ns(namespace, tag))
    }

    fn new_element(dom_element: web_sys::Element) -> Self {
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

    /// Add a child element after existing children.
    pub fn child(mut self, child: impl Into<Element>) -> Self {
        let child = child.into();
        self.child_groups
            .borrow_mut()
            .append_new_group_sync(&child.dom_element);
        self.element.event_callbacks.extend(child.event_callbacks);
        self.element.futures.extend(child.futures);

        self
    }

    pub fn child_signal(
        mut self,
        child_signal: impl Signal<Item = impl Into<Element>> + 'static,
    ) -> Self {
        let group_index = self.child_groups.borrow_mut().new_group();
        let children = self.child_groups.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            let child = child.into();
            children
                .borrow_mut()
                .upsert_only_child(group_index, &child.dom_element);
            _child_storage = Some(child);
            async {}
        });

        self.spawn_future(updater);

        self
    }

    pub fn optional_child_signal(
        mut self,
        child_signal: impl Signal<Item = Option<impl Into<Element>>> + 'static,
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
                    .upsert_only_child(group_index, &child.dom_element);
                _child_storage = Some(child);
            } else {
                children.borrow_mut().remove_child(group_index);
                _child_storage = None;
            }

            async {}
        });

        self.spawn_future(updater);

        self
    }

    // TODO: Docs
    // TODO: tests
    pub fn children_signal(
        mut self,
        children: impl SignalVec<Item = impl Into<Element>> + 'static,
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

        self.spawn_future(updater);
        self
    }

    /// Add a text node after existing children.
    pub fn text(self, child: &str) -> Self {
        let text_node = document::create_text_node(child);
        self.child_groups
            .borrow_mut()
            .append_new_group_sync(&text_node);
        self
    }

    pub fn text_signal(
        mut self,
        child_signal: impl Signal<Item = impl Into<String>> + 'static,
    ) -> Self {
        let text_node = document::create_text_node(intern(""));
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

        self.spawn_future(updater);
        self
    }

    pub fn spawn_future(&mut self, future: impl Future<Output = ()> + 'static) {
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
    /// element.effect_signal(
    ///     is_hidden.signal(),
    ///     move |elem: &HtmlInputElement, is_hidden| elem.set_hidden(is_hidden),
    /// );
    /// ```
    pub fn effect<DomType: JsCast + 'static>(self, f: impl FnOnce(&DomType) + 'static) -> Self {
        let dom_element = self.dom_element().clone().dyn_into().unwrap_throw();
        after_render(move || f(&dom_element));

        self
    }

    // TODO: Test
    pub fn effect_signal<T, DomType>(
        mut self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Clone + Fn(&DomType, T) + 'static,
    ) -> Self
    where
        T: 'static,
        DomType: Clone + JsCast + 'static,
    {
        let dom_element: DomType = self.dom_element().clone().dyn_into().unwrap_throw();

        let future = sig.for_each(move |x| {
            clone!(dom_element, f);
            after_render(move || f(&dom_element, x));
            async {}
        });

        self.spawn_future(future);

        self
    }

    fn check_attribute_unique(&mut self, name: &str) {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));
        let _ = name;
    }

    fn dom_element(&self) -> &web_sys::Element {
        &self.element.dom_element
    }
}

impl ElementBuilder for GenericElementBuilder {
    type Target = Element;

    fn attribute<T: Attribute>(mut self, name: &str, value: T) -> Self {
        self.check_attribute_unique(name);

        value.set_attribute(name, self.dom_element());
        self
    }

    fn attribute_signal<T: Attribute + 'static>(
        mut self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self {
        self.check_attribute_unique(name);
        let dom_element = self.dom_element().clone();

        let updater = value.for_each({
            let name = name.to_owned();

            move |new_value| {
                clone!(name, dom_element);

                queue_update(move || new_value.set_attribute(&name, &dom_element));

                async {}
            }
        });

        self.spawn_future(updater);
        self
    }

    fn on(mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self {
        {
            let dom_element = self.element.dom_element.clone();
            self.element
                .event_callbacks
                .push(EventCallback::new(dom_element, name, f));
        }

        self
    }

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

impl From<GenericElementBuilder> for Element {
    fn from(builder: GenericElementBuilder) -> Self {
        builder.build()
    }
}

/// An HTML element.
///
/// Elements can only appear once in the document. If an element is added again,
/// it will be moved.
pub struct Element {
    pub(super) dom_element: web_sys::Element,
    event_callbacks: Vec<EventCallback>,
    futures: Vec<DiscardOnDrop<CancelableFutureHandle>>,
}

/// An HTML element builder.
pub trait ElementBuilder: Sized {
    type Target;

    fn attribute<T: Attribute>(self, name: &str, value: T) -> Self;

    fn attribute_signal<T: Attribute + 'static>(
        self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self;

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

    fn into_element(self) -> Element;
}
