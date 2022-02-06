use silkenweb::html_element;

html_element!(
    ui5-icon<web_sys::HtmlElement> {
        attributes {
            accessible-name: String,
            interactive: bool,
            name: String,
            show-tooltip: bool,
        }
    }
);
