use super::Dom;

pub struct Dry;

impl Dom for Dry {
    type Element = DryElement;
    type Node = DryNode;
    type Text = DryText;
}

pub struct DryElement {}
pub struct DryText {}
pub struct DryNode {}
