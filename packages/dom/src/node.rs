use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;

use self::{element::Element, text::Text};
use crate::hydration::{
    lazy::IsDry,
    node::{DryNode, HydrationNode, HydrationNodeData, WetNode},
};

pub mod element;
pub mod text;

pub struct Node(NodeEnum);

impl Node {
    pub fn eval_dom_node(&self) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.eval_dom_element().into(),
            NodeEnum::Text(text) => text.eval_dom_text().into(),
        }
    }

    fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        match &mut self.0 {
            NodeEnum::Element(elem) => elem.take_futures(),
            NodeEnum::Text(text) => text.take_futures(),
        }
    }
}

impl HydrationNode for Node {}

impl IsDry for Node {
    fn is_dry(&self) -> bool {
        match &self.0 {
            NodeEnum::Element(elem) => elem.hydro_elem.is_dry(),
            NodeEnum::Text(text) => text.hydro_text.is_dry(),
        }
    }
}

impl DryNode for Node {
    fn clone_into_hydro(&self) -> HydrationNodeData {
        match &self.0 {
            NodeEnum::Element(elem) => elem.hydro_elem.clone_into_hydro(),
            NodeEnum::Text(text) => text.hydro_text.clone_into_hydro(),
        }
    }

    fn into_hydro(self) -> HydrationNodeData {
        match self.0 {
            NodeEnum::Element(elem) => elem.hydro_elem.into_hydro(),
            NodeEnum::Text(text) => text.hydro_text.into_hydro(),
        }
    }
}

impl WetNode for Node {
    fn dom_node(&self) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.hydro_elem.dom_node(),
            NodeEnum::Text(text) => text.hydro_text.dom_node(),
        }
    }
}

impl From<Element> for Node {
    fn from(elem: Element) -> Self {
        Self(NodeEnum::Element(elem))
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self(NodeEnum::Text(text))
    }
}

enum NodeEnum {
    Element(Element),
    Text(Text),
}
