//! Builders for HTML elements.
//!
//! Each HTML element has a function, a struct and a builder struct. The
//! function is a constructor for the builder. The builder has methods for each
//! attribute for that element, as well as methods for each event. For example:
//!
//! ```no_run
//! # use silkenweb::elements::{html::{a, A, ABuilder}, ElementEvents};
//! let link: ABuilder = a()
//!     .href("https://example.com/")
//!     .on_click(|event: web_sys::MouseEvent, link: web_sys::HtmlAnchorElement| {});
//! ```
//!
//! The builder type implements:
//!
//! - [`ElementBuilder`]
//! - [`HtmlElement`]
//! - [`HtmlElementEvents`]
//! - [`ElementEvents`]
//! - [`ParentBuilder`] (if it is a parent element)
//! - [`ShadowRootParentBuilder`] (if it's allowed to have a shadow root
//!   attached)
//!
//! [`ParentBuilder`]: crate::node::element::ParentBuilder
//! [`ShadowRootParentBuilder`]: crate::node::element::ShadowRootParentBuilder

use std::marker::PhantomData;

use futures_signals::signal::{Signal, SignalExt};
use paste::paste;
use silkenweb_base::intern_str;
use wasm_bindgen::JsCast;

use crate::node::element::ElementBuilder;

pub mod html;
pub mod svg;

/// Wrap a [`web_sys::CustomEvent`] and cast detail.
///
/// This is used when defining custom HTML elements to represent web components.
/// See [`html_element`] for more details.
#[derive(Clone)]
pub struct CustomEvent<T>(web_sys::CustomEvent, PhantomData<T>);

impl<T: JsCast> CustomEvent<T> {
    /// The original event.
    pub fn event(&self) -> &web_sys::CustomEvent {
        &self.0
    }

    /// The event detail, downcast into `T`.
    ///
    /// The downcast is unchecked.
    pub fn detail(&self) -> T {
        self.0.detail().unchecked_into()
    }
}

impl<T> From<web_sys::CustomEvent> for CustomEvent<T> {
    fn from(src: web_sys::CustomEvent) -> Self {
        Self(src, PhantomData)
    }
}

