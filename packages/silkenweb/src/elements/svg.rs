//! SVG Elements

use self::{
    attributes::{
        AnimationTiming, AnimationValue, ConditionalProcessing, FilterPrimitives, OtherAnimation,
        Presentation, TransferFunction,
    },
    content_type::{AutoOrLength, Length},
};
use crate::elements::svg::content_type::NumberOrPercentage;

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
    /// The <circle> SVG element is an SVG basic shape, used to draw circles
    /// based on a center point and a radius.
    circle = {
        dom_type: web_sys::SvgCircleElement;
        attributes {
            /// The x-axis coordinate of the center of the circle. Value type:
            /// <length>|<percentage> ; Default value: 0; Animatable: yes
            cx: Length,

            /// The y-axis coordinate of the center of the circle. Value type:
            /// <length>|<percentage> ; Default value: 0; Animatable: yes
            cy: Length,

            /// The radius of the circle. A value lower or equal to zero
            /// disables rendering of the circle. Value type:
            /// <length>|<percentage> ; Default value: 0; Animatable: yes
            r: Length,

            /// The total length for the circle's circumference, in user units.
            /// Value type: <number> ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl ConditionalProcessing for CircleBuilder {}
impl Presentation for CircleBuilder {}

parent_element!(circle);

svg_element!(
    /// The <clipPath> SVG element defines a clipping path, to be used by the
    /// clip-path property.
    ///
    /// A clipping path restricts the region to which paint can be applied.
    /// Conceptually, parts of the drawing that lie outside of the region
    /// bounded by the clipping path are not drawn.
    clip_path("clipPath") = {
        dom_type: web_sys::SvgClipPathElement;
        attributes {
            /// Defines the coordinate system for the contents of the <clipPath>
            /// element. Value type: userSpaceOnUse|objectBoundingBox ; Default
            /// value: userSpaceOnUse; Animatable: yes
            clip_path_units("clipPathUnits"): String,
        };
    }
);

impl ConditionalProcessing for ClipPathBuilder {}
impl Presentation for ClipPathBuilder {}

parent_element!(clip_path);

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
    /// The <desc> element provides an accessible, long-text description of any
    /// SVG container element or graphics element.
    ///
    /// Text in a <desc> element is not rendered as part of the graphic. If the
    /// element can be described by visible text, it is possible to reference
    /// that text with the aria-describedby attribute. If aria-describedby is
    /// used, it will take precedence over <desc>.
    ///
    /// The hidden text of a <desc> element can also be concatenated with the
    /// visible text of other elements using multiple IDs in an aria-describedby
    /// value. In that case, the <desc> element must provide an ID for
    /// reference.
    desc = {
        dom_type: web_sys::SvgDescElement;
    }
);

parent_element!(desc);

svg_element!(
    /// The <ellipse> element is an SVG basic shape, used to create ellipses
    /// based on a center coordinate, and both their x and y radius.
    ellipse = {
        dom_type: web_sys::SvgEllipseElement;

        attributes {
            /// The x position of the ellipse. Value type: <length>|<percentage>
            /// ; Default value: 0; Animatable: yes
            cx: Length,

            /// The y position of the ellipse. Value type: <length>|<percentage>
            /// ; Default value: 0; Animatable: yes
            cy: Length,

            /// The radius of the ellipse on the x axis. Value type:
            /// auto|<length>|<percentage> ; Default value: auto; Animatable:
            /// yes
            rx: AutoOrLength,

            /// The radius of the ellipse on the y axis. Value type:
            /// auto|<length>|<percentage> ; Default value: auto; Animatable:
            /// yes
            ry: AutoOrLength,
        };
    }
);

parent_element!(ellipse);
impl ConditionalProcessing for EllipseBuilder {}
impl Presentation for EllipseBuilder {}

svg_element!(
    fe_blend("feBlend") = {
        dom_type: web_sys::SvgfeBlendElement;

        attributes {
            r#in: String,
            in2: String,
            mode: String,
        };
    }
);

impl ConditionalProcessing for FeBlendBuilder {}
impl Presentation for FeBlendBuilder {}
impl FilterPrimitives for FeBlendBuilder {}

parent_element!(fe_blend);

svg_element!(
    fe_color_matrix("feColorMatrix") = {
        dom_type: web_sys::SvgfeColorMatrixElement;

        attributes {
            r#in: String,
            r#type: String,
            values: String,
        };
    }
);

impl ConditionalProcessing for FeColorMatrixBuilder {}
impl Presentation for FeColorMatrixBuilder {}
impl FilterPrimitives for FeColorMatrixBuilder {}

parent_element!(fe_color_matrix);

svg_element!(
    fe_component_transfer("feComponentTransfer") = {
        dom_type: web_sys::SvgfeComponentTransferElement;

        attributes { r#in: String };
    }
);

impl ConditionalProcessing for FeComponentTransferBuilder {}
impl Presentation for FeComponentTransferBuilder {}
impl FilterPrimitives for FeComponentTransferBuilder {}

parent_element!(fe_component_transfer);

svg_element!(
    fe_composite("feComposite") = {
        dom_type: web_sys::SvgfeCompositeElement;

        attributes {
            r#in: String,
            in2: String,
            operator: String,
            k1: f64,
            k2: f64,
            k3: f64,
            k4: f64,
        };
    }
);

impl ConditionalProcessing for FeCompositeBuilder {}
impl Presentation for FeCompositeBuilder {}
impl FilterPrimitives for FeCompositeBuilder {}

parent_element!(fe_composite);

svg_element!(
    fe_convolve_matrix("feConvolveMatrix") = {
        dom_type: web_sys::SvgfeConvolveMatrixElement;

        attributes {
            r#in: String,
            order: String,
            kernel_matrix("kernelMatrix"): String,
            divisor: f64,
            bias: f64,
            target_x("targetX"): u64,
            target_y("targetY"): u64,
            edge_mode("edgeMode"): String,
            preserve_alpha("preserveAlpha"): bool,
        };
    }
);

impl ConditionalProcessing for FeConvolveMatrixBuilder {}
impl Presentation for FeConvolveMatrixBuilder {}
impl FilterPrimitives for FeConvolveMatrixBuilder {}

parent_element!(fe_convolve_matrix);

svg_element!(
    fe_diffuse_lighting("feDiffuseLighting") = {
        dom_type: web_sys::SvgfeDiffuseLightingElement;

        attributes {
            r#in: String,
            surface_scale("surfaceScale"): f64,
            diffuse_constant("diffuseConstant"): f64,
        };
    }
);

impl ConditionalProcessing for FeDiffuseLightingBuilder {}
impl Presentation for FeDiffuseLightingBuilder {}
impl FilterPrimitives for FeDiffuseLightingBuilder {}

parent_element!(fe_diffuse_lighting);

svg_element!(
    fe_displacement_map("feDisplacementMap") = {
        dom_type: web_sys::SvgfeDisplacementMapElement;

        attributes {
            r#in: String,
            in2: String,
            scale: f64,
            x_channel_selector("xChannelSelector"): String,
            y_channel_selector("yChannelSelector"): String,
        };
    }
);

impl ConditionalProcessing for FeDisplacementMapBuilder {}
impl Presentation for FeDisplacementMapBuilder {}
impl FilterPrimitives for FeDisplacementMapBuilder {}

parent_element!(fe_displacement_map);

svg_element!(
    fe_distant_light("feDistantLight") = {
        dom_type: web_sys::SvgfeDistantLightElement;

        attributes {
            azimuth: f64,
            elevation: f64,
        };
    }
);

parent_element!(fe_distant_light);

svg_element!(
    fe_flood("feFlood") = {
        dom_type: web_sys::SvgfeFloodElement;

        attributes {
            flood_color: String,
            flood_opacity: NumberOrPercentage,
        };
    }
);

impl ConditionalProcessing for FeFloodBuilder {}
impl Presentation for FeFloodBuilder {}
impl FilterPrimitives for FeFloodBuilder {}

parent_element!(fe_flood);

svg_element!(
    fe_func_a("feFuncA") = {
        dom_type: web_sys::SvgfeFuncAElement;

        attributes {};
    }
);

impl TransferFunction for FeFuncABuilder {}

parent_element!(fe_func_a);

svg_element!(
    fe_func_b("feFuncB") = {
        dom_type: web_sys::SvgfeFuncBElement;

        attributes {};
    }
);

impl TransferFunction for FeFuncBBuilder {}

parent_element!(fe_func_b);

svg_element!(
    fe_func_g("feFuncG") = {
        dom_type: web_sys::SvgfeFuncGElement;

        attributes {};
    }
);

impl TransferFunction for FeFuncGBuilder {}

parent_element!(fe_func_g);

svg_element!(
    fe_func_r("feFuncR") = {
        dom_type: web_sys::SvgfeFuncRElement;

        attributes {};
    }
);

impl TransferFunction for FeFuncRBuilder {}

parent_element!(fe_func_r);

svg_element!(
    fe_gaussian_blur("feGaussianBlur") = {
        dom_type: web_sys::SvgfeGaussianBlurElement;

        attributes {
            r#in: String,
            std_deviation("stdDeviation"): String,
            edge_mode("edgeMode"): String,
        };
    }
);

impl ConditionalProcessing for FeGaussianBlurBuilder {}
impl Presentation for FeGaussianBlurBuilder {}
impl FilterPrimitives for FeGaussianBlurBuilder {}

parent_element!(fe_gaussian_blur);

svg_element!(
    fe_image("feImage") = {
        dom_type: web_sys::SvgfeImageElement;

        attributes {
            preserve_aspect_ratio("preserveAspectRatio"): String,
        };
    }
);

impl ConditionalProcessing for FeImageBuilder {}
impl Presentation for FeImageBuilder {}
impl FilterPrimitives for FeImageBuilder {}

parent_element!(fe_image);

svg_element!(
    fe_merge("feMerge") = {
        dom_type: web_sys::SvgfeMergeElement;

        attributes {};
    }
);

impl ConditionalProcessing for FeMergeBuilder {}
impl Presentation for FeMergeBuilder {}
impl FilterPrimitives for FeMergeBuilder {}

parent_element!(fe_merge);

svg_element!(
    fe_merge_node("feMergeNode") = {
        dom_type: web_sys::SvgfeMergeNodeElement;

        attributes { r#in: String };
    }
);

parent_element!(fe_merge_node);

svg_element!(
    fe_morphology("feMorphology") = {
        dom_type: web_sys::SvgfeMorphologyElement;

        attributes {
            r#in: String,
            operator: String,
            radius: String,
        };
    }
);

impl ConditionalProcessing for FeMorphologyBuilder {}
impl Presentation for FeMorphologyBuilder {}
impl FilterPrimitives for FeMorphologyBuilder {}

parent_element!(fe_morphology);

svg_element!(
    fe_offset("feOffset") = {
        dom_type: web_sys::SvgfeOffsetElement;

        attributes {
            r#in: String,
            dx: f64,
            dy: f64,
        };
    }
);

impl ConditionalProcessing for FeOffsetBuilder {}
impl Presentation for FeOffsetBuilder {}
impl FilterPrimitives for FeOffsetBuilder {}

parent_element!(fe_offset);

svg_element!(
    fe_point_light("fePointLight") = {
        dom_type: web_sys::SvgfePointLightElement;

        attributes {
            x: f64,
            y: f64,
            z: f64,
        };
    }
);

parent_element!(fe_point_light);

svg_element!(
    fe_specular_lighting("feSpecularLighting") = {
        dom_type: web_sys::SvgfeSpecularLightingElement;

        attributes {
            r#in: String,
            surface_scale("surfaceScale"): f64,
            specular_constant("specularConstant"): f64,
            specular_exponent("specularExponent"): f64,
        };
    }
);

impl ConditionalProcessing for FeSpecularLightingBuilder {}
impl Presentation for FeSpecularLightingBuilder {}
impl FilterPrimitives for FeSpecularLightingBuilder {}

parent_element!(fe_specular_lighting);

svg_element!(
    fe_spot_light("feSpotLight") = {
        dom_type: web_sys::SvgfeSpotLightElement;

        attributes {
            x: f64,
            y: f64,
            z: f64,
            points_at_x("pointsAtX"): f64,
            points_at_y("pointsAtY"): f64,
            points_at_z("pointsAtZ"): f64,
            specular_exponent("specularExponent"): f64,
            limiting_cone_angle("limitingConeAngle"): f64,
        };
    }
);

impl ConditionalProcessing for FeSpotLightBuilder {}
impl Presentation for FeSpotLightBuilder {}
impl FilterPrimitives for FeSpotLightBuilder {}

parent_element!(fe_spot_light);

svg_element!(
    fe_tile("feTile") = {
        dom_type: web_sys::SvgfeTileElement;

        attributes { r#in: String };
    }
);

impl ConditionalProcessing for FeTileBuilder {}
impl Presentation for FeTileBuilder {}
impl FilterPrimitives for FeTileBuilder {}

parent_element!(fe_tile);

svg_element!(
    fe_turbulence("feTurbulence") = {
        dom_type: web_sys::SvgfeTurbulenceElement;

        attributes {
            base_frequency("baseFrequency"): String,
            num_octaves("numOctaves"): u64,
            seed: f64,
            stitch_tiles("stitchTiles"): String,
            r#type: String,
        };
    }
);

impl ConditionalProcessing for FeTurbulenceBuilder {}
impl Presentation for FeTurbulenceBuilder {}
impl FilterPrimitives for FeTurbulenceBuilder {}

parent_element!(fe_turbulence);

svg_element!(
    filter = {
        dom_type: web_sys::SvgFilterElement;

        attributes {
            x: Length,
            y: Length,
            width: Length,
            height: Length,
            filter_units("filterUnits"): String,
            primitive_units("primitiveUnits"): String,
        };
    }
);

parent_element!(filter);

impl Presentation for FilterBuilder {}

svg_element!(
    foreign_object("foreignObject") = {
        dom_type: web_sys::SvgFilterElement;

        attributes {
            width: AutoOrLength,
            height: AutoOrLength,
            x: Length,
            y: Length,
        };
    }
);

parent_element!(foreign_object);

impl ConditionalProcessing for ForeignObjectBuilder {}

svg_element!(
    line = {
        dom_type: web_sys::SvgLineElement;

        attributes {};
    }
);

parent_element!(line);

impl ConditionalProcessing for LineBuilder {}
impl Presentation for LineBuilder {}

svg_element!(
    linear_gradient("linearGradient") = {
        dom_type: web_sys::SvgLinearGradientElement;

        attributes {};
    }
);

parent_element!(linear_gradient);

impl Presentation for LinearGradientBuilder {}

svg_element!(
    marker = {
        dom_type: web_sys::SvgMarkerElement;

        attributes {};
    }
);

parent_element!(marker);

impl ConditionalProcessing for MarkerBuilder {}
impl Presentation for MarkerBuilder {}

svg_element!(
    mask = {
        dom_type: web_sys::SvgMaskElement;

        attributes {};
    }
);

parent_element!(mask);

impl ConditionalProcessing for MaskBuilder {}
impl Presentation for MaskBuilder {}

svg_element!(
    metadata = {
        dom_type: web_sys::SvgMetadataElement;

        attributes {};
    }
);

parent_element!(metadata);

svg_element!(
    mpath = {
        dom_type: web_sys::SvgmPathElement;

        attributes {};
    }
);

parent_element!(mpath);

svg_element!(
    pattern = {
        dom_type: web_sys::SvgPatternElement;

        attributes {};
    }
);

parent_element!(pattern);

impl ConditionalProcessing for PatternBuilder {}
impl Presentation for PatternBuilder {}

svg_element!(
    polygon = {
        dom_type: web_sys::SvgPolygonElement;

        attributes {};
    }
);

parent_element!(polygon);

impl ConditionalProcessing for PolygonBuilder {}
impl Presentation for PolygonBuilder {}

svg_element!(
    polyline = {
        dom_type: web_sys::SvgPolylineElement;

        attributes {};
    }
);

parent_element!(polyline);

impl ConditionalProcessing for PolylineBuilder {}
impl Presentation for PolylineBuilder {}

svg_element!(
    script = {
        dom_type: web_sys::SvgScriptElement;

        attributes {};
    }
);

parent_element!(script);

svg_element!(
    set = {
        dom_type: web_sys::SvgSetElement;

        attributes {};
    }
);

parent_element!(set);

impl AnimationTiming for SetBuilder {}
impl OtherAnimation for SetBuilder {}

svg_element!(
    stop = {
        dom_type: web_sys::SvgStopElement;

        attributes {};
    }
);

parent_element!(stop);

impl Presentation for StopBuilder {}

svg_element!(
    style = {
        dom_type: web_sys::SvgStyleElement;

        attributes {};
    }
);

parent_element!(style);

svg_element!(
    switch = {
        dom_type: web_sys::SvgSwitchElement;

        attributes {};
    }
);

parent_element!(switch);

impl ConditionalProcessing for SwitchBuilder {}
impl Presentation for SwitchBuilder {}

svg_element!(
    symbol = {
        dom_type: web_sys::SvgSymbolElement;

        attributes {};
    }
);

parent_element!(symbol);

impl Presentation for SymbolBuilder {}

svg_element!(
    text = {
        dom_type: web_sys::SvgTextElement;

        attributes {};
    }
);

parent_element!(text);

impl ConditionalProcessing for TextBuilder {}
impl Presentation for TextBuilder {}

svg_element!(
    text_path("textPath") = {
        dom_type: web_sys::SvgTextPathElement;

        attributes {};
    }
);

parent_element!(text_path);

impl ConditionalProcessing for TextPathBuilder {}
impl Presentation for TextPathBuilder {}

svg_element!(
    title = {
        dom_type: web_sys::SvgTitleElement;

        attributes {};
    }
);

parent_element!(title);

svg_element!(
    tspan = {
        dom_type: web_sys::SvgtSpanElement;

        attributes {};
    }
);

parent_element!(tspan);

impl ConditionalProcessing for TspanBuilder {}
impl Presentation for TspanBuilder {}

svg_element!(
    view = {
        dom_type: web_sys::SvgViewElement;

        attributes {};
    }
);

parent_element!(view);

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
