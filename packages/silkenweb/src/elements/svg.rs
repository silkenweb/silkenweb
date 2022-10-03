//! SVG Elements

use self::{
    attributes::Presentation,
    content_type::{Length, LengthOrPercentage},
};

pub mod attributes;
pub mod content_type;
pub mod path;

// TODO: Add all svg elements, (element, global) * (attributes, events)
svg_element!(
    svg <web_sys::SvgsvgElement> {
        attributes {
            width: String,
            height: String
        }
    }
);

parent_element!(svg);

svg_element!(
    path <web_sys::SvgPathElement> {
        attributes {
            path_length("pathLength"): f64,
        }
    }
);

impl Presentation for PathBuilder {}

svg_element!(
    rect <web_sys::SvgPathElement> {
        attributes {
            x: LengthOrPercentage,
            y: LengthOrPercentage,
            width: LengthOrPercentage,
            height: LengthOrPercentage,
            rx: LengthOrPercentage,
            ry: LengthOrPercentage,
            transform: String,
            path_length("pathLength"): f64,
        }
    }
);

svg_element!(
    snake(r#use),
    camel(Use, UseBuilder),
    text("use")
    <web_sys::SvgUseElement> {
        attributes {
            href("href"): String,
            x("x"): Length,
            y("x"): Length,
            width("width"): Length,
            height("height"): Length,
        }
    }
);
