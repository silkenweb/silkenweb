use silkenweb::{dom_type, html_element};
use web_sys as dom;

html_element!(
    ui5-icon {
        accessible-name: String,
        interactive: bool,
        name: String,
        show-tooltip: bool,
    }
);

dom_type!(ui5-icon<dom::HtmlElement>);
