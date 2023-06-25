//! HTML element types.
//!
//! Each HTML element has an associated struct. For example, the `<a>` element
//! is represented by [`html::A`]. The constructor is a free function of the
//! same name, so you can write `a()` rather than `A::new()`. There are methods
//! for each attribute and event. Event methods are prefixed with `on_`. For
//! example:
//!
//! ```no_run
//! # use silkenweb::prelude::*;
//! # use html::{a, A};
//! let link: A = a()
//!     .href("https://example.com/")
//!     .on_click(|event, element| {});
//! ```
//!
//! The element type implements various traits, including:
//!
//! - [`Element`]
//! - [`HtmlElement`]
//! - [`HtmlElementEvents`]
//! - [`ElementEvents`]
//! - [`ParentElement`] (if it is a parent element)
//! - [`ShadowRootParent`] (if it's allowed to have a shadow root attached)
//!
//! Each element can be frozen, making it immutable, using the `freeze` method.
//!
//! [`ParentElement`]: crate::node::element::ParentElement
//! [`ShadowRootParent`]: crate::node::element::ShadowRootParent

use std::marker::PhantomData;

use wasm_bindgen::JsCast;

use crate::node::element::Element;

pub mod html;
pub mod svg;

/// Wrap a [`web_sys::CustomEvent`].
///
/// This is used when defining custom HTML elements to represent web components.
/// See [`custom_html_element`] for more details.
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
    ($($t:tt)*) => {
        attributes![
            [
                attribute_parent = (),
                attribute_doc_macro = global_attribute_doc
            ]

            $($t)*
        ];
    };
}

macro_rules! global_attribute_doc {
    ($element:expr, $name:expr) => {
        concat!(
            "The Global [",
            $name,
            "](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-",
            $name,
            ") Attribute"
        )
    };
}

