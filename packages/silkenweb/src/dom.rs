pub mod dry;
pub mod wet;

pub trait Dom {
    type Element;
    type Text;
    type Node;
}
