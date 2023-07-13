//! An abstraction for the underlying DOM.
//!
//! An application, or parts of an application, can be parameterized on the
//! underlying DOM type so it can be rendered in different ways:
//!
//! - Server side rendering with [`Dry`]
//! - Client side rendering with [`Wet`]
//! - Hydrating a server side rendered initial application state with [`Hydro`]
//! - As a template, or part of a template, with [`Template`]
//!
//! See the concrete DOM types for some examples.
use std::marker::PhantomData;

use silkenweb_macros::cfg_browser;

use self::{
    dry::{DryElement, DryNode, DryText},
    hydro::{HydroElement, HydroNode, HydroText},
    template::{TemplateElement, TemplateNode, TemplateText},
    wet::{WetElement, WetNode, WetText},
};
use crate::{ServerSend, ServerSync};

pub(super) mod private;

mod dry;
mod hydro;
mod template;
mod wet;

/// The main DOM abstraction.
///
/// This is not user implementable.
pub trait Dom: private::Dom {}

/// A DOM that can be instantiated from a [`Template`] DOM.
pub trait InstantiableDom: Dom + private::InstantiableDom {}

/// The [`Dom`] type to which a node belongs.
pub trait InDom {
    type Dom: Dom;
}

#[cfg_browser(true)]
/// The default DOM for the current platform.
pub type DefaultDom = Wet;

#[cfg_browser(false)]
/// The default DOM for the current platform.
pub type DefaultDom = Dry;

/// A DOM that can only be rendered on the server
///
/// # Example
///
/// Type annotations have been provided for clarity, but the types can be
/// inferred.
///
/// ```
/// # use silkenweb::{dom::Dry, elements::html::{p, P}, prelude::*};
/// let app: P<Dry> = p().text("Hello, world!").into();
///
/// assert_eq!(app.freeze().to_string(), "<p>Hello, world!</p>");
/// ```
pub struct Dry;

impl Dom for Dry {}

impl private::Dom for Dry {
    type Element = DryElement;
    type Node = DryNode;
    type Text = DryText;
}

impl InstantiableDom for Dry {}

impl private::InstantiableDom for Dry {
    type InstantiableElement = DryElement;
    type InstantiableNode = DryNode;
}

/// A DOM that can be rendered on the client or hydrated onto an existing DOM
/// element.
///
/// # Example
///
/// Type annotations have been provided for clarity, but the types can be
/// inferred.
///
/// ```no_run
/// # use html::{p, P};
/// # use silkenweb::{dom::Hydro, hydration::hydrate, prelude::*};
/// let app: P<Hydro> = p().text("Hello, world!");
///
/// hydrate("app-id", app);
/// ```
pub struct Hydro;

impl Dom for Hydro {}

impl private::Dom for Hydro {
    type Element = HydroElement;
    type Node = HydroNode;
    type Text = HydroText;
}

impl InstantiableDom for Hydro {}

impl private::InstantiableDom for Hydro {
    type InstantiableElement = HydroElement;
    type InstantiableNode = HydroNode;
}

/// A DOM that can only be rendered on the client.
///
/// # Example
///
/// Type annotations have been provided for clarity, but the types can be
/// inferred.
///
/// ```no_run
/// # use html::{p, P};
/// # use silkenweb::{dom::Wet, document::Document, prelude::*};
/// let app: P<Wet> = p().text("Hello, world!");
///
/// Wet::mount("app-id", app);
/// ```
pub struct Wet;

impl Dom for Wet {}

impl private::Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

impl InstantiableDom for Wet {}

impl private::InstantiableDom for Wet {
    type InstantiableElement = WetElement;
    type InstantiableNode = WetNode;
}

/// A template DOM that can be used to instantiate other DOM types by cloning.
///
/// Cloning a template can be faster than creating each DOM node individually.
/// It's likely to get a maximum 10-20% increase, so should only be used in hot
/// code paths.
///
/// # Example
///
/// Type annotations have been provided for clarity, but the types can be
/// inferred.
///
/// ```
/// # use html::{p, P};
/// # use silkenweb::{
/// #     dom::{Dry, Template},
/// #     node::element::Const,
/// #     prelude::*,
/// # };
/// let elem: P<Template<String, Dry>> = p().on_instantiate(|p, message| p.text(message));
/// let template: P<Template<String, Dry>, Const> = elem.freeze();
/// let hello = template.instantiate(&"Hello, world!".to_string());
/// let goodbye = template.instantiate(&"Goodbye!".to_string());
///
/// assert_eq!(hello.freeze().to_string(), "<p>Hello, world!</p>");
/// assert_eq!(goodbye.freeze().to_string(), "<p>Goodbye!</p>");
/// ```
pub struct Template<Param, D: InstantiableDom = DefaultDom>(PhantomData<(Param, D)>);

impl<Param: 'static, D: InstantiableDom> Dom for Template<Param, D> {}

impl<Param: 'static, D: InstantiableDom> private::Dom for Template<Param, D> {
    type Element = TemplateElement<Param, D>;
    type Node = TemplateNode<Param, D>;
    type Text = TemplateText<D>;
}

// TODO: Doc
pub trait InitializeElemFn<Param, Elem>:
    Fn(Elem, &Param) -> Elem + ServerSend + ServerSync
{
}

impl<T, Param, Elem> InitializeElemFn<Param, Elem> for T where
    T: Fn(Elem, &Param) -> Elem + ServerSend + ServerSync
{
}