macro_rules! global_attributes {
    ($(
        $(#[$attr_meta:meta])*
        $attr:ident: $typ:ty
    ),* $(,)? ) => { paste!{
        $crate::attributes![
            $($(#[$attr_meta])*
            #[doc = ""]
            #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-" $attr ")"]
            $attr ($crate::text_name!($attr)): $typ,)*
        ];
    }};
}

fn class_attribute_text<T: AsRef<str>>(classes: impl IntoIterator<Item = T>) -> Option<String> {
    let mut classes = classes.into_iter();

    if let Some(first) = classes.next() {
        let mut text = first.as_ref().to_owned();

        for class in classes {
            let class = class.as_ref();
            text.reserve(1 + class.len());
            text.push(' ');
            text.push_str(class);
        }

        Some(text)
    } else {
        None
    }
}

/// An HTML element
///
/// Methods for setting attributes specific to HTML elements
pub trait HtmlElement: ElementBuilder {
    fn class(self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.attribute(intern_str("class"), class_attribute_text(value))
    }

    fn class_signal<Iter: IntoIterator<Item = impl AsRef<str>>>(
        self,
        value: impl Signal<Item = Iter> + 'static,
    ) -> Self {
        self.attribute_signal(
            intern_str("class"),
            value.map(move |class| class_attribute_text(class)),
        )
    }

    global_attributes![
        /// Provides a hint for generating a keyboard shortcut for the current
        /// element. This attribute consists of a space-separated list of
        /// characters. The browser should use the first one that exists on the
        /// computer keyboard layout.
        accesskey: String,
        /// Controls whether and how text input is automatically capitalized as
        /// it is entered/edited by the user. It can have the following values:
        ///
        /// - off or none, no autocapitalization is applied (all letters default
        ///   to lowercase)
        /// - on or sentences, the first letter of each sentence defaults to a
        ///   capital letter; all other letters default to lowercase
        /// - words, the first letter of each word defaults to a capital letter;
        ///   all other letters default to lowercase
        /// - characters, all letters should default to uppercase
        autocapitalize: String,
        /// Indicates that an element is to be focused on page load, or as soon
        /// as the `<dialog>` it is part of is displayed. This attribute is a
        /// boolean, initially false.
        autofocus: bool,
        /// An enumerated attribute indicating if the element should be
        /// editable by the user. If so, the browser modifies its widget to
        /// allow editing. The attribute must take one of the following values:
        ///
        /// - true or the empty string, which indicates that the element must be
        ///   editable;
        /// - false, which indicates that the element must not be editable.
        contenteditable: String,
        /// The id of a `<menu>` to use as the contextual menu for this element.
        contextmenu: String,
        /// An enumerated attribute indicating the directionality of the
        /// element's text. It can have the following values:
        ///
        /// - ltr, which means left to right and is to be used for languages
        ///   that are written from the left to the right (like English);
        /// - rtl, which means right to left and is to be used for languages
        ///   that are written from the right to the left (like Arabic);
        /// - auto, which lets the user agent decide. It uses a basic algorithm
        ///   as it parses the characters inside the element until it finds a
        ///   character with a strong directionality, then it applies that
        ///   directionality to the whole element.
        dir: String,
        /// An enumerated attribute indicating whether the element can be
        /// dragged, using the Drag and Drop API. It can have the following
        /// values:
        ///
        /// - true, which indicates that the element may be dragged
        /// - false, which indicates that the element may not be dragged.
        draggable: String,
        /// Hints what action label (or icon) to present for the enter key on
        /// virtual keyboards.
        enterkeyhint: String,
        /// Used to transitively export shadow parts from a nested shadow tree
        /// into a containing light tree.
        exportparts: String,
        /// A Boolean attribute indicates that the element is not yet, or is no
        /// longer, relevant. For example, it can be used to hide elements of
        /// the page that can't be used until the login process has been
        /// completed. The browser won't render such elements. This attribute
        /// must not be used to hide content that could legitimately be shown.
        hidden: bool,
        /// Defines a unique identifier (ID) which must be unique in the whole
        /// document. Its purpose is to identify the element when linking (using
        /// a fragment identifier), scripting, or styling (with CSS).
        id: String,
        /// Provides a hint to browsers as to the type of virtual keyboard
        /// configuration to use when editing this element or its contents. Used
        /// primarily on `<input>` elements, but is usable on any element while
        /// in contenteditable mode.
        inputmode: String,
        /// Allows you to specify that a standard HTML element should behave
        /// like a registered custom built-in element (see Using custom elements
        /// for more details).
        is: String,
        /// The unique, global identifier of an item.
        itemid: String,
        /// Used to add properties to an item. Every HTML element may have an
        /// itemprop attribute specified, where an itemprop consists of a name
        /// and value pair.
        itemprop: String,
        /// Properties that are not descendants of an element with the itemscope
        /// attribute can be associated with the item using an itemref. It
        /// provides a list of element ids (not itemids) with additional
        /// properties elsewhere in the document.
        itemref: String,
        /// itemscope (usually) works along with itemtype to specify that the
        /// HTML contained in a block is about a particular item. itemscope
        /// creates the Item and defines the scope of the itemtype associated
        /// with it. itemtype is a valid URL of a vocabulary (such as
        /// schema.org) that describes the item and its properties context.
        itemscope: String,
        /// Specifies the URL of the vocabulary that will be used to define
        /// itemprops (item properties) in the data structure. itemscope is used
        /// to set the scope of where in the data structure the vocabulary set
        /// by itemtype will be active.
        itemtype: String,
        /// Helps define the language of an element: the language that
        /// non-editable elements are in, or the language that editable elements
        /// should be written in by the user. The attribute contains one
        /// “language tag” (made of hyphen-separated “language subtags”) in the
        /// format defined in RFC 5646: Tags for Identifying Languages (also
        /// known as BCP 47). xml:lang has priority over it.
        lang: String,
        /// A cryptographic nonce ("number used once") which can be used by
        /// Content Security Policy to determine whether or not a given fetch
        /// will be allowed to proceed.
        nonce: String,
        /// A space-separated list of the part names of the element. Part names
        /// allows CSS to select and style specific elements in a shadow tree
        /// via the ::part pseudo-element.
        part: String,
        /// Assigns a slot in a shadow DOM shadow tree to an element: An element
        /// with a slot attribute is assigned to the slot created by the
        /// `<slot>` element whose name attribute's value matches that
        /// slot attribute's value.
        slot: String,
        /// An enumerated attribute defines whether the element may be checked
        /// for spelling errors. It may have the following values:
        ///
        /// - true, which indicates that the element should be, if possible,
        ///   checked for spelling errors;
        /// - false, which indicates that the element should not be checked for
        ///   spelling errors.
        spellcheck: String,
        /// Contains CSS styling declarations to be applied to the element. Note
        /// that it is recommended for styles to be defined in a separate file
        /// or files. This attribute and the `<style>` element have mainly the
        /// purpose of allowing for quick styling, for example for testing
        /// purposes.
        style: String,
        /// An integer attribute indicating if the element can take input focus
        /// (is focusable), if it should participate to sequential keyboard
        /// navigation, and if so, at what position. It can take several values:
        ///
        /// - a negative value means that the element should be focusable, but
        ///   should not be reachable via sequential keyboard navigation;
        /// - 0 means that the element should be focusable and reachable via
        ///   sequential keyboard navigation, but its relative order is defined
        ///   by the platform convention;
        /// - a positive value means that the element should be focusable and
        ///   reachable via sequential keyboard navigation; the order in which
        ///   the elements are focused is the increasing value of the tabindex.
        ///   If several elements share the same tabindex, their relative order
        ///   follows their relative positions in the document.
        tabindex: i32,
        /// Contains a text representing advisory information related to the
        /// element it belongs to. Such information can typically, but not
        /// necessarily, be presented to the user as a tooltip.
        title: String,
        /// An enumerated attribute that is used to specify whether an element's
        /// attribute values and the values of its Text node children are to be
        /// translated when the page is localized, or whether to leave them
        /// unchanged. It can have the following values:
        ///
        /// - empty string and yes, which indicates that the element will be
        ///   translated.
        /// - no, which indicates that the element will not be translated.
        translate: String,
    ];
}

/// Events common to all HTML elements
pub trait HtmlElementEvents: ElementBuilder {
    events!(Self::DomType {
        animationend: web_sys::AnimationEvent,
        animationiteration: web_sys::AnimationEvent,
        animationstart: web_sys::AnimationEvent,
        beforeinput: web_sys::InputEvent,
        change: web_sys::Event,
        gotpointercapture: web_sys::PointerEvent,
        input: web_sys::InputEvent,
        lostpointercapture: web_sys::PointerEvent,
        pointercancel: web_sys::PointerEvent,
        pointerdown: web_sys::PointerEvent,
        pointerenter: web_sys::PointerEvent,
        pointerleave: web_sys::PointerEvent,
        pointermove: web_sys::PointerEvent,
        pointerout: web_sys::PointerEvent,
        pointerover: web_sys::PointerEvent,
        pointerup: web_sys::PointerEvent,
        transitionend: web_sys::TransitionEvent,
    });
}

/// Events common to all elements
pub trait ElementEvents: ElementBuilder {
    events!(Self::DomType {
        auxclick: web_sys::MouseEvent,
        blur: web_sys::FocusEvent,
        click: web_sys::MouseEvent,
        compositionend: web_sys::CompositionEvent,
        compositionstart: web_sys::CompositionEvent,
        compositionupdate: web_sys::CompositionEvent,
        contextmenu: web_sys::MouseEvent,
        dblclick: web_sys::MouseEvent,
        error: web_sys::Event,
        focusin: web_sys::FocusEvent,
        focusout: web_sys::FocusEvent,
        focus: web_sys::FocusEvent,
        fullscreenchange: web_sys::Event,
        fullscreenerror: web_sys::Event,
        keydown: web_sys::KeyboardEvent,
        keyup: web_sys::KeyboardEvent,
        mousedown: web_sys::MouseEvent,
        mouseenter: web_sys::MouseEvent,
        mouseleave: web_sys::MouseEvent,
        mousemove: web_sys::MouseEvent,
        mouseout: web_sys::MouseEvent,
        mouseover: web_sys::MouseEvent,
        mouseup: web_sys::MouseEvent,
        scroll: web_sys::Event,
        select: web_sys::Event,
        touchcancel: web_sys::TouchEvent,
        touchend: web_sys::TouchEvent,
        touchmove: web_sys::TouchEvent,
        touchstart: web_sys::TouchEvent,
        wheel: web_sys::WheelEvent,
        // The events are currently marked as unstable in web_sys:
        //
        // copy: web_sys::ClipboardEvent,
        // cut: web_sys::ClipboardEvent,
        // paste: web_sys::ClipboardEvent,
    });
}
