//! SVG Elements

use self::{
    attributes::{ConditionalProcessing, Presentation},
    content_type::{AutoOrLength, Length},
};

pub mod attributes;
pub mod content_type;
pub mod path;

svg_element!(
    a <web_sys::SvgaElement> {
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
            /// Value type: no-referrer|no-referrer-when-downgrade|same-origin|origin|strict-origin|origin-when-cross-origin|strict-origin-when-cross-origin|unsafe-url ;
            /// Default value: none; Animatable: no
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
        }
    }
);

// TODO: Add all svg elements, (element, global) * (attributes, events)
svg_element!(
    svg <web_sys::SvgsvgElement> {
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
        }
    }
);

impl Presentation for SvgBuilder {}
impl ConditionalProcessing for SvgBuilder {}

parent_element!(svg);

svg_element!(
    path <web_sys::SvgPathElement> {
        attributes {
            /// This attribute lets authors specify the total length for the
            /// path, in user units.
            /// Value type: <number> ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        }
    }
);

impl Presentation for PathBuilder {}
impl ConditionalProcessing for PathBuilder {}

svg_element!(
    rect <web_sys::SvgPathElement> {
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
        }
    }
);

impl Presentation for RectBuilder {}
impl ConditionalProcessing for RectBuilder {}

svg_element!(
    snake(r#use),
    camel(Use, UseBuilder),
    text("use")
    <web_sys::SvgUseElement> {
        attributes {
            /// The URL to an element/fragment that needs to be duplicated.
            /// Value type: <URL> ; Default value: none; Animatable: yes
            href("href"): String,
            /// The x coordinate of the use element.
            /// Value type: <coordinate> ; Default value: 0; Animatable: yes
            x("x"): Length,
            /// The y coordinate of the use element.
            /// Value type: <coordinate> ; Default value: 0; Animatable: yes
            y("x"): Length,
            /// The width of the use element.
            /// Value type: <length> ; Default value: 0; Animatable: yes
            width("width"): Length,
            /// The height of the use element.
            /// Value type: <length> ; Default value: 0; Animatable: yes
            height("height"): Length,
        }
    }
);

impl Presentation for UseBuilder {}
impl ConditionalProcessing for UseBuilder {}
