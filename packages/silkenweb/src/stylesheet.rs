use std::{collections::HashMap, pin::Pin};

use futures_signals::signal::{always, Signal, SignalExt};
use silkenweb_signals_ext::{value::SignalOrValue, SignalProduct};

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

    pub fn into_string_signal(self) -> impl Signal<Item = String> {
        let mut result = always(String::new()).boxed_local();

        for rule in self.rules {
            result = (result, rule.into_string_signal())
                .signal_ref(move |result, rule| format!("{result}{rule}\n"))
                .boxed_local();
        }

        result
    }
}

pub struct StyleRule {
    selector: String,
    properties: StyleDeclaration,
}

impl StyleRule {
    pub fn new(selector: String) -> Self {
        Self {
            selector,
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
    property_map: HashMap<String, Pin<Box<dyn Signal<Item = String>>>>,
}

impl StyleDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

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
        self.property_map.insert(name.into(), value_sig);
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
}
