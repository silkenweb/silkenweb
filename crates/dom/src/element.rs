#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::{
    self,
    cell::{RefCell, RefMut},
    future::Future,
    rc::Rc,
};

use futures_signals::{
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use wasm_bindgen::{intern, JsCast, JsValue};

use self::{
    child_groups::ChildGroups,
    child_vec::ChildVec,
    strict::{StrictElement, StrictNode, StrictNodeRef, StrictText},
};
use crate::{attribute::Attribute, clone, render::queue_update};

mod child_groups;
mod child_vec;
mod event;
mod strict;

/// Build an HTML element.
pub struct ElementBuilderBase {
    element: Element,
    child_groups: Rc<RefCell<ChildGroups>>,
    #[cfg(debug_assertions)]
    attributes: HashSet<String>,
}

impl ElementBuilderBase {
    pub fn new(tag: &str) -> Self {
        Self::new_element(StrictElement::new(tag))
    }

    pub fn new_in_namespace(namespace: &str, tag: &str) -> Self {
        Self::new_element(StrictElement::new_in_namespace(namespace, tag))
    }

    fn new_element(element: StrictElement) -> Self {
        let node = element.clone_into_node();

        Self {
            element: Element(element),
            child_groups: Rc::new(RefCell::new(ChildGroups::new(node))),
            #[cfg(debug_assertions)]
            attributes: HashSet::new(),
        }
    }

    fn check_attribute_unique(&mut self, name: &str) {
        #[cfg(debug_assertions)]
        debug_assert!(self.attributes.insert(name.into()));
        let _ = name;
    }

    fn child_groups_mut(&self) -> RefMut<ChildGroups> {
        self.child_groups.as_ref().borrow_mut()
    }

    fn as_node_ref(&self) -> &StrictNode<web_sys::Element> {
        self.element.0.as_node_ref()
    }
}

impl ParentBuilder for ElementBuilderBase {
    /// Add a child element after existing children.
    fn child(self, child: impl Into<Element>) -> Self {
        let child = child.into();
        self.child_groups_mut().append_new_group_sync(&child.0);
        self.element.0.store_child(child.0);

        self
    }

    fn child_signal(self, child_signal: impl Signal<Item = impl Into<Element>> + 'static) -> Self {
        let group_index = self.child_groups_mut().new_group();
        let child_groups = self.child_groups.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            let child = child.into();
            child_groups
                .borrow_mut()
                .upsert_only_child(group_index, child.0.clone_into_node());
            _child_storage = Some(child);
            async {}
        });

        self.spawn_future(updater)
    }

    fn optional_child_signal(
        self,
        child_signal: impl Signal<Item = Option<impl Into<Element>>> + 'static,
    ) -> Self {
        let group_index = self.child_groups_mut().new_group();
        let child_groups = self.child_groups.clone();
        // Store the child in here until we replace it.
        let mut _child_storage = None;

        let updater = child_signal.for_each(move |child| {
            if let Some(child) = child {
                let child = child.into();
                child_groups
                    .borrow_mut()
                    .upsert_only_child(group_index, child.0.clone_into_node());
                _child_storage = Some(child);
            } else {
                child_groups.borrow_mut().remove_child(group_index);
                _child_storage = None;
            }

            async {}
        });

        self.spawn_future(updater)
    }

    fn children_signal(
        self,
        children: impl SignalVec<Item = impl Into<Element>> + 'static,
    ) -> Self {
        let group_index = self.child_groups_mut().new_group();
        let child_vec = ChildVec::new(
            self.element.0.clone_into_node(),
            self.child_groups.clone(),
            group_index,
        );

        let updater = children.for_each(move |update| {
            child_vec.as_ref().borrow_mut().apply_update(update);
            async {}
        });

        self.spawn_future(updater)
    }

    /// Add a text node after existing children.
    fn text(self, child: &str) -> Self {
        let text_node = StrictText::new(child);
        self.child_groups_mut().append_new_group_sync(&text_node);
        self
    }

    fn text_signal(self, child_signal: impl Signal<Item = impl Into<String>> + 'static) -> Self {
        let text_node = StrictText::new(intern(""));
        self.child_groups_mut().append_new_group_sync(&text_node);

        let updater = child_signal.for_each({
            clone!(text_node);

            move |new_value| {
                text_node.set_text(new_value.into());
                async {}
            }
        });

        self.spawn_future(updater)
    }
}

impl ElementBuilder for ElementBuilderBase {
    type DomType = web_sys::Element;
    type Target = Element;

    fn attribute<T: Attribute>(mut self, name: &str, value: T) -> Self {
        self.check_attribute_unique(name);

        self.as_node_ref().attribute(name, value);
        self
    }

    fn attribute_signal<T: Attribute + 'static>(
        mut self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self {
        self.check_attribute_unique(name);
        let element = self.as_node_ref().clone();

        let updater = value.for_each({
            let name = name.to_owned();

            move |new_value| {
                clone!(name, element);

                queue_update(move || element.attribute(&name, new_value));

                async {}
            }
        });

        self.spawn_future(updater)
    }

    /// Apply an effect after the next render.
    fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
        self.as_node_ref().effect(f);
        self
    }

    /// Apply an effect after the next render each time a singal yields a new
    /// value.
    fn effect_signal<T>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Clone + Fn(&Self::DomType, T) + 'static,
    ) -> Self
    where
        T: 'static,
    {
        let element = self.as_node_ref().clone();

        let future = sig.for_each(move |x| {
            clone!(f, element);
            element.effect(move |elem| f(elem, x));
            async {}
        });

        self.spawn_future(future)
    }

    fn spawn_future(mut self, future: impl Future<Output = ()> + 'static) -> Self {
        self.element.0.spawn_future(future);
        self
    }

    fn on(mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) -> Self {
        self.element.0.on(name, f);
        self
    }

    fn build(mut self) -> Self::Target {
        self.element.0.shrink_to_fit();
        self.child_groups_mut().shrink_to_fit();
        self.element
    }
}

impl From<ElementBuilderBase> for Element {
    fn from(builder: ElementBuilderBase) -> Self {
        builder.build()
    }
}

/// An HTML element.
///
/// Elements can only appear once in the document. If an element is added again,
/// it will be moved.
pub struct Element(strict::StrictElement);

impl Element {
    pub(super) fn eval_dom_element(&self) -> web_sys::Element {
        self.0.eval_dom_element()
    }
}

/// An HTML element builder.
pub trait ElementBuilder: Sized {
    type Target;
    type DomType: JsCast + 'static;

    fn attribute<T: Attribute>(self, name: &str, value: T) -> Self;

    fn attribute_signal<T: Attribute + 'static>(
        self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self;

    fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self;

    fn effect_signal<T: 'static>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Fn(&Self::DomType, T) + Clone + 'static,
    ) -> Self;

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

pub trait ParentBuilder: Sized {
    fn text(self, child: &str) -> Self;

    fn text_signal(self, child: impl Signal<Item = impl Into<String>> + 'static) -> Self;

    fn child(self, c: impl Into<Element>) -> Self;

    fn children(mut self, children: impl IntoIterator<Item = impl Into<Element>>) -> Self {
        for child in children {
            self = self.child(child);
        }

        self
    }

    fn child_signal(self, child: impl Signal<Item = impl Into<Element>> + 'static) -> Self;

    fn children_signal(self, children: impl SignalVec<Item = impl Into<Element>> + 'static)
        -> Self;

    fn optional_child_signal(
        self,
        child: impl Signal<Item = Option<impl Into<Element>>> + 'static,
    ) -> Self;
}
