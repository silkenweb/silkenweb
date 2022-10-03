use futures_signals::signal::{Signal, SignalExt};
use silkenweb_base::intern_str;

use super::{
    content_type::{Length, NumberOrPercentage},
    path::Data,
};
use crate::{elements::class_attribute_text, node::element::ElementBuilder};

/// An SVG element
///
/// Methods for setting attributes specific to SVG elements
pub trait Global: ElementBuilder {
    fn class(self, value: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.attribute(intern_str("class"), class_attribute_text(value))
    }

    fn class_signal<Iter: IntoIterator<Item = impl AsRef<str>>>(
        self,
        value: impl Signal<Item = Iter> + 'static,
    ) -> Self {
        self.attribute_signal(
            intern_str("class"),
            value.map(move |class| class_attribute_text(class)),
        )
    }

    attributes![
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

pub trait AnimationTiming: ElementBuilder {
    attributes![
        /// The begin attribute defines when an animation should begin or when 
        /// an element should be discarded.
        ///
        /// The attribute value is a semicolon separated list of values. The 
        /// interpretation of a list of start times is detailed in the SMIL 
        /// specification in "Evaluation of begin and end time lists". Each 
        /// individual value can be one of the following : 
        ///     - <offset-value>
        ///     - <syncbase-value>
        ///     - <event-value>
        ///     - <repeat-value>
        ///     - <accessKey-value>
        ///     - <wallclock-sync-value>
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
        /// The restart attribute specifies whether or not an animation can restart.
        restart: String,
        // /// The repeatCount attribute indicates the number of times an animation 
        // /// will take place.
        // repeat_count("repeatCount"): String,
        // /// The repeatDur attribute specifies the total duration for repeating an 
        // /// animation.
        // repeat_dur("repeatDur"): String,
        /// The fill attribute has two different meanings. For shapes and text it's a presentation attribute that defines the color (or any SVG paint servers like gradients or patterns) used to paint the element; for animation it defines the final state of the animation.
        fill: String,
    ];
}

pub trait AnimationValue: ElementBuilder {
    attributes![
        // TODO: Add attributes
    ];
}

pub trait OtherAnimation: ElementBuilder {
    attributes![
        // TODO: Add attributes
    ];
}

/// SVG Presentation Attributes
pub trait Presentation: ElementBuilder {
    attributes![
        /// It specifies how an object is aligned along the font baseline with
        /// respect to its parent. Value:
        /// auto|baseline|before-edge|text-before-edge|middle|central|after-edge|text-after-edge|ideographic|alphabetic|hanging|mathematical|inherit;
        /// Animatable: Yes
        alignment
            - baseline: String,
        /// It allows repositioning of the dominant-baseline relative to the
        /// dominant-baseline of the parent text content element. Value:
        /// auto|baseline|super|sub|<percentage>|<length>|inherit; Animatable:
        /// Yes
        baseline
            - shift: String,
        /// It binds the element it is applied to with a given <clipPath>
        /// element. Value: none|<FuncIRI>|inherit; Animatable: Yes
        clip - path: String,
        /// It indicates how to determine what side of a path is inside a shape
        /// in order to know how a <clipPath> should clip its target. Value:
        /// nonezero|evenodd|inherit; Animatable: Yes
        clip - rule: String,
        /// It provides a potential indirect value (currentcolor) for the fill,
        /// stroke, stop-color, flood-color and lighting-color presentation
        /// attributes. Value: <color>|inherit; Animatable: Yes
        color: String,
        /// It specifies the color space for gradient interpolations, color
        /// animations, and alpha compositing. Value:
        /// auto|sRGB|linearRGB|inherit; Animatable: Yes
        color
            - interpolation: String,
        /// It specifies the color space for imaging operations performed via
        /// filter effects. Value: auto|sRGB|linearRGB|inherit; Animatable: Yes
        color
            - interpolation
            - filters: String,
        /// It provides a hint to the browser about how to optimize its color
        /// interpolation and compositing operations. Value:
        /// auto|optimizeSpeed|optimizeQuality|inherit; Animatable: Yes
        color
            - rendering: String,
        /// It specifies the mouse cursor displayed when the mouse pointer is
        /// over an element. Value: <FuncIRI>|<keywords>|inherit; Animatable:
        /// Yes
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
        dominant
            - baseline: String,
        /// It defines the color of the inside of the graphical element it
        /// applies to. Value: <paint>; Animatable: Yes
        fill: String,
        /// It specifies the opacity of the color or the content the current
        /// object is filled with. Value: <number>|<percentage>; Animatable: Yes
        fill - opacity: NumberOrPercentage,
        /// It indicates how to determine what side of a path is inside a shape.
        /// Value: nonzero|evenodd|inherit; Animatable: Yes
        fill - rule: String,
        /// It defines the filter effects defined by the <filter> element that
        /// shall be applied to its element. Value: <FuncIRI>|none|inherit;
        /// Animatable: Yes
        filter: String,
        /// It indicates what color to use to flood the current filter primitive
        /// subregion defined through the <feFlood> or <feDropShadow> element.
        /// Value: <color>; Animatable: Yes
        flood
            - color: String,
        /// It indicates the opacity value to use across the current filter
        /// primitive subregion defined through the <feFlood> or <feDropShadow>
        /// element. Value: <number>|<percentage>; Animatable: Yes
        flood
            - opacity: NumberOrPercentage,
        /// It indicates which font family will be used to render the text of
        /// the element. Value: see CSS font-family; Animatable: Yes
        font - family: String,
        /// It specifies the size of the font. Value: see CSS font-size;
        /// Animatable: Yes
        font - size: String,
        /// It specifies that the font size should be chosen based on the height
        /// of lowercase letters rather than the height of capital letters.
        /// Value: <number>|none|inherit; Animatable: Yes
        font - size
            - adjust: String,
        /// It selects a normal, condensed, or expanded face from a font. Value:
        /// see CSS font-stretch; Animatable: Yes
        font - stretch: String,
        /// It specifies whether a font should be styled with a normal, italic,
        /// or oblique face from its font-family. Value: normal|italic|oblique;
        /// Animatable: Yes
        font - style: String,
        /// It specifies whether a font should be used with some of their
        /// variation such as small caps or ligatures. Value: see CSS
        /// font-variant; Animatable: Yes
        font - variant: String,
        /// It specifies the weight (or boldness) of the font. Value:
        /// normal|bold|lighter|bolder|100|200|300|400|500|600|700|800|900;
        /// Animatable: Yes
        font - weight: String,
        /// It provides a hint to the browser about how to make speed vs.
        /// quality tradeoffs as it performs image processing. Value:
        /// auto|optimizeQuality|optimizeSpeed; Animatable: Yes
        image
            - rendering: String,
        /// It controls spacing between text characters. Value:
        /// normal|<length>|inherit; Animatable: Yes
        letter
            - spacing: String,
        /// It defines the color of the light source for filter primitives
        /// elements <feDiffuseLighting> and <feSpecularLighting>. Value:
        /// <color>; Animatable: Yes
        lighting
            - color: String,
        /// It defines the arrowhead or polymarker that will be drawn at the
        /// final vertex of the given <path> element or basic shape. Value:
        /// <FuncIRI>|none|inherit; Animatable: Yes
        marker
            - end: String,
        /// It defines the arrowhead or polymarker that will be drawn at every
        /// vertex other than the first and last vertex of the given <path>
        /// element or basic shape. Value: <FuncIRI>|none|inherit; Animatable:
        /// Yes
        marker
            - mid: String,
        /// It defines the arrowhead or polymarker that will be drawn at the
        /// first vertex of the given <path> element or basic shape. Value:
        /// <FuncIRI>|none|inherit; Animatable: Yes
        marker
            - start: String,
        /// It alters the visibility of an element by either masking or clipping
        /// the image at specific points. Value: see CSS mask; Animatable: Yes
        mask: String,
        /// It specifies the transparency of an object or a group of objects.
        /// Value: <opacity-value>; Animatable: Yes
        opacity: f64,
        /// Specifies whether the content of a block-level element is clipped
        /// when it overflows the element's box. Value:
        /// visible|hidden|scroll|auto|inherit; Animatable: Yes
        overflow: String,
        /// Defines whether or when an element may be the target of a mouse
        /// event. Value:
        /// bounding-box|visiblePainted|visibleFill|visibleStroke|visible
        /// |painted|fill|stroke|all|none; Animatable: Yes
        pointer
            - events: String,
        /// Hints about what tradeoffs to make as the browser renders <path>
        /// element or basic shapes. Value:
        /// auto|optimizeSpeed|crispEdges|geometricPrecision |inherit;
        /// Animatable: Yes
        shape
            - rendering: String,
        /// - Value:; Animatable: -
        solid
            - color: String,
        /// - Value:; Animatable: -
        solid
            - opacity: String,
        /// Indicates what color to use at that gradient stop. Value:
        /// currentcolor|<color>|<icccolor>|inherit; Animatable: Yes
        stop - color: String,
        /// Defines the opacity of a given gradient stop. Value:
        /// <opacity-value>|inherit; Animatable: Yes
        stop - opacity: String,
        /// Defines the color used to paint the outline of the shape. Value:
        /// <paint>; Animatable: Yes
        stroke: String,
        /// Defines the pattern of dashes and gaps used to paint the outline of
        /// the shape. Value: none|<dasharray>; Animatable: Yes
        stroke
            - dasharray: String,
        /// Defines an offset on the rendering of the associated dash array.
        /// Value: <percentage>|<length>; Animatable: Yes
        stroke
            - dashoffset: Length,
        /// Defines the shape to be used at the end of open subpaths when they
        /// are stroked. Value: butt|round|square; Animatable: Yes
        stroke
            - linecap: String,
        /// Defines the shape to be used at the corners of paths when they are
        /// stroked. Value: arcs|bevel|miter|miter-clip|round; Animatable: Yes
        stroke
            - linejoin: String,
        /// Defines a limit on the ratio of the miter length to the stroke-width
        /// used to draw a miter join. Value: <number>; Animatable: Yes
        stroke
            - miterlimit: f64,
        /// Defines the opacity of the stroke of a shape. Value:
        /// <opacity-value>|<percentage>; Animatable: Yes
        stroke
            - opacity: NumberOrPercentage,
        /// Defines the width of the stroke to be applied to the shape. Value:
        /// <length>|<percentage>; Animatable: Yes
        stroke
            - width: Length,
        /// Defines the vertical alignment a string of text. Value:
        /// start|middle|end|inherit; Animatable: Yes
        text - anchor: String,
        /// Sets the appearance of decorative lines on text. Value:
        /// none|underline|overline|line-through|blink|inherit; Animatable: Yes
        text - decoration: String,
        /// Hints about what tradeoffs to make as the browser renders text.
        /// Value: auto|optimizeSpeed|optimizeLegibility|geometricPrecision|inherit;
        /// Animatable: Yes
        text - rendering: String,
        /// Defines a list of transform definitions that are applied to an
        /// element and the element's children. Value: <transform-list>;
        /// Animatable: Yes
        transform: String,
        /// - Value:; Animatable: -
        unicode
            - bidi: String,
        /// Specifies the vector effect to use when drawing an object. Value:
        /// default|non-scaling-stroke|inherit|<uri>; Animatable: Yes
        vector
            - effect: String,
    ];
}

/// SVG Conditional Processing Attributes
pub trait ConditionalProcessing: ElementBuilder {
    attributes![
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
