use futures_signals::signal::{Signal, SignalExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    node::element::{ChildBuilder, Element, ElementBuilder, ParentBuilder},
    prelude::HtmlElement,
};

use self::element::{ui5_bar, Ui5Bar, Ui5BarBuilder};

#[derive(Copy, Clone, Eq, PartialEq, Display)]
pub enum BarDesign {
    Header,
    Subheader,
    Footer,
    FloatingFooter,
}

impl Attribute for BarDesign {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<BarDesign> for BarDesign {}

mod element {
    use silkenweb::{html_element, parent_element};

    use super::BarDesign;

    html_element!(
        ui5-bar<web_sys::HtmlElement> {
            attributes {
                design: BarDesign
            }
        }
    );

    parent_element!(ui5 - bar);
}

pub struct BarBuilder {
    builder: Ui5BarBuilder,
    children: ChildBuilder,
}
pub type Bar = Ui5Bar;

pub fn bar(
    start: impl ElementBuilder<Target = impl Into<Element>> + HtmlElement,
    middle: impl ElementBuilder<Target = impl Into<Element>>,
    end: impl ElementBuilder<Target = impl Into<Element>> + HtmlElement,
) -> BarBuilder {
    let children = ChildBuilder::new()
        .child(start.slot("startContent").build().into())
        .child(middle.build().into())
        .child(end.slot("endContent").build().into());

    BarBuilder {
        builder: ui5_bar(),
        children,
    }
}

pub fn bar_signal(
    start: impl Signal<Item = impl ElementBuilder<Target = impl Into<Element>> + HtmlElement> + 'static,
    middle: impl Signal<Item = impl ElementBuilder<Target = impl Into<Element>>> + 'static,
    end: impl Signal<Item = impl ElementBuilder<Target = impl Into<Element>> + HtmlElement> + 'static,
) -> BarBuilder {
    let children = ChildBuilder::new()
        .child_signal(start.map(|e| e.slot("startContent").build().into()))
        .child_signal(middle.map(|e| e.build().into()))
        .child_signal(end.map(|e| e.slot("endContent").build().into()));

    BarBuilder {
        builder: ui5_bar(),
        children,
    }
}

impl BarBuilder {
    pub fn design(self, design: BarDesign) -> Self {
        Self {
            builder: self.builder.design(design),
            children: self.children,
        }
    }

    pub fn design_signal(self, design: impl Signal<Item = BarDesign> + 'static) -> Self {
        Self {
            builder: self.builder.design_signal(design),
            children: self.children,
        }
    }
}

impl ElementBuilder for BarBuilder {
    type DomType = web_sys::HtmlElement;
    type Target = Bar;

    fn attribute<T: Attribute>(self, name: &str, value: T) -> Self {
        Self {
            builder: self.builder.attribute(name, value),
            children: self.children,
        }
    }

    fn attribute_signal<T: Attribute + 'static>(
        self,
        name: &str,
        value: impl Signal<Item = T> + 'static,
    ) -> Self {
        Self {
            builder: self.builder.attribute_signal(name, value),
            children: self.children,
        }
    }

    fn effect(self, f: impl FnOnce(&Self::DomType) + 'static) -> Self {
        Self {
            builder: self.builder.effect(f),
            children: self.children,
        }
    }

    fn effect_signal<T: 'static>(
        self,
        sig: impl Signal<Item = T> + 'static,
        f: impl Fn(&Self::DomType, T) + Clone + 'static,
    ) -> Self {
        Self {
            builder: self.builder.effect_signal(sig, f),
            children: self.children,
        }
    }

    fn handle(&self) -> silkenweb::node::element::ElementHandle<Self::DomType> {
        self.builder.handle()
    }

    fn spawn_future(self, future: impl std::future::Future<Output = ()> + 'static) -> Self {
        Self {
            builder: self.builder.spawn_future(future),
            children: self.children,
        }
    }

    fn on(self, name: &'static str, f: impl FnMut(wasm_bindgen::JsValue) + 'static) -> Self {
        Self {
            builder: self.builder.on(name, f),
            children: self.children,
        }
    }

    fn build(self) -> Self::Target {
        self.builder.child_builder(self.children)
    }
}
