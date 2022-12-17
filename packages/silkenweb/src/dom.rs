use self::{
    dry::{DryElement, DryNode, DryText},
    hydro::{HydroElement, HydroNode, HydroText},
    wet::{WetElement, WetNode, WetText},
};

pub(crate) mod private;

mod dry;
mod hydro;
mod wet;

pub trait Dom: private::Dom {}

pub trait InstantiableDom: Dom + private::InstantiableDom {}

pub type DefaultDom = Wet;

trait TrackSibling: Clone {
    fn set_next_sibling(&self, next_sibling: Option<&Self>);
}

pub struct Dry;

impl Dom for Dry {}

impl private::Dom for Dry {
    type Element = DryElement;
    type Node = DryNode;
    type Text = DryText;
}

impl InstantiableDom for Dry {}

impl private::InstantiableDom for Dry {
    type InstantiableElement = DryElement;
    type InstantiableNode = DryNode;
}

pub struct Hydro;

impl Dom for Hydro {}

impl private::Dom for Hydro {
    type Element = HydroElement;
    type Node = HydroNode;
    type Text = HydroText;
}

impl InstantiableDom for Hydro {}

impl private::InstantiableDom for Hydro {
    type InstantiableElement = HydroElement;
    type InstantiableNode = HydroNode;
}

pub struct Wet;

impl Dom for Wet {}

impl private::Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

impl InstantiableDom for Wet {}

impl private::InstantiableDom for Wet {
    type InstantiableElement = WetElement;
    type InstantiableNode = WetNode;
}
