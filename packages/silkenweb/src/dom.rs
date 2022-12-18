use std::marker::PhantomData;

use self::{
    dry::{DryElement, DryNode, DryText},
    hydro::{HydroElement, HydroNode, HydroText},
    template::{TemplateElement, TemplateNode, TemplateText},
    wet::{WetElement, WetNode, WetText},
};

pub(super) mod private;

mod dry;
mod hydro;
mod template;
mod wet;

pub trait Dom: private::Dom {}

pub trait InstantiableDom: Dom + private::InstantiableDom {}

pub type DefaultDom = Wet;

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

pub struct Template<D: InstantiableDom, Param>(PhantomData<(D, Param)>);

impl<D: InstantiableDom, Param: 'static> Dom for Template<D, Param> {}

impl<D: InstantiableDom, Param: 'static> private::Dom for Template<D, Param> {
    type Element = TemplateElement<D, Param>;
    type Node = TemplateNode<D, Param>;
    type Text = TemplateText<D>;
}
