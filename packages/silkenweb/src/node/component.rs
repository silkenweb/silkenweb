use super::{
    element::{GenericElement, ParentElement, ShadowRootParent},
    Node,
};
use crate::{
    dom::{DefaultDom, InstantiableDom},
    elements::{
        html::{div, slot, style, Div, Slot},
        HtmlElement,
    },
    value::Value,
};

// TODO: Docs
pub struct Component<D: InstantiableDom = DefaultDom> {
    element: Option<Div<D>>,
    id: usize,
}

impl<D: InstantiableDom> Component<D> {
    // TODO: Docs
    pub fn new() -> Self {
        Self {
            element: Some(div()),
            id: 0,
        }
    }

    // TODO: Docs
    pub fn styled(css: &str) -> Self {
        Self {
            element: Some(div().child(style().text(css))),
            id: 0,
        }
    }

    // TODO: Docs
    pub fn slot(&mut self, child: impl HtmlElement + Into<Node<D>> + Value + 'static) -> Slot {
        let id = self.id.to_string();
        self.id += 1;
        self.element = Some(self.element.take().unwrap().child(child.slot(&id)));
        slot().name(id)
    }

    // TODO: `multi_slot`
    // TODO: `chidlren`

    // TODO: Docs
    pub fn child(self, child: impl Into<Node<D>> + Value + 'static) -> Self {
        Self {
            element: self
                .element
                .map(|elem| elem.attach_shadow_children([child])),
            id: self.id,
        }
    }
}

impl<D: InstantiableDom> Default for Component<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: InstantiableDom> Value for Component<D> {}

impl<D: InstantiableDom> From<Component<D>> for GenericElement<D> {
    fn from(value: Component<D>) -> Self {
        value.element.unwrap().into()
    }
}
