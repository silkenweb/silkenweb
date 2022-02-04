use silkenweb::{
    dom::node::element::{ElementBuilder, ParentBuilder},
    elements::html_element,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};

html_element!(
    ui5-side-navigation<web_sys::HtmlElement> {
        attributes {
            collapsed: bool,
        }

        custom_events {
            selection-change: SelectionChanged,
        }
    }
);

impl Ui5SideNavigationBuilder {
    pub fn child(self, child: impl ElementBuilder<Target = Ui5SideNavigationItem>) -> Self {
        Self {
            builder: self.builder.child(child.build()),
        }
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl ElementBuilder<Target = Ui5SideNavigationItem>>,
    ) -> Self {
        for c in children {
            self = self.child(c);
        }

        self
    }
}

html_element!(
    ui5-side-navigation-item<web_sys::HtmlElement> {
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
            builder: self.builder.child(child.into()),
        }
    }
}

html_element!(
    ui5-side-navigation-sub-item<web_sys::HtmlElement> {
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
    event: web_sys::CustomEvent,
}

impl SelectionChanged {
    pub fn event(&self) -> &web_sys::CustomEvent {
        &self.event
    }

    pub fn item(&self) -> web_sys::HtmlElement {
        self.event
            .detail()
            .unchecked_into::<Item>()
            .item()
            .dyn_into()
            .unwrap_throw()
    }
}

impl From<web_sys::CustomEvent> for SelectionChanged {
    fn from(event: web_sys::CustomEvent) -> Self {
        Self { event }
    }
}

#[wasm_bindgen]
extern "C" {
    type Item;

    #[wasm_bindgen(structural, method, getter)]
    fn item(this: &Item) -> JsValue;
}