/// An HTML element
///
/// Methods for setting attributes specific to HTML elements
pub trait HtmlElement: Element {
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
pub trait HtmlElementEvents: Element {
    events!(Self::DomType {
        beforeinput: web_sys::InputEvent,
        change: web_sys::Event,
        error: web_sys::Event,
        input: web_sys::InputEvent,
        drag: web_sys::DragEvent,
        dragend: web_sys::DragEvent,
        dragenter: web_sys::DragEvent,
        dragleave: web_sys::DragEvent,
        dragover: web_sys::DragEvent,
        dragstart: web_sys::DragEvent,
        drop: web_sys::DragEvent,
        load: web_sys::Event,
    });
}

/// Events common to all elements
pub trait ElementEvents: Element {
    events!(Self::DomType {
        animationcancel: web_sys::AnimationEvent,
        animationend: web_sys::AnimationEvent,
        animationiteration: web_sys::AnimationEvent,
        animationstart: web_sys::AnimationEvent,
        auxclick: web_sys::MouseEvent,
        blur: web_sys::FocusEvent,
        click: web_sys::MouseEvent,
        compositionend: web_sys::CompositionEvent,
        compositionstart: web_sys::CompositionEvent,
        compositionupdate: web_sys::CompositionEvent,
        contextmenu: web_sys::MouseEvent,
        dblclick: web_sys::MouseEvent,
        focusin: web_sys::FocusEvent,
        focusout: web_sys::FocusEvent,
        focus: web_sys::FocusEvent,
        fullscreenchange: web_sys::Event,
        fullscreenerror: web_sys::Event,
        gotpointercapture: web_sys::PointerEvent,
        keydown: web_sys::KeyboardEvent,
        keyup: web_sys::KeyboardEvent,
        lostpointercapture: web_sys::PointerEvent,
        mousedown: web_sys::MouseEvent,
        mouseenter: web_sys::MouseEvent,
        mouseleave: web_sys::MouseEvent,
        mousemove: web_sys::MouseEvent,
        mouseout: web_sys::MouseEvent,
        mouseover: web_sys::MouseEvent,
        mouseup: web_sys::MouseEvent,
        pointercancel: web_sys::PointerEvent,
        pointerdown: web_sys::PointerEvent,
        pointerenter: web_sys::PointerEvent,
        pointerleave: web_sys::PointerEvent,
        pointermove: web_sys::PointerEvent,
        pointerout: web_sys::PointerEvent,
        pointerover: web_sys::PointerEvent,
        pointerrawupdate: web_sys::PointerEvent,
        pointerup: web_sys::PointerEvent,
        transitioncancel: web_sys::TransitionEvent,
        transitionend: web_sys::TransitionEvent,
        transitionrun: web_sys::TransitionEvent,
        transitionstart: web_sys::TransitionEvent,
        scroll: web_sys::Event,
        scrollend: web_sys::Event,
        securitypolicyviolation: web_sys::SecurityPolicyViolationEvent,
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

macro_rules! aria_attributes {
    ($($t:tt)*) => {
        attributes![
            [
                attribute_parent = (),
                attribute_doc_macro = aria_attribute_doc
            ]

            $($t)*
        ];
    };
}

macro_rules! aria_attribute_doc {
    ($element:expr, $name:expr) => {
        concat!(
            "The ARIA [",
            $name,
            "](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/",
            $name,
            ") Attribute"
        )
    };
}

/// [ARIA] attributes.
///
/// [ARIA]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA
pub trait AriaElement: Element {
    attributes![
        /// The ARIA [role](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles)
        /// attribute
        role: String,
    ];
    aria_attributes![
        /// The aria-activedescendant attribute identifies the currently active
        /// element when focus is on a composite widget, combobox, textbox,
        /// group, or application.
        aria_activedescendant: String,
        /// In ARIA live regions, the global aria-atomic attribute indicates
        /// whether assistive technologies such as a screen reader will present
        /// all, or only parts of, the changed region based on the change
        /// notifications defined by the aria-relevant attribute.
        aria_atomic: String,
        /// The aria-autocomplete attribute indicates whether inputting text
        /// could trigger display of one or more predictions of the user's
        /// intended value for a combobox, searchbox, or textbox and specifies
        /// how predictions will be presented if they are made.
        aria_autocomplete: String,
        /// The global aria-braillelabel property defines a string value that
        /// labels the current element, which is intended to be converted into
        /// Braille.
        aria_braillelabel: String,
        /// The global aria-brailleroledescription attribute defines a
        /// human-readable, author-localized abbreviated description for the
        /// role of an element intended to be converted into Braille.
        aria_brailleroledescription: String,
        /// Used in ARIA live regions, the global aria-busy state indicates an
        /// element is being modified and that assistive technologies may want
        /// to wait until the changes are complete before informing the user
        /// about the update.
        aria_busy: String,
        /// The aria-checked attribute indicates the current "checked" state of
        /// checkboxes, radio buttons, and other widgets.
        aria_checked: String,
        /// The aria-colcount attribute defines the total number of columns in a
        /// table, grid, or treegrid when not all columns are present in the
        /// DOM.
        aria_colcount: i64,
        /// The aria-colindex attribute defines an element's column index or
        /// position with respect to the total number of columns within a table,
        /// grid, or treegrid.
        aria_colindex: u64,
        /// The aria-colindextext attribute defines a human readable text
        /// alternative of the numeric aria-colindex.
        aria_colindextext: String,
        /// The aria-colspan attribute defines the number of columns spanned by
        /// a cell or gridcell within a table, grid, or treegrid.
        aria_colspan: u64,
        /// The global aria-controls property identifies the element (or
        /// elements) whose contents or presence are controlled by the element
        /// on which this attribute is set.
        aria_controls: String,
        /// A non-null aria-current state on an element indicates that this
        /// element represents the current item within a container or set of
        /// related elements.
        aria_current: String,
        /// The global aria-describedby attribute identifies the element (or
        /// elements) that describes the element on which the attribute is set.
        aria_describedby: String,
        /// The global aria-description attribute defines a string value that
        /// describes or annotates the current element.
        aria_description: String,
        /// The global aria-details attribute identifies the element (or
        /// elements) that provide additional information related to the object.
        aria_details: String,
        /// The aria-disabled state indicates that the element is perceivable
        /// but disabled, so it is not editable or otherwise operable.
        aria_disabled: String,
        /// The aria-errormessage attribute on an object identifies the element
        /// that provides an error message for that object.
        aria_errormessage: String,
        /// The aria-expanded attribute is set on an element to indicate if a
        /// control is expanded or collapsed, and whether or not its child
        /// elements are displayed or hidden.
        aria_expanded: String,
        /// The global aria-flowto attribute identifies the next element (or
        /// elements) in an alternate reading order of content. This allows
        /// assistive technology to override the general default of reading in
        /// document source order at the user's discretion.
        aria_flowto: String,
        /// The aria-haspopup attribute indicates the availability and type of
        /// interactive popup element that can be triggered by the element on
        /// which the attribute is set.
        aria_haspopup: String,
        /// The aria-hidden state indicates whether the element is exposed to an
        /// accessibility API.
        aria_hidden: String,
        /// The aria-invalid state indicates the entered value does not conform
        /// to the format expected by the application.
        aria_invalid: String,
        /// The global aria-keyshortcuts attribute indicates keyboard shortcuts
        /// that an author has implemented to activate or give focus to an
        /// element.
        aria_keyshortcuts: String,
        /// The aria-label attribute defines a string value that labels an
        /// interactive element.
        aria_label: String,
        /// The aria-labelledby attribute identifies the element (or elements)
        /// that labels the element it is applied to.
        aria_labelledby: String,
        /// The aria-level attribute defines the hierarchical level of an
        /// element within a structure.
        aria_level: u64,
        /// The global aria-live attribute indicates that an element will be
        /// updated, and describes the types of updates the user agents,
        /// assistive technologies, and user can expect from the live region.
        aria_live: String,
        /// The aria-modal attribute indicates whether an element is modal when
        /// displayed.
        aria_modal: String,
        /// The aria-multiline attribute indicates whether a textbox accepts
        /// multiple lines of input or only a single line.
        aria_multiline: String,
        /// The aria-multiselectable attribute indicates that the user may
        /// select more than one item from the current selectable descendants.
        aria_multiselectable: String,
        /// The aria-orientation attribute indicates whether the element's
        /// orientation is horizontal, vertical, or unknown/ambiguous.
        aria_orientation: String,
        /// The aria-owns attribute identifies an element (or elements) in order
        /// to define a visual, functional, or contextual relationship between a
        /// parent and its child elements when the DOM hierarchy cannot be used
        /// to represent the relationship.
        aria_owns: String,
        /// The aria-placeholder attribute defines a short hint (a word or short
        /// phrase) intended to help the user with data entry when a form
        /// control has no value. The hint can be a sample value or a brief
        /// description of the expected format.
        aria_placeholder: String,
        /// The aria-posinset attribute defines an element's number or position
        /// in the current set of listitems or treeitems when not all items are
        /// present in the DOM.
        aria_posinset: u64,
        /// The aria-pressed attribute indicates the current "pressed" state of
        /// a toggle button.
        aria_pressed: String,
        /// The aria-readonly attribute indicates that the element is not
        /// editable, but is otherwise operable.
        aria_readonly: String,
        /// Used in ARIA live regions, the global aria-relevant attribute
        /// indicates what notifications the user agent will trigger when the
        /// accessibility tree within a live region is modified.
        aria_relevant: String,
        /// The aria-required attribute indicates that user input is required on
        /// the element before a form may be submitted.
        aria_required: String,
        /// The aria-roledescription attribute defines a human-readable,
        /// author-localized description for the role of an element.
        aria_roledescription: String,
        /// The aria-rowcount attribute defines the total number of rows in a
        /// table, grid, or treegrid.
        aria_rowcount: i64,
        /// The aria-rowindex attribute defines an element's position with
        /// respect to the total number of rows within a table, grid, or
        /// treegrid.
        aria_rowindex: u64,
        /// The aria-rowindextext attribute defines a human-readable text
        /// alternative of aria-rowindex.
        aria_rowindextext: String,
        /// The aria-rowspan attribute defines the number of rows spanned by a
        /// cell or gridcell within a table, grid, or treegrid.
        aria_rowspan: u64,
        /// The aria-selected attribute indicates the current "selected" state
        /// of various widgets.
        aria_selected: String,
        /// The aria-setsize attribute defines the number of items in the
        /// current set of listitems or treeitems when not all items in the set
        /// are present in the DOM.
        aria_setsize: i64,
        /// The aria-sort attribute indicates if items in a table or grid are
        /// sorted in ascending or descending order.
        aria_sort: String,
        /// The aria-valuemax attribute defines the maximum allowed value for a
        /// range widget.
        aria_valuemax: f64,
        /// The aria-valuemin attribute defines the minimum allowed value for a
        /// range widget.
        aria_valuemin: f64,
        /// The aria-valuenow attribute defines the current value for a range
        /// widget.
        aria_valuenow: f64,
        /// The aria-valuetext attribute defines the human readable text
        /// alternative of aria-valuenow for a range widget.
        aria_valuetext: String,
    ];
}
