//! Groups of SVG attributes.

use super::{
    content_type::{Length, NumberOrPercentage},
    path::Data,
};
use crate::node::element::Element;

macro_rules! svg_attributes {
    ($($t:tt)*) => {
        attributes![
            [
                attribute_parent = (),
                attribute_doc_macro = svg_attribute_doc
            ]

            $($t)*
        ];
    };
}

/// SVG [core] attributes
///
/// [core]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Core
pub trait Core: Element {
    svg_attributes![
        /// Defines a unique identifier (ID) which must be unique in the whole
        /// document. Its purpose is to identify the element when linking (using
        /// a fragment identifier), scripting, or styling (with CSS). Value: Any
        /// valid ID string; Animatable: No.
        id: String,
        /// Participates in defining the language of the element, the language
        /// that non-editable elements are written in or the language that
        /// editable elements should be written in. The tag contains one single
        /// entry value in the format defined in
        /// [RFC 5646: Tags for Identifying Languages (also known as BCP 47)](https://datatracker.ietf.org/doc/html/rfc5646).
        lang: String,
        /// The tabindex SVG attribute allows you to control whether an element
        /// is focusable and to define the relative order of the element for the
        /// purposes of sequential focus navigation. Animatable: No
        tabindex: i32,
        /// It specifies style information for its element. It functions
        /// identically to the style attribute in HTML. Value: Any valid style
        /// string; Animatable: No
        style: String,
    ];
}

/// SVG [animation] timing attributes
///
/// [animation]:https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#animation_attributes
pub trait AnimationTiming: Element {
    svg_attributes![
        /// The begin attribute defines when an animation should begin or when
        /// an element should be discarded.
        ///
        /// The attribute value is a semicolon separated list of values. The
        /// interpretation of a list of start times is detailed in the SMIL
        /// specification in "Evaluation of begin and end time lists". Each
        /// individual value can be one of the following :
        ///     - `<offset-value>`
        ///     - `<syncbase-value>`
        ///     - `<event-value>`
        ///     - `<repeat-value>`
        ///     - `<accessKey-value>`
        ///     - `<wallclock-sync-value>`
        ///     - the keyword indefinite.
        begin: String,
        /// The dur attribute indicates the simple duration of an animation.
        dur: String,
        /// The end attribute defines an end value for the animation that can
        /// constrain the active duration.
        end: String,
        /// The min attribute specifies the minimum value of the active
        /// animation duration.
        min: String,
        /// The max attribute specifies the maximum value of the active
        /// animation duration.
        max: String,
        /// The restart attribute specifies whether or not an animation can
        /// restart.
        restart: String,
        /// The repeatCount attribute indicates the number of times an animation
        /// will take place.
        repeat_count("repeatCount"): String,
        /// The repeatDur attribute specifies the total duration for repeating
        /// an animation.
        repeat_dur("repeatDur"): String,
        /// The fill attribute has two different meanings. For shapes and text
        /// it's a presentation attribute that defines the color (or any SVG
        /// paint servers like gradients or patterns) used to paint the element;
        /// for animation it defines the final state of the animation.
        fill: String,
    ];
}

/// SVG [animation] value attributes
///
/// [animation]:https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#animation_attributes
pub trait AnimationValue: Element {
    svg_attributes![
        /// The calcMode attribute specifies the interpolation mode for the
        /// animation.
        ///
        /// The default mode is linear, however if the attribute does not
        /// support linear interpolation (e.g. for strings), the calcMode
        /// attribute is ignored and discrete interpolation is used.
        calc_mode("calcMode"): String,
        /// The values attribute has different meanings, depending upon the
        /// context where it's used, either it defines a sequence of values used
        /// over the course of an animation, or it's a list of numbers for a
        /// color matrix, which is interpreted differently depending on the type
        /// of color change to be performed.
        values: String,
        /// The keyTimes attribute represents a list of time values used to
        /// control the pacing of the animation.
        ///
        /// Each time in the list corresponds to a value in the values attribute
        /// list, and defines when the value is used in the animation. Each time
        /// value in the keyTimes list is specified as a floating point value
        /// between 0 and 1 (inclusive), representing a proportional offset into
        /// the duration of the animation element.
        key_times("keyTimes"): String,
        /// The keySplines attribute defines a set of Bézier curve control
        /// points associated with the keyTimes list, defining a cubic Bézier
        /// function that controls interval pacing.
        ///
        /// This attribute is ignored unless the calcMode attribute is set to
        /// spline.
        ///
        /// If there are any errors in the keySplines specification (bad values,
        /// too many or too few values), the animation will not occur.
        key_splines("keySplines"): String,
        /// The from attribute indicates the initial value of the attribute that
        /// will be modified during the animation.
        ///
        /// When used with the to attribute, the animation will change the
        /// modified attribute from the from value to the to value. When used
        /// with the by attribute, the animation will change the attribute
        /// relatively from the from value by the value specified in by.
        from: String,
        /// The to attribute indicates the final value of the attribute that
        /// will be modified during the animation.
        ///
        /// The value of the attribute will change between the from attribute
        /// value and this value.
        to: String,
        /// The by attribute specifies a relative offset value for an attribute
        /// that will be modified during an animation.
        ///
        /// The starting value for the attribute is either indicated by
        /// specifying it as value for the attribute given in the attributeName
        /// or the from attribute.
        by: String,
        /// Undocumented in MDN
        auto_reverse("autoReverse"): String,
        /// Undocumented in MDN
        accelerate: String,
        /// Undocumented in MDN
        decelerate: String,
    ];
}

