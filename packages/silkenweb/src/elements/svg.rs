//! SVG Elements

use self::content_type::{LengthOrPercentage, Number};
use crate::{attribute::AsAttribute, node::element::ElementBuilder};

pub mod content_type;

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
            d: String,
            stroke: String,
            fill: String
        }
    }
);

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
        }
    }
);

impl RectBuilder {
    pub fn path_length<N: Number>(self, value: impl AsAttribute<N>) -> Self {
        Self {
            builder: self.builder.attribute("pathLength", value),
        }
    }
}
