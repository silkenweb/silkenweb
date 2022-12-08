use super::Dom;

pub struct Wet;

impl Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

pub struct WetElement {}
pub struct WetText {}
pub struct WetNode {}
