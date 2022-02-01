// TODO: Add all svg elements
svg_element!(
    svg <web_sys::SvgsvgElement> {
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
    path <web_sys::SvgPathElement> {
        // TODO: Add all attributes
        attributes {
            d: String,
            stroke: String,
            fill: String
        }

        // TODO: Add events
    }
);

svg_element!(
    circle <web_sys::SvgCircleElement> {
        // TODO: Add all attributes
        attributes {
            cx: String,
            cy: String,
            r: String,
            stroke: String,
            stroke-width: String,
            fill: String,
        }

        // TODO: Add events
    }
);
