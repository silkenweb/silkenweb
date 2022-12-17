pub(crate) mod private;

pub mod dry;
pub mod hydro;
pub mod wet;

pub trait Dom: private::Dom {}

pub trait InstantiableDom: Dom + private::InstantiableDom {}

pub type DefaultDom = wet::Wet;

trait TrackSibling: Clone {
    fn set_next_sibling(&self, next_sibling: Option<&Self>);
}
