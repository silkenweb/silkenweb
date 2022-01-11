use web_sys as dom;

// TODO: Add all svg elements
svg_element!(
    svg <dom::SvgsvgElement> {
        // TODO: Add all attributes
        attributes {
            width: String,
            height: String
        }

        // TODO: Add events
    }
);

parent_element!(svg);

svg_element!(
    path <dom::SvgPathElement> {
        // TODO: Add all attributes
        attributes {
            d: String,
            stroke: String,
            fill: String
        }

        // TODO: Add events
    }
);
