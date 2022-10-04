//! SVG Elements

use self::{
    attributes::{
        AnimationTiming, AnimationValue, ConditionalProcessing, OtherAnimation, Presentation,
    },
    content_type::{AutoOrLength, Length},
};

pub mod attributes;
pub mod content_type;
pub mod path;

svg_element!(
    /// The <a> SVG element creates a hyperlink to other web pages, files,
    /// locations in the same page, email addresses, or any other URL. It is
    /// very similar to HTML's <a> element.
    ///
    /// SVG's <a> element is a container, which means you can create a link
    /// around text (like in HTML) but also around any shape.
    a = {
        dom_type: web_sys::SvgaElement;
        attributes {
            /// Instructs browsers to download a URL instead of navigating to
            /// it, so the user will be prompted to save it as a local file.
            /// Value type: <string> ; Default value: none; Animatable: no
            download: String,

            /// The URL or URL fragment the hyperlink points to.
            /// Value type: <URL> ;
            /// Default value: none; Animatable: yes
            href: String,

            /// The human language of the URL or URL fragment that the hyperlink
            /// points to.
            /// Value type: <string> ;
            /// Default value: none; Animatable: yes
            hreflang: String,

            /// A space-separated list of URLs to which, when the hyperlink is
            /// followed, POST requests with the body PING will be sent by the
            /// browser (in the background). Typically used for tracking. For a
            /// more widely-supported feature addressing the same use cases, see
            /// Navigator.sendBeacon().
            /// Value type: <list-of-URLs> ;
            /// Default value: none; Animatable: no
            ping: String,

            /// Which referrer to send when fetching the URL.
            /// Value type:
            /// no-referrer|no-referrer-when-downgrade|same-origin|origin|strict-origin|origin-when-cross-origin|strict-origin-when-cross-origin|unsafe-url
            /// ; Default value: none; Animatable: no
            referrerpolicy: String,

            /// The relationship of the target object to the link object.
            /// Value type: <list-of-Link-Types> ;
            /// Default value: none; Animatable: yes
            rel: String,

            /// Where to display the linked URL.
            /// Value type: _self|_parent|_top|_blank|<name> ;
            /// Default value: _self; Animatable: yes
            target: String,

            /// A MIME type for the linked URL.
            /// Value type: <string> ; Default value: none; Animatable: yes
            r#type: String,
        };
    }
);

parent_element!(a);
impl ConditionalProcessing for ABuilder {}
impl Presentation for ABuilder {}

svg_element!(
    /// The SVG <animate> element provides a way to animate an attribute of an
    /// element over time.
    animate = {
        dom_type: web_sys::SvgaElement;
    }
);

impl AnimationTiming for AnimateBuilder {}
impl AnimationValue for AnimateBuilder {}
impl OtherAnimation for AnimateBuilder {}

svg_element!(
    /// The <defs> element is used to store graphical objects that will be used
    /// at a later time. Objects created inside a <defs> element are not
    /// rendered directly. To display them you have to reference them (with a
    /// <use> element for example).
    ///
    /// Graphical objects can be referenced from anywhere, however, defining
    /// these objects inside of a <defs> element promotes understandability of
    /// the SVG content and is beneficial to the overall accessibility of the
    /// document.
    defs = {
        dom_type: web_sys::SvgDefsElement;
    }
);

parent_element!(defs);
impl ConditionalProcessing for DefsBuilder {}
impl Presentation for DefsBuilder {}

svg_element!(
    /// The <g> SVG element is a container used to group other SVG elements.
    ///
    /// Transformations applied to the <g> element are performed on its child
    /// elements, and its attributes are inherited by its children. It can also
    /// group multiple elements to be referenced later with the <use> element.
    g = {
        dom_type: web_sys::SvggElement;
    }
);

parent_element!(g);
impl ConditionalProcessing for GBuilder {}
impl Presentation for GBuilder {}

