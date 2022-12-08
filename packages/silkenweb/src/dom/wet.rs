use super::{Dom, DomElement};
use crate::hydration::node::Namespace;

pub struct Wet;

impl Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

pub struct WetElement {}

impl DomElement for WetElement {
    fn new(_ns: Namespace, _tag: &str) -> Self {
        WetElement {}
    }
}

// TODO: Is `Clone` required?
impl Clone for WetElement {
    fn clone(&self) -> Self {
        Self {}
    }
}

pub struct WetText {}
pub struct WetNode {}
