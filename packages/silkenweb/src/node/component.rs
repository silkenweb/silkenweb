use include_doc::function_body;

use super::{
    element::{Const, GenericElement, ParentElement, ShadowRootParent, TextParentElement},
    ChildNode, Node,
};
use crate::{
    dom::{DefaultDom, InDom, InstantiableDom},
    elements::{
        html::{div, slot, style, Div, Slot},
        HtmlElement,
    },
    value::Value,
};

/// A lightweight type to encapsulate HTML and CSS using shadow DOM.
///
/// See [Using Shadow DOM] for some background on shadow DOM.
///
/// The [`slot`][`Self::slot`] and [`multi_slot`][`Self::multi_slot`] methods
/// generate a slot name and assign it to their arguments. They return a
/// slot, which should be added to the shadow DOM with the
/// [`child`][`Self::child`] or [`children`][`Self::children`] methods.
///
/// # Example
///
/// This creates a `<div>` with an open shadow root attached. The light DOM
/// will contain the `name` and `description` spans, assigned to a slot each.
/// The shadow DOM will contain the `<dl>`, `<dt>`, and `<dd>` elements and
/// slots for the light DOM elements. HTML `id` attributes and CSS are
/// encapsulated by the shadow DOM.
///
/// ```
#[doc = function_body!("tests/doc/node/component.rs", component, [])]
/// ```
/// 
/// [Using Shadow DOM]: https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_shadow_DOM
pub struct Component<D: InstantiableDom = DefaultDom> {
    element: Option<Div<D>>,
    id: usize,
}

impl<D: InstantiableDom> Component<D> {
    /// Constructor
    pub fn new() -> Self {
        Self {
            element: Some(div()),
            id: 0,
        }
    }

    /// Constuct a [`Component`] with a `<style>` element in the shadow DOM.
    pub fn styled(css: &str) -> Self {
        Self::new().child(style().text(css))
    }

    /// Add `child` to the light DOM.
    ///
    /// See [`Component`] documentation for more details.
    pub fn slot(&mut self, child: impl HtmlElement + ChildNode<D>) -> Slot<D> {
        let id = self.new_id();
        self.element = Some(self.element.take().unwrap().child(child.slot(&id)));
        slot().name(id)
    }

    /// Add `children` to the light DOM.
    ///
    /// See [`Component`] documentation for more details.
    pub fn multi_slot<E>(&mut self, children: impl IntoIterator<Item = E>) -> Slot<D>
    where
        E: HtmlElement + ChildNode<D>,
    {
        let id = self.new_id();

        self.element = Some(
            self.element
                .take()
                .unwrap()
                .children(children.into_iter().map(|child| child.slot(&id))),
        );
        slot().name(id)
    }

    /// Add `child` to the shadow DOM.
    pub fn child(self, child: impl ChildNode<D>) -> Self {
        Self {
            element: self
                .element
                .map(|elem| elem.attach_shadow_children([child])),
            id: self.id,
        }
    }

    /// Add `children` to the shadow DOM.
    pub fn children<N>(self, children: impl IntoIterator<Item = N> + 'static) -> Self
    where
        N: ChildNode<D>,
    {
        Self {
            element: self
                .element
                .map(|elem| elem.attach_shadow_children(children)),
            id: self.id,
        }
    }

    fn new_id(&mut self) -> String {
        let id = self.id.to_string();
        self.id += 1;
        id
    }
}

impl<D: InstantiableDom> Default for Component<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: InstantiableDom> Value for Component<D> {}

impl<D: InstantiableDom> InDom for Component<D> {
    type Dom = D;
}

impl<D: InstantiableDom> From<Component<D>> for GenericElement<D> {
    fn from(value: Component<D>) -> Self {
        value.element.unwrap().into()
    }
}

impl<D: InstantiableDom> From<Component<D>> for GenericElement<D, Const> {
    fn from(value: Component<D>) -> Self {
        value.element.unwrap().into()
    }
}

impl<D: InstantiableDom> From<Component<D>> for Node<D> {
    fn from(value: Component<D>) -> Self {
        value.element.unwrap().into()
    }
}
