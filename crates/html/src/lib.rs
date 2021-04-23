#![allow(clippy::must_use_candidate)]

#[macro_use]
pub mod macros;
pub mod elements;

use surfinia_core::{hooks::list_state::ElementList, Builder, Element, ElementBuilder, Text};
use web_sys as dom;

pub fn element_list<Key, Value, GenerateChild, ChildElem, ParentElem>(
    root: ParentElem,
    generate_child: GenerateChild,
    initial: impl Iterator<Item = (Key, Value)>,
) -> ElementList<Key, ParentElem::Target, Value>
// TODO: Change order of type params
where
    Value: 'static,
    ChildElem: Into<Element>,
    ParentElem: Into<ElementBuilder> + Builder,
    GenerateChild: 'static + Fn(&Value) -> ChildElem,
    Key: 'static + Ord + Eq + Clone,
{
    ElementList::new(root.into(), move |c| generate_child(c).into(), initial)
}

html_element!(div {});
dom_type!(div<dom::HtmlDivElement>);
text_parent!(div);

html_element!(button {});
dom_type!(button<dom::HtmlButtonElement>);
text_parent!(button);

html_element!(section {});
dom_type!(section<dom::HtmlElement>);

html_element!(header {});
dom_type!(header<dom::HtmlElement>);

html_element!(footer {});
dom_type!(footer<dom::HtmlElement>);

html_element!(span {});
dom_type!(span<dom::HtmlSpanElement>);
text_parent!(span);

html_element!(h1 {});
dom_type!(h1<dom::HtmlHeadingElement>);
text_parent!(h1);

html_element!(strong {});
dom_type!(strong<dom::HtmlElement>);
text_parent!(strong);

html_element!(input {
    type_: String,
    placeholder: String,
    value: String,
    autofocus: bool,
    checked: bool,
    readonly: bool,
});
dom_type!(input<dom::HtmlInputElement>);

html_element!(label { for_: String });
dom_type!(label<dom::HtmlLabelElement>);
text_parent!(label);

html_element!(ul {});
dom_type!(ul<dom::HtmlUListElement>);
text_parent!(ul);

html_element!(li {});
dom_type!(li<dom::HtmlLiElement>);
text_parent!(li);

html_element!(a {});
dom_type!(a<dom::HtmlAnchorElement>);
text_parent!(a);