/// Other SVG [animation] attributes
///
/// [animation]:https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#animation_attributes
pub trait OtherAnimation: Element {
    svg_attributes![
        /// The attributeName attribute indicates the name of the CSS property
        /// or attribute of the target element that is going to be changed
        /// during an animation.
        attribute_name("attributeName"): String,
        /// The additive attribute controls whether or not an animation is
        /// additive.
        additive: String,
        /// The accumulate attribute controls whether or not an animation is
        /// cumulative.
        ///
        /// It is frequently useful for repeated animations to build upon the
        /// previous results, accumulating with each iteration. This attribute
        /// said to the animation if the value is added to the previous animated
        /// attribute's value on each iteration.
        accumulate: String,
    ];
}

/// SVG [Presentation] Attributes
///
/// [Presentation]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Presentation
pub trait Presentation: Element {
    svg_attributes![
        /// It specifies how an object is aligned along the font baseline with
        /// respect to its parent. Value:
        /// auto|baseline|before-edge|text-before-edge|middle|central|after-edge|text-after-edge|ideographic|alphabetic|hanging|mathematical|inherit;
        /// Animatable: Yes
        alignment_baseline: String,
        /// It allows repositioning of the dominant-baseline relative to the
        /// dominant-baseline of the parent text content element. Value:
        /// auto|baseline|super|sub|`<percentage>`|`<length>`|inherit;
        /// Animatable: Yes
        baseline_shift: String,
        /// It binds the element it is applied to with a given `<clipPath>`
        /// element. Value: none|`<FuncIRI>`|inherit; Animatable: Yes
        clip_path: String,
        /// It indicates how to determine what side of a path is inside a shape
        /// in order to know how a `<clipPath>` should clip its target. Value:
        /// nonezero|evenodd|inherit; Animatable: Yes
        clip_rule: String,
        /// It provides a potential indirect value (currentcolor) for the fill,
        /// stroke, stop-color, flood-color and lighting-color presentation
        /// attributes. Value: `<color>`|inherit; Animatable: Yes
        color: String,
        /// It specifies the color space for gradient interpolations, color
        /// animations, and alpha compositing. Value:
        /// auto|sRGB|linearRGB|inherit; Animatable: Yes
        color_interpolation: String,
        /// It specifies the color space for imaging operations performed via
        /// filter effects. Value: auto|sRGB|linearRGB|inherit; Animatable: Yes
        color_interpolation_filters: String,
        /// It provides a hint to the browser about how to optimize its color
        /// interpolation and compositing operations. Value:
        /// auto|optimizeSpeed|optimizeQuality|inherit; Animatable: Yes
        color_rendering: String,
        /// It specifies the mouse cursor displayed when the mouse pointer is
        /// over an element. Value: `<FuncIRI>`|`<keywords>`|inherit;
        /// Animatable: Yes
        cursor: String,
        /// It defines a path to be drawn. Value: path()|none
        d: Data,
        /// It specifies the base writing direction of text. Value:
        /// ltr|rtl|inherit; Animatable: Yes
        direction: String,
        /// It allows to control the rendering of graphical or container
        /// elements. Value: see CSS display; Animatable: Yes
        display: String,
        /// It defines the baseline used to align the box's text and
        /// inline-level contents. Value:
        /// auto|text-bottom|alphabetic|ideographic|middle|central|
        /// mathematical|hanging|text-top; Animatable: Yes
        dominant_baseline: String,
        /// It defines the color of the inside of the graphical element it
        /// applies to. Value: `<paint>`; Animatable: Yes
        fill: String,
        /// It specifies the opacity of the color or the content the current
        /// object is filled with. Value: `<number>`|`<percentage>`; Animatable:
        /// Yes
        fill_opacity: NumberOrPercentage,
        /// It indicates how to determine what side of a path is inside a shape.
        /// Value: nonzero|evenodd|inherit; Animatable: Yes
        fill_rule: String,
        /// It defines the filter effects defined by the `<filter>` element that
        /// shall be applied to its element. Value: `<FuncIRI>`|none|inherit;
        /// Animatable: Yes
        filter: String,
        /// It indicates what color to use to flood the current filter primitive
        /// subregion defined through the `<feFlood>` or `<feDropShadow>`
        /// element. Value: `<color>`; Animatable: Yes
        flood_color: String,
        /// It indicates the opacity value to use across the current filter
        /// primitive subregion defined through the `<feFlood>` or
        /// `<feDropShadow>` element. Value: `<number>`|`<percentage>`;
        /// Animatable: Yes
        flood_opacity: NumberOrPercentage,
        /// It indicates which font family will be used to render the text of
        /// the element. Value: see CSS font-family; Animatable: Yes
        font_family: String,
        /// It specifies the size of the font. Value: see CSS font-size;
        /// Animatable: Yes
        font_size: String,
        /// It specifies that the font size should be chosen based on the height
        /// of lowercase letters rather than the height of capital letters.
        /// Value: `<number>`|none|inherit; Animatable: Yes
        font_size_adjust: String,
        /// It selects a normal, condensed, or expanded face from a font. Value:
        /// see CSS font-stretch; Animatable: Yes
        font_stretch: String,
        /// It specifies whether a font should be styled with a normal, italic,
        /// or oblique face from its font-family. Value: normal|italic|oblique;
        /// Animatable: Yes
        font_style: String,
        /// It specifies whether a font should be used with some of their
        /// variation such as small caps or ligatures. Value: see CSS
        /// font-variant; Animatable: Yes
        font_variant: String,
        /// It specifies the weight (or boldness) of the font. Value:
        /// normal|bold|lighter|bolder|100|200|300|400|500|600|700|800|900;
        /// Animatable: Yes
        font_weight: String,
        /// It provides a hint to the browser about how to make speed vs.
        /// quality tradeoffs as it performs image processing. Value:
        /// auto|optimizeQuality|optimizeSpeed; Animatable: Yes
        image_rendering: String,
        /// It controls spacing between text characters. Value:
        /// normal|`<length>`|inherit; Animatable: Yes
        letter_spacing: String,
        /// It defines the color of the light source for filter primitives
        /// elements `<feDiffuseLighting>` and `<feSpecularLighting>`. Value:
        /// `<color>`; Animatable: Yes
        lighting_color: String,
        /// It defines the arrowhead or polymarker that will be drawn at the
        /// final vertex of the given `<path>` element or basic shape. Value:
        /// `<FuncIRI>`|none|inherit; Animatable: Yes
        marker_end: String,
        /// It defines the arrowhead or polymarker that will be drawn at every
        /// vertex other than the first and last vertex of the given `<path>`
        /// element or basic shape. Value: `<FuncIRI>`|none|inherit; Animatable:
        /// Yes
        marker_mid: String,
        /// It defines the arrowhead or polymarker that will be drawn at the
        /// first vertex of the given `<path>` element or basic shape. Value:
        /// `<FuncIRI>`|none|inherit; Animatable: Yes
        marker_start: String,
        /// It alters the visibility of an element by either masking or clipping
        /// the image at specific points. Value: see CSS mask; Animatable: Yes
        mask: String,
        /// It specifies the transparency of an object or a group of objects.
        /// Value: `<opacity-value>`; Animatable: Yes
        opacity: f64,
        /// Specifies whether the content of a block-level element is clipped
        /// when it overflows the element's box. Value:
        /// visible|hidden|scroll|auto|inherit; Animatable: Yes
        overflow: String,
        /// Defines whether or when an element may be the target of a mouse
        /// event. Value:
        /// bounding-box|visiblePainted|visibleFill|visibleStroke|visible
        /// |painted|fill|stroke|all|none; Animatable: Yes
        pointer_events: String,
        /// Hints about what tradeoffs to make as the browser renders `<path>`
        /// element or basic shapes. Value:
        /// auto|optimizeSpeed|crispEdges|geometricPrecision |inherit;
        /// Animatable: Yes
        shape_rendering: String,
        /// - Value:; Animatable: -
        solid_color: String,
        ///_Value:; Animatable: -
        solid_opacity: String,
        /// Indicates what color to use at that gradient stop. Value:
        /// currentcolor|`<color>`|`<icccolor>`|inherit; Animatable: Yes
        stop_color: String,
        /// Defines the opacity of a given gradient stop. Value:
        /// `<opacity-value>`|inherit; Animatable: Yes
        stop_opacity: String,
        /// Defines the color used to paint the outline of the shape. Value:
        /// `<paint>`; Animatable: Yes
        stroke: String,
        /// Defines the pattern of dashes and gaps used to paint the outline of
        /// the shape. Value: none|`<dasharray>`; Animatable: Yes
        stroke_dasharray: String,
        /// Defines an offset on the rendering of the associated dash array.
        /// Value: `<percentage>`|`<length>`; Animatable: Yes
        stroke_dashoffset: Length,
        /// Defines the shape to be used at the end of open subpaths when they
        /// are stroked. Value: butt|round|square; Animatable: Yes
        stroke_linecap: String,
        /// Defines the shape to be used at the corners of paths when they are
        /// stroked. Value: arcs|bevel|miter|miter-clip|round; Animatable: Yes
        stroke_linejoin: String,
        /// Defines a limit on the ratio of the miter length to the stroke-width
        /// used to draw a miter join. Value: `<number>`; Animatable: Yes
        stroke_miterlimit: f64,
        /// Defines the opacity of the stroke of a shape. Value:
        /// `<opacity-value>`|`<percentage>`; Animatable: Yes
        stroke_opacity: NumberOrPercentage,
        /// Defines the width of the stroke to be applied to the shape. Value:
        /// `<length>`|`<percentage>`; Animatable: Yes
        stroke_width: Length,
        /// Defines the vertical alignment a string of text. Value:
        /// start|middle|end|inherit; Animatable: Yes
        text_anchor: String,
        /// Sets the appearance of decorative lines on text. Value:
        /// none|underline|overline|line-through|blink|inherit; Animatable: Yes
        text_decoration: String,
        /// Hints about what tradeoffs to make as the browser renders text.
        /// Value: auto|optimizeSpeed|optimizeLegibility|geometricPrecision|inherit;
        /// Animatable: Yes
        text_rendering: String,
        /// Defines a list of transform definitions that are applied to an
        /// element and the element's children. Value: `<transform-list>`;
        /// Animatable: Yes
        transform: String,
        /// - Value:; Animatable: -
        unicode_bidi: String,
        /// Specifies the vector effect to use when drawing an object. Value:
        /// default|non-scaling-stroke|inherit|`<uri>`; Animatable: Yes
        vector_effect: String,
    ];
}

