use silkenweb::{html_element, Builder};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys as dom;

html_element!(
    ui5-side-navigation<dom::HtmlElement> {
        attributes {
            collapsed: bool,
        }

        custom_events {
            selection-change: SelectionChanged,
        }
    }
);

impl Ui5SideNavigationBuilder {
    pub fn child(self, child: impl Builder<Target = Ui5SideNavigationItem>) -> Self {
        Self {
            builder: self.builder.child(child.build().into_element()),
        }
    }
}

html_element!(
    ui5-side-navigation-item<dom::HtmlElement> {
        attributes {
            expanded: bool,
            // TODO: enum for icons
            icon: String,
            selected: bool,
            text: String,
            whole-item-toggleable: bool
        }
    }
);

impl Ui5SideNavigationItemBuilder {
    pub fn child(self, child: impl Into<Ui5SideNavigationSubItem>) -> Self {
        Self {
            builder: self.builder.child(child.into().into_element()),
        }
    }
}

html_element!(
    ui5-side-navigation-sub-item<dom::HtmlElement> {
        attributes {
            expanded: bool,
            // TODO: enum for icons
            icon: String,
            selected: bool,
            text: String,
        }
    }
);

pub struct SelectionChanged {
    event: dom::CustomEvent,
}

impl SelectionChanged {
    pub fn event(&self) -> &dom::CustomEvent {
        &self.event
    }

    pub fn item(&self) -> dom::HtmlElement {
        self.event
            .detail()
            .unchecked_into::<Item>()
            .item()
            .dyn_into()
            .unwrap()
    }
}

impl From<dom::CustomEvent> for SelectionChanged {
    fn from(event: dom::CustomEvent) -> Self {
        Self { event }
    }
}

#[wasm_bindgen]
extern "C" {
    type Item;

    #[wasm_bindgen(structural, method, getter)]
    fn item(this: &Item) -> JsValue;
}
