use self::element::Element;

pub mod element;

pub struct Node(NodeEnum);

impl Node {
    pub fn eval_dom_node(&self) -> web_sys::Node {
        match &self.0 {
            NodeEnum::Element(elem) => elem.eval_dom_element().into(),
        }
    }
}

enum NodeEnum {
    Element(Element),
}

impl From<Element> for Node {
    fn from(elem: Element) -> Self {
        Self(NodeEnum::Element(elem))
    }
}
