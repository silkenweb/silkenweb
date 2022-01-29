use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;

use self::element::Element;
use crate::hydration::node::HydrationNodeData;

pub mod element;

pub struct Node(NodeEnum);

impl Node {
    pub fn eval_dom_node(&self) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.eval_dom_element().into(),
        }
    }

    fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        match &mut self.0 {
            NodeEnum::Element(elem) => elem.take_futures(),
        }
    }

    fn into_hydro(self) -> HydrationNodeData {
        match self.0 {
            NodeEnum::Element(elem) => elem.into_hydro(),
        }
    }

    fn clone_into_hydro(&self) -> HydrationNodeData {
        match &self.0 {
            NodeEnum::Element(elem) => elem.clone_into_hydro(),
        }
    }
}

impl From<Element> for Node {
    fn from(elem: Element) -> Self {
        Self(NodeEnum::Element(elem))
    }
}

enum NodeEnum {
    Element(Element),
}
