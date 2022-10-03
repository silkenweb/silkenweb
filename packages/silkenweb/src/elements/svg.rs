//! SVG Elements

use self::{
    attributes::{ConditionalProcessing, Presentation},
    content_type::{AutoOrLengthOrPercentage, LengthOrPercentage},
};

pub mod attributes;
pub mod content_type;
pub mod path;

// TODO: Add all svg elements, (element, global) * (attributes, events)
svg_element!(
    svg <web_sys::SvgsvgElement> {
        attributes {

            /// The displayed height of the rectangular viewport. (Not the
            /// height of its coordinate system.)
            /// Value type: <length>|<percentage> ; Default value: auto;
            /// Animatable: yes
            height: AutoOrLengthOrPercentage,

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
            width: AutoOrLengthOrPercentage,

            /// The displayed x coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: <length>|<percentage> ;
            /// Default value: 0; Animatable: yes
            x: LengthOrPercentage,

            /// The displayed y coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: <length>|<percentage> ;
            /// Default value: 0; Animatable: yes
            y: LengthOrPercentage,
        }
    }
);

impl Presentation for SvgBuilder {}
impl ConditionalProcessing for SvgBuilder {}

parent_element!(svg);

svg_element!(
    path <web_sys::SvgPathElement> {
        attributes {
            path_length("pathLength"): f64,
        }
    }
);

impl Presentation for PathBuilder {}
impl ConditionalProcessing for PathBuilder {}

svg_element!(
    rect <web_sys::SvgPathElement> {
        attributes {
            x: LengthOrPercentage,
            y: LengthOrPercentage,
            width: LengthOrPercentage,
            height: LengthOrPercentage,
            rx: LengthOrPercentage,
            ry: LengthOrPercentage,
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
            href("href"): String,
            x("x"): LengthOrPercentage,
            y("x"): LengthOrPercentage,
            width("width"): LengthOrPercentage,
            height("height"): LengthOrPercentage,
        }
    }
);

impl Presentation for UseBuilder {}
impl ConditionalProcessing for UseBuilder {}
