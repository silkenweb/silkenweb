use std::{
    cell::{RefCell, RefMut},
    fmt::{self, Display},
    rc::Rc,
};

use wasm_bindgen::JsValue;

use self::{
    dry::{DryElement, DryText},
    event::EventCallback,
    wet::{WetElement, WetText},
};
use super::{
    lazy::{IsDry, Lazy},
    HydrationStats,
};
use crate::{attribute::Attribute, node::private::ElementImpl};

mod event;

pub(super) mod dry;
pub(super) mod wet;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Namespace {
    /// New elements in the `Html` namespace are created with `create_element`,
    /// thus avoiding converting the namespace to a javascript string.
    Html,
    Other(Option<&'static str>),
}

impl Namespace {
    fn as_str(&self) -> &str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::Other(None) => "",
            Namespace::Other(Some(ns)) => ns,
        }
    }
}
