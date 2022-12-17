pub(crate) mod private;

pub mod dry;
pub mod wet;

pub trait Dom: private::Dom {}

pub trait InstantiableDom: Dom + private::InstantiableDom {}

pub type DefaultDom = wet::Wet;