// TODO: Add all svg elements, (element, global) * (attributes, events)
svg_element!(
    /// The svg element is a container that defines a new coordinate system and
    /// viewport. It is used as the outermost element of SVG documents, but it
    /// can also be used to embed an SVG fragment inside an SVG or HTML
    /// document.
    ///
    /// Note: The xmlns attribute is only required on the outermost svg element
    /// of SVG documents. It is unnecessary for inner svg elements or inside
    /// HTML documents.
    svg  = { dom_type: web_sys::SvgsvgElement;
        attributes {
            /// The displayed height of the rectangular viewport. (Not the
            /// height of its coordinate system.)
            /// Value type: <length>|<percentage> ; Default value: auto;
            /// Animatable: yes
            height: AutoOrLength,

            /// How the svg fragment must be deformed if it is displayed with a
            /// different aspect ratio.
            /// Value type: (none| xMinYMin| xMidYMin| xMaxYMin| xMinYMid| xMidYMid| xMaxYMid| xMinYMax| xMidYMax| xMaxYMax) (meet|slice)? ;
            /// Default value: xMidYMid meet; Animatable: yes
            preserve_aspect_ratio("preserveAspectRatio"): String,

            /// The SVG viewport coordinates for the current SVG fragment.
            /// Value type: <list-of-numbers> ; Default value: none;
            /// Animatable: yes
            view_box("viewBox"): String,

            /// The displayed width of the rectangular viewport. (Not the width
            /// of its coordinate system.) Value type: <length>|<percentage> ;
            /// Default value: auto; Animatable: yes
            width: AutoOrLength,

            /// The displayed x coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: <length>|<percentage> ;
            /// Default value: 0; Animatable: yes
            x: Length,

            /// The displayed y coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: <length>|<percentage> ;
            /// Default value: 0; Animatable: yes
            y: Length,
        };
    }
);

impl ConditionalProcessing for SvgBuilder {}
impl Presentation for SvgBuilder {}

parent_element!(svg);

svg_element!(
    /// The `path` SVG element is the generic element to define a shape. All the
    /// basic shapes can be created with a path element.
    path  = { dom_type: web_sys::SvgPathElement;
        attributes {
            /// This attribute lets authors specify the total length for the
            /// path, in user units.
            /// Value type: <number> ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl ConditionalProcessing for PathBuilder {}
impl Presentation for PathBuilder {}

parent_element!(path);

svg_element!(
    /// The <rect> element is a basic SVG shape that draws rectangles, defined
    /// by their position, width, and height. The rectangles may have their
    /// corners rounded.
    rect  = { dom_type: web_sys::SvgRectElement;
        attributes {
            /// The x coordinate of the rect. Value type: <length>|<percentage> ; Default
            /// value: 0; Animatable: yes
            x: Length,

            /// The y coordinate of the rect. Value type: <length>|<percentage> ; Default
            /// value: 0; Animatable: yes
            y: Length,

            /// The width of the rect. Value type: auto|<length>|<percentage> ; Default
            /// value: auto; Animatable: yes
            width: Length,

            /// The height of the rect. Value type: auto|<length>|<percentage> ; Default
            /// value: auto; Animatable: yes
            height: Length,

            /// The horizontal corner radius of the rect. Defaults to ry if it is specified.
            /// Value type: auto|<length>|<percentage> ; Default value: auto; Animatable:
            /// yes
            rx: Length,

            /// The vertical corner radius of the rect. Defaults to rx if it is specified.
            /// Value type: auto|<length>|<percentage> ; Default value: auto; Animatable:
            /// yes
            ry: Length,

            /// The total length of the rectangle's perimeter, in user units. Value type:
            /// <number> ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl ConditionalProcessing for RectBuilder {}
impl Presentation for RectBuilder {}

parent_element!(rect);

svg_element!(
    /// The <use> element takes nodes from within the SVG document, and
    /// duplicates them somewhere else.
    r#use = {
        dom_type: web_sys::SvgUseElement;
        attributes {
            /// The URL to an element/fragment that needs to be duplicated.
            /// Value type: <URL> ; Default value: none; Animatable: yes
            href: String,
            /// The x coordinate of the use element.
            /// Value type: <coordinate> ; Default value: 0; Animatable: yes
            x: Length,
            /// The y coordinate of the use element.
            /// Value type: <coordinate> ; Default value: 0; Animatable: yes
            y: Length,
            /// The width of the use element.
            /// Value type: <length> ; Default value: 0; Animatable: yes
            width: Length,
            /// The height of the use element.
            /// Value type: <length> ; Default value: 0; Animatable: yes
            height: Length,
        };
    }
);

impl ConditionalProcessing for UseBuilder {}
impl Presentation for UseBuilder {}

parent_element!(use);
