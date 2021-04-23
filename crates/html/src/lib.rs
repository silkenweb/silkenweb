#![allow(clippy::must_use_candidate)]

#[macro_use]
pub mod macros;
pub mod elements;

use surfinia_core::{hooks::list_state::ElementList, Builder, Element, ElementBuilder};

pub fn element_list<Key, Value, GenerateChild, ChildElem, ParentElem>(
    root: ParentElem,
    generate_child: GenerateChild,
    initial: impl Iterator<Item = (Key, Value)>,
) -> ElementList<Key, Value>
where
    Value: 'static,
    ChildElem: Into<Element>,
    ParentElem: Into<ElementBuilder> + Builder,
    GenerateChild: 'static + Fn(&Value) -> ChildElem,
    Key: 'static + Ord + Eq + Clone,
{
    ElementList::new(root.into(), move |c| generate_child(c).into(), initial)
}
