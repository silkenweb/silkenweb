use silkenweb::html_element;
use web_sys as dom;

html_element!(
    ui5-icon<dom::HtmlElement> {
        attributes {
            accessible-name: String,
            interactive: bool,
            name: String,
            show-tooltip: bool,
        }
    }
);
