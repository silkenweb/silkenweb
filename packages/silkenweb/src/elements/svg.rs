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
            x: String,
            y: String,
            width: String,
            height: String,
            transform: String,
        }
    }
);