/// SVG [Conditional Processing] Attributes
///
/// [Conditional Processing]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/Conditional_Processing
pub trait ConditionalProcessing: Element {
    svg_attributes![
        /// List all the browser specific capabilities that must be supported by
        /// the browser to be allowed to render the associated element. Value: A
        /// list of space-separated URI; Animatable: No
        required_extensions("requiredExtensions"): String,
        /// Indicates which language the user must have chosen to render the
        /// associated element. Value: A list of comma-separated language tags
        /// according to RFC 5646: Tags for Identifying Languages (also known as
        /// BCP 47); Animatable: No
        system_language("systemLanguage"): String,
    ];
}

/// SVG [Filter] Primitve Attributes
///
/// [Filter]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#filters_attributes
pub trait FilterPrimitives: Element {
    svg_attributes![
        /// The height attribute defines the vertical length of an element in
        /// the user coordinate system.
        height: Length,
        /// The result attribute defines the assigned name for this filter
        /// primitive. If supplied, then graphics that result from processing
        /// this filter primitive can be referenced by an in attribute on a
        /// subsequent filter primitive within the same `<filter>` element. If
        /// no value is provided, the output will only be available for
        /// re-use as the implicit input into the next filter primitive
        /// if that filter primitive provides no value for its in
        /// attribute.
        result: String,
        /// The width attribute defines the horizontal length of an element in
        /// the user coordinate system.
        width: Length,
        /// The x attribute defines an x-axis coordinate in the user coordinate
        /// system.
        x: Length,
        /// The y attribute defines a y-axis coordinate in the user coordinate
        /// system.
        y: Length,
    ];
}

/// SVG [Filter] Transfer Function Attributes
///
/// [Filter]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#filters_attributes
pub trait TransferFunction: Element {
    svg_attributes![
        /// The type attribute is a generic attribute and it has different
        /// meaning based on the context in which it's used.
        r#type: String,
        /// The tableValues attribute defines a list of numbers defining a
        /// lookup table of values for a color component transfer function.
        table_values("tableValues"): String,
        /// The intercept attribute defines the intercept of the linear function
        /// of color component transfers when the type attribute is set to
        /// linear.
        intercept: f64,
        /// The amplitude attribute controls the amplitude of the gamma function
        /// of a component transfer element when its type attribute is gamma.
        amplitude: f64,
        /// The exponent attribute defines the exponent of the gamma function.
        exponent: f64,
    ];
}
