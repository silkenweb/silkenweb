use std::pin::Pin;

use futures_signals::signal::{always, Signal, SignalExt};
use silkenweb_base::clone;
use silkenweb_signals_ext::{
    value::{Sig, SignalOrValue},
    SignalProduct,
};

use crate::{
    dom::{private::DomElement, Dom},
    node::element::{Element, GenericElement},
};

// TODO: Doc and examples
#[derive(Default)]
pub struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rule(mut self, rule: StyleRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn into_string_signal(self) -> Pin<Box<dyn Signal<Item = String>>> {
        let mut result = always(String::new()).boxed_local();

        for rule in self.rules {
            result = (result, rule.into_string_signal())
                .signal_ref(move |result, rule| format!("{result}{rule}"))
                .boxed_local();
        }

        result
    }

    pub fn text(self) -> Sig<Pin<Box<dyn Signal<Item = String>>>> {
        Sig(self.into_string_signal())
    }
}

pub struct StyleRule {
    selector: String,
    properties: StyleDeclaration,
}

impl StyleRule {
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
            properties: StyleDeclaration::new(),
        }
    }

    pub fn style(
        mut self,
        name: impl Into<String>,
        value: impl SignalOrValue<Item = impl Into<String>>,
    ) -> Self {
        self.properties = self.properties.style(name, value);
        self
    }

    pub fn into_string_signal(self) -> impl Signal<Item = String> {
        self.properties
            .into_string_signal()
            .map(move |props| format!("{} {{\n{props}}}\n", self.selector))
    }
}

#[derive(Default)]
pub struct StyleDeclaration {
    property_map: Vec<(String, PropValue)>,
}

type PropValue = Pin<Box<dyn Signal<Item = String>>>;

impl StyleDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: `important!` styles
    pub fn style(
        mut self,
        name: impl Into<String>,
        value: impl SignalOrValue<Item = impl Into<String>>,
    ) -> Self {
        let value_sig = value.select(
            |(), val| always(val.into()).boxed_local(),
            |(), val| val.map(|x| x.into()).boxed_local(),
            (),
        );
        self.property_map.push((name.into(), value_sig));
        self
    }

    pub fn into_string_signal(self) -> impl Signal<Item = String> {
        let mut result = always(String::new()).boxed_local();

        for (key, value) in self.property_map {
            result = (result, value)
                .signal_ref(move |result, value| format!("{result}  {key}: {value};\n"))
                .boxed_local();
        }

        result
    }

    pub fn into_attr(self) -> Sig<Pin<Box<dyn Signal<Item = String>>>> {
        Sig(self.into_string_signal().boxed_local())
    }

    pub(crate) fn onto_element<D>(self, mut dest_elem: GenericElement<D>) -> GenericElement<D>
    where
        D: Dom,
    {
        let dom_elem = dest_elem.element().clone();

        for (name, value) in self.property_map {
            clone!(mut dom_elem);
            let future = value.for_each(move |value| {
                dom_elem.style_property(&name, &value);
                async {}
            });

            dest_elem = dest_elem.spawn_future(future);
        }

        dest_elem
    }
}

#[crate::cfg_browser(false)]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stylesheet() {
        let sheet = StyleSheet::new().rule(
            StyleRule::new(":root")
                .style("--test0", "value0")
                .style("--test1", "value1"),
        );

        sheet
            .into_string_signal()
            .for_each(|value| async move {
                assert_eq!(
                    value,
                    r#":root {
  --test0: value0;
  --test1: value1;
}
"#
                );
            })
            .await;
    }
}
