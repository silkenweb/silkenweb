//! SVG Elements

use self::{
    attributes::{
        AnimationTiming, AnimationValue, ConditionalProcessing, FilterPrimitives, OtherAnimation,
        Presentation, TransferFunction,
    },
    content_type::{AutoOrLength, Length},
};
use crate::{dom::Dom, elements::svg::content_type::NumberOrPercentage};

pub mod attributes;
pub mod content_type;
pub mod path;

svg_element!(
    /// The `<a>` SVG element creates a hyperlink to other web pages, files,
    /// locations in the same page, email addresses, or any other URL. It is
    /// very similar to HTML's `<a>` element.
    ///
    /// SVG's `<a>` element is a container, which means you can create a link
    /// around text (like in HTML) but also around any shape.
    a = {
        dom_type: web_sys::SvgaElement;
        attributes {
            /// Instructs browsers to download a URL instead of navigating to
            /// it, so the user will be prompted to save it as a local file.
            /// Value type: `<string>` ; Default value: none; Animatable: no
            download: String,

            /// The URL or URL fragment the hyperlink points to.
            /// Value type: `<URL>` ;
            /// Default value: none; Animatable: yes
            href: String,

            /// The human language of the URL or URL fragment that the hyperlink
            /// points to.
            /// Value type: `<string>` ;
            /// Default value: none; Animatable: yes
            hreflang: String,

            /// A space-separated list of URLs to which, when the hyperlink is
            /// followed, POST requests with the body PING will be sent by the
            /// browser (in the background). Typically used for tracking. For a
            /// more widely-supported feature addressing the same use cases, see
            /// Navigator.sendBeacon().
            /// Value type: `<list-of-URLs>` ;
            /// Default value: none; Animatable: no
            ping: String,

            /// Which referrer to send when fetching the URL.
            /// Value type:
            /// no-referrer|no-referrer-when-downgrade|same-origin|origin|strict-origin|origin-when-cross-origin|strict-origin-when-cross-origin|unsafe-url
            /// ; Default value: none; Animatable: no
            referrerpolicy: String,

            /// The relationship of the target object to the link object.
            /// Value type: `<list-of-Link-Types>` ;
            /// Default value: none; Animatable: yes
            rel: String,

            /// Where to display the linked URL.
            /// Value type: _self|_parent|_top|_blank|`<name>` ;
            /// Default value: _self; Animatable: yes
            target: String,

            /// A MIME type for the linked URL.
            /// Value type: `<string>` ; Default value: none; Animatable: yes
            r#type: String,
        };
    }
);

parent_element!(a);
impl<D: Dom> ConditionalProcessing for A<D> {}
impl<D: Dom> Presentation for A<D> {}

svg_element!(
    /// The SVG `<animate>` element provides a way to animate an attribute of an
    /// element over time.
    animate = {
        dom_type: web_sys::SvgAnimateElement;
    }
);

impl<D: Dom> AnimationTiming for Animate<D> {}
impl<D: Dom> AnimationValue for Animate<D> {}
impl<D: Dom> OtherAnimation for Animate<D> {}

svg_element!(
    /// The `<animateMotion>` SVG element provides a way to define how an
    /// element moves along a motion path.
    animate_motion("animateMotion") = {
        dom_type: web_sys::SvgAnimateMotionElement;

        attributes {
            /// This attribute indicate, in the range [0,1], how far is the
            ///  object along the path for each keyTimes associated values.
            ///  Value type: <number>*; Default value: none; Animatable: no
            key_points("keyPoints"): String,

            /// This attribute defines the path of the motion, using the same
            ///  syntax as the d attribute. Value type: <string>; Default
            ///  value: none; Animatable: no
            path: String,

            /// This attribute defines a rotation applied to the element
            /// animated along a path, usually to make it pointing in the
            /// direction of the animation.
            /// Value type: <number>|auto|auto-reverse; Default value: 0;
            /// Animatable: no
            rotate: String,
        };
    }
);

impl<D: Dom> AnimationTiming for AnimateMotion<D> {}
impl<D: Dom> AnimationValue for AnimateMotion<D> {}
impl<D: Dom> OtherAnimation for AnimateMotion<D> {}

parent_element!(animate_motion);

svg_element!(
    /// The `<animateTransform>` SVG element animates a transformation
    /// attribute on its target element, thereby allowing animations to
    /// control translation, scaling, rotation, and/or skewing.
    animate_transform("animateTransform") = {
        dom_type: web_sys::SvgAnimateTransformElement;

        attributes {
            /// `type` defines the type of transformation, whose values change
            /// over time.
            /// Value: translate | scale | rotate | skewX | skewY
            /// Animatable:	No
            r#type: String,
        };
    }
);

impl<D: Dom> AnimationTiming for AnimateTransform<D> {}
impl<D: Dom> AnimationValue for AnimateTransform<D> {}
impl<D: Dom> OtherAnimation for AnimateTransform<D> {}

parent_element!(animate_transform);

svg_element!(
    /// The `<circle>` SVG element is an SVG basic shape, used to draw circles
    /// based on a center point and a radius.
    circle = {
        dom_type: web_sys::SvgCircleElement;
        attributes {
            /// The x-axis coordinate of the center of the circle. Value type:
            /// `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            cx: Length,

            /// The y-axis coordinate of the center of the circle. Value type:
            /// `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            cy: Length,

            /// The radius of the circle. A value lower or equal to zero
            /// disables rendering of the circle. Value type:
            /// `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            r: Length,

            /// The total length for the circle's circumference, in user units.
            /// Value type: `<number>` ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl<D: Dom> ConditionalProcessing for Circle<D> {}
impl<D: Dom> Presentation for Circle<D> {}

parent_element!(circle);

svg_element!(
    /// The `<clipPath>` SVG element defines a clipping path, to be used by the
    /// clip-path property.
    ///
    /// A clipping path restricts the region to which paint can be applied.
    /// Conceptually, parts of the drawing that lie outside of the region
    /// bounded by the clipping path are not drawn.
    clip_path("clipPath") = {
        dom_type: web_sys::SvgClipPathElement;
        attributes {
            /// Defines the coordinate system for the contents of the `<clipPath>`
            /// element. Value type: userSpaceOnUse|objectBoundingBox ; Default
            /// value: userSpaceOnUse; Animatable: yes
            clip_path_units("clipPathUnits"): String,
        };
    }
);

impl<D: Dom> ConditionalProcessing for ClipPath<D> {}
impl<D: Dom> Presentation for ClipPath<D> {}

parent_element!(clip_path);

svg_element!(
    /// The `<defs>` element is used to store graphical objects that will be
    /// used at a later time. Objects created inside a `<defs>` element are
    /// not rendered directly. To display them you have to reference them
    /// (with a `<use>` element for example).
    ///
    /// Graphical objects can be referenced from anywhere, however, defining
    /// these objects inside of a `<defs>` element promotes understandability of
    /// the SVG content and is beneficial to the overall accessibility of the
    /// document.
    defs = {
        dom_type: web_sys::SvgDefsElement;
    }
);

parent_element!(defs);
impl<D: Dom> ConditionalProcessing for Defs<D> {}
impl<D: Dom> Presentation for Defs<D> {}

svg_element!(
    /// The `<desc>` element provides an accessible, long-text description of
    /// any SVG container element or graphics element.
    ///
    /// Text in a `<desc>` element is not rendered as part of the graphic. If
    /// the element can be described by visible text, it is possible to
    /// reference that text with the aria-describedby attribute. If
    /// aria-describedby is used, it will take precedence over `<desc>`.
    ///
    /// The hidden text of a `<desc>` element can also be concatenated with the
    /// visible text of other elements using multiple IDs in an aria-describedby
    /// value. In that case, the `<desc>` element must provide an ID for
    /// reference.
    desc = {
        dom_type: web_sys::SvgDescElement;
    }
);

parent_element!(desc);

svg_element!(
    /// The `<ellipse>` element is an SVG basic shape, used to create ellipses
    /// based on a center coordinate, and both their x and y radius.
    ellipse = {
        dom_type: web_sys::SvgEllipseElement;

        attributes {
            /// The x position of the ellipse. Value type:
            /// `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            cx: Length,

            /// The y position of the ellipse. Value type:
            /// `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            cy: Length,

            /// The radius of the ellipse on the x axis. Value type:
            /// auto|`<length>`|`<percentage>` ; Default value: auto;
            /// Animatable: yes
            rx: AutoOrLength,

            /// The radius of the ellipse on the y axis. Value type:
            /// auto|`<length>`|`<percentage>` ; Default value: auto;
            /// Animatable: yes
            ry: AutoOrLength,
        };
    }
);

parent_element!(ellipse);
impl<D: Dom> ConditionalProcessing for Ellipse<D> {}
impl<D: Dom> Presentation for Ellipse<D> {}

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

impl<D: Dom> ConditionalProcessing for FeBlend<D> {}
impl<D: Dom> Presentation for FeBlend<D> {}
impl<D: Dom> FilterPrimitives for FeBlend<D> {}

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

impl<D: Dom> ConditionalProcessing for FeColorMatrix<D> {}
impl<D: Dom> Presentation for FeColorMatrix<D> {}
impl<D: Dom> FilterPrimitives for FeColorMatrix<D> {}

parent_element!(fe_color_matrix);

svg_element!(
    fe_component_transfer("feComponentTransfer") = {
        dom_type: web_sys::SvgfeComponentTransferElement;

        attributes { r#in: String };
    }
);

impl<D: Dom> ConditionalProcessing for FeComponentTransfer<D> {}
impl<D: Dom> Presentation for FeComponentTransfer<D> {}
impl<D: Dom> FilterPrimitives for FeComponentTransfer<D> {}

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

impl<D: Dom> ConditionalProcessing for FeComposite<D> {}
impl<D: Dom> Presentation for FeComposite<D> {}
impl<D: Dom> FilterPrimitives for FeComposite<D> {}

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

impl<D: Dom> ConditionalProcessing for FeConvolveMatrix<D> {}
impl<D: Dom> Presentation for FeConvolveMatrix<D> {}
impl<D: Dom> FilterPrimitives for FeConvolveMatrix<D> {}

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

impl<D: Dom> ConditionalProcessing for FeDiffuseLighting<D> {}
impl<D: Dom> Presentation for FeDiffuseLighting<D> {}
impl<D: Dom> FilterPrimitives for FeDiffuseLighting<D> {}

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

impl<D: Dom> ConditionalProcessing for FeDisplacementMap<D> {}
impl<D: Dom> Presentation for FeDisplacementMap<D> {}
impl<D: Dom> FilterPrimitives for FeDisplacementMap<D> {}

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
    fe_drop_shadow("feDropShadow") = {
        dom_type: web_sys::SvgfeDropShadowElement;

        attributes {
            /// This attribute defines the x offset of the drop shadow.
            /// Value type: `<number>`; Default value: 2; Animatable: yes
            dx: f64,

            /// This attribute defines the y offset of the drop shadow.
            /// Value type: `<number>``; Default value: 2; Animatable: yes
            dy: f64,

            /// This attribute defines the standard deviation for the blur
            /// operation in the drop shadow.
            /// Value type: <number-optional-number>; Default value: 2;
            /// Animatable: yes
            std_deviation("stdDeviation"): String,

            r#in: String,
        };
    }
);

impl<D: Dom> Presentation for FeDropShadow<D> {}
impl<D: Dom> FilterPrimitives for FeDropShadow<D> {}

parent_element!(fe_drop_shadow);

svg_element!(
    fe_flood("feFlood") = {
        dom_type: web_sys::SvgfeFloodElement;

        attributes {
            flood_color: String,
            flood_opacity: NumberOrPercentage,
        };
    }
);

impl<D: Dom> ConditionalProcessing for FeFlood<D> {}
impl<D: Dom> Presentation for FeFlood<D> {}
impl<D: Dom> FilterPrimitives for FeFlood<D> {}

parent_element!(fe_flood);

svg_element!(
    fe_func_a("feFuncA") = {
        dom_type: web_sys::SvgfeFuncAElement;

        attributes {};
    }
);

impl<D: Dom> TransferFunction for FeFuncA<D> {}

parent_element!(fe_func_a);

svg_element!(
    fe_func_b("feFuncB") = {
        dom_type: web_sys::SvgfeFuncBElement;

        attributes {};
    }
);

impl<D: Dom> TransferFunction for FeFuncB<D> {}

parent_element!(fe_func_b);

svg_element!(
    fe_func_g("feFuncG") = {
        dom_type: web_sys::SvgfeFuncGElement;

        attributes {};
    }
);

impl<D: Dom> TransferFunction for FeFuncG<D> {}

parent_element!(fe_func_g);

svg_element!(
    fe_func_r("feFuncR") = {
        dom_type: web_sys::SvgfeFuncRElement;

        attributes {};
    }
);

impl<D: Dom> TransferFunction for FeFuncR<D> {}

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

impl<D: Dom> ConditionalProcessing for FeGaussianBlur<D> {}
impl<D: Dom> Presentation for FeGaussianBlur<D> {}
impl<D: Dom> FilterPrimitives for FeGaussianBlur<D> {}

parent_element!(fe_gaussian_blur);

svg_element!(
    fe_image("feImage") = {
        dom_type: web_sys::SvgfeImageElement;

        attributes {
            preserve_aspect_ratio("preserveAspectRatio"): String,
        };
    }
);

impl<D: Dom> ConditionalProcessing for FeImage<D> {}
impl<D: Dom> Presentation for FeImage<D> {}
impl<D: Dom> FilterPrimitives for FeImage<D> {}

parent_element!(fe_image);

svg_element!(
    fe_merge("feMerge") = {
        dom_type: web_sys::SvgfeMergeElement;

        attributes {};
    }
);

impl<D: Dom> ConditionalProcessing for FeMerge<D> {}
impl<D: Dom> Presentation for FeMerge<D> {}
impl<D: Dom> FilterPrimitives for FeMerge<D> {}

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

impl<D: Dom> ConditionalProcessing for FeMorphology<D> {}
impl<D: Dom> Presentation for FeMorphology<D> {}
impl<D: Dom> FilterPrimitives for FeMorphology<D> {}

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

impl<D: Dom> ConditionalProcessing for FeOffset<D> {}
impl<D: Dom> Presentation for FeOffset<D> {}
impl<D: Dom> FilterPrimitives for FeOffset<D> {}

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

impl<D: Dom> ConditionalProcessing for FeSpecularLighting<D> {}
impl<D: Dom> Presentation for FeSpecularLighting<D> {}
impl<D: Dom> FilterPrimitives for FeSpecularLighting<D> {}

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

impl<D: Dom> ConditionalProcessing for FeSpotLight<D> {}
impl<D: Dom> Presentation for FeSpotLight<D> {}
impl<D: Dom> FilterPrimitives for FeSpotLight<D> {}

parent_element!(fe_spot_light);

svg_element!(
    fe_tile("feTile") = {
        dom_type: web_sys::SvgfeTileElement;

        attributes { r#in: String };
    }
);

impl<D: Dom> ConditionalProcessing for FeTile<D> {}
impl<D: Dom> Presentation for FeTile<D> {}
impl<D: Dom> FilterPrimitives for FeTile<D> {}

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

impl<D: Dom> ConditionalProcessing for FeTurbulence<D> {}
impl<D: Dom> Presentation for FeTurbulence<D> {}
impl<D: Dom> FilterPrimitives for FeTurbulence<D> {}

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

impl<D: Dom> Presentation for Filter<D> {}

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

impl<D: Dom> ConditionalProcessing for ForeignObject<D> {}

svg_element!(
    /// The `<g>` SVG element is a container used to group other SVG elements.
    ///
    /// Transformations applied to the `<g>` element are performed on its child
    /// elements, and its attributes are inherited by its children. It can also
    /// group multiple elements to be referenced later with the `<use>` element.
    g = {
        dom_type: web_sys::SvggElement;
    }
);

parent_element!(g);
impl<D: Dom> ConditionalProcessing for G<D> {}
impl<D: Dom> Presentation for G<D> {}

svg_element!(
    line = {
        dom_type: web_sys::SvgLineElement;

        attributes {
            /// Defines the x-axis coordinate of the line starting point. Value
            /// type: `<length>`|`<percentage>`|`<number>` ; Default value: 0;
            /// Animatable: yes
            x1: Length,

            /// Defines the x-axis coordinate of the line ending point. Value
            /// type: `<length>`|`<percentage>`|`<number>` ; Default value: 0;
            /// Animatable: yes
            x2: Length,

            /// Defines the y-axis coordinate of the line starting point. Value
            /// type: `<length>`|`<percentage>`|`<number>` ; Default value: 0;
            /// Animatable: yes
            y1: Length,

            /// Defines the y-axis coordinate of the line ending point. Value
            /// type: `<length>`|`<percentage>`|`<number>` ; Default value: 0;
            /// Animatable: yes
            y2: Length,

            /// Defines the total path length in user units. Value type:
            /// `<number>` ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

parent_element!(line);

impl<D: Dom> ConditionalProcessing for Line<D> {}
impl<D: Dom> Presentation for Line<D> {}

svg_element!(
    marker = {
        dom_type: web_sys::SvgMarkerElement;

        attributes {
            /// This attribute defines the height of the marker viewport. Value
            /// type: `<length>` ; Default value: 3; Animatable: yes
            marker_height("markerHeight"): Length,

            /// This attribute defines the coordinate system for the attributes
            /// markerWidth, markerHeight and the contents of the `<marker>`.
            /// Value type: userSpaceOnUse|strokeWidth ; Default value:
            /// strokeWidth; Animatable: yes
            marker_units("markerUnits"): String,

            /// This attribute defines the width of the marker viewport. Value
            /// type: `<length>` ; Default value: 3; Animatable: yes
            marker_width("markerWidth"): Length,

            /// This attribute defines the orientation of the marker relative to
            /// the shape it is attached to. Value type:
            /// auto|auto-start-reverse|`<angle>` ; Default value: 0; Animatable:
            /// yes
            orient: String,

            /// This attribute defines how the svg fragment must be deformed if
            /// it is embedded in a container with a different aspect ratio.
            /// Value type: (none| xMinYMin| xMidYMin| xMaxYMin| xMinYMid|
            /// xMidYMid| xMaxYMid| xMinYMax| xMidYMax| xMaxYMax) (meet|slice)?
            /// ; Default value: xMidYMid meet; Animatable: yes
            preserve_aspect_ratio("preserveAspectRatio"): String,

            /// This attribute defines the x coordinate for the reference point
            /// of the marker. Value type: left|center|right|`<coordinate>` ;
            /// Default value: 0; Animatable: yes
            ref_x("refX"): String,

            /// This attribute defines the y coordinate for the reference point
            /// of the marker. Value type: top|center|bottom|`<coordinate>` ;
            /// Default value: 0; Animatable: yes
            ref_y("refY"): String,

            /// This attribute defines the bound of the SVG viewport for the
            /// current SVG fragment. Value type: `<list-of-numbers>` ; Default
            /// value: none; Animatable: yes
            view_box("viewBox"): String,
        };
    }
);

parent_element!(marker);

impl<D: Dom> ConditionalProcessing for Marker<D> {}
impl<D: Dom> Presentation for Marker<D> {}

svg_element!(
    mask = {
        dom_type: web_sys::SvgMaskElement;

        attributes {
            /// This attribute defines the height of the masking area. Value
            /// type: `<length>` ; Default value: 120%; Animatable: yes
            height: Length,

            /// This attribute defines the coordinate system for the contents of
            /// the `<mask>`. Value type: userSpaceOnUse|objectBoundingBox ;
            /// Default value: userSpaceOnUse; Animatable: yes
            mask_content_units("maskContentUnits"): String,

            /// This attribute defines the coordinate system for attributes x,
            /// y, width and height on the `<mask>`. Value type:
            /// userSpaceOnUse|objectBoundingBox ; Default value:
            /// objectBoundingBox; Animatable: yes
            mask_units("maskUnits"): String,

            /// This attribute defines the x-axis coordinate of the top-left
            /// corner of the masking area. Value type: `<coordinate>` ; Default
            /// value: -10%; Animatable: yes
            x: Length,

            /// This attribute defines the y-axis coordinate of the top-left
            /// corner of the masking area. Value type: `<coordinate>` ; Default
            /// value: -10%; Animatable: yes
            y: Length,

            /// This attribute defines the width of the masking area. Value
            /// type: `<length>` ; Default value: 120%; Animatable: yes
            width: Length,
        };
    }
);

parent_element!(mask);

impl<D: Dom> ConditionalProcessing for Mask<D> {}
impl<D: Dom> Presentation for Mask<D> {}

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
    /// The `path` SVG element is the generic element to define a shape. All the
    /// basic shapes can be created with a path element.
    path = {
        dom_type: web_sys::SvgPathElement;

        attributes {
            /// This attribute lets authors specify the total length for the
            /// path, in user units.
            /// Value type: `<number>` ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl<D: Dom> ConditionalProcessing for Path<D> {}
impl<D: Dom> Presentation for Path<D> {}

parent_element!(path);

svg_element!(
    pattern = {
        dom_type: web_sys::SvgPatternElement;

        attributes {
            /// This attribute determines the height of the pattern tile. Value
            /// type: `<length>`|`<percentage>`; Default value: 0; Animatable: yes
            height: Length,

            /// This attribute reference a template pattern that provides
            /// default values for the `<pattern>` attributes. Value type: `<URL>`;
            /// Default value: none; Animatable: yes
            href: String,

            /// This attribute defines the coordinate system for the contents of
            /// the `<pattern>`. Value type: userSpaceOnUse|objectBoundingBox;
            /// Default value: userSpaceOnUse; Animatable: yes Note:
            /// This attribute has no effect if a viewBox attribute is specified
            /// on the `<pattern>` element.
            pattern_content_units("patternContentUnits"): String,

            /// This attribute contains the definition of an optional additional
            /// transformation from the pattern coordinate system onto the
            /// target coordinate system. Value type: `<transform-list>`; Default
            /// value: none; Animatable: yes
            pattern_transform("patternTransform"): String,

            /// This attribute defines the coordinate system for attributes x,
            /// y, width, and height. Value type:
            /// userSpaceOnUse|objectBoundingBox; Default value:
            /// objectBoundingBox; Animatable: yes
            pattern_units("patternUnits"): String,

            /// This attribute defines how the SVG fragment must be deformed if
            /// it is embedded in a container with a different aspect ratio.
            /// Value type: (none| xMinYMin| xMidYMin| xMaxYMin| xMinYMid|
            /// xMidYMid| xMaxYMid| xMinYMax| xMidYMax| xMaxYMax) (meet|slice)?
            /// ; Default value: xMidYMid meet; Animatable: yes
            preserve_aspect_ratio("preserveAspectRatio"): String,

            /// This attribute defines the bound of the SVG viewport for the
            /// pattern fragment. Value type: `<list-of-numbers>` ; Default value:
            /// none; Animatable: yes
            view_box("viewBox"): String,

            /// This attribute determines the width of the pattern tile. Value
            /// type: `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            width: Length,

            /// This attribute determines the x coordinate shift of the pattern
            /// tile. Value type: `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            x: Length,

            /// This attribute determines the y coordinate shift of the pattern
            /// tile. Value type: `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            y: Length,
        };
    }
);

parent_element!(pattern);

impl<D: Dom> ConditionalProcessing for Pattern<D> {}
impl<D: Dom> Presentation for Pattern<D> {}

svg_element!(
    polygon = {
        dom_type: web_sys::SvgPolygonElement;

        attributes {
            /// This attribute defines the list of points (pairs of x,y absolute
            /// coordinates) required to draw the polygon. Value type: `<number>`+
            /// ; Default value: ""; Animatable: yes
            points: String,

            /// This attribute lets specify the total length for the path, in
            /// user units. Value type: `<number>` ; Default value: none;
            /// Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

parent_element!(polygon);

impl<D: Dom> ConditionalProcessing for Polygon<D> {}
impl<D: Dom> Presentation for Polygon<D> {}

svg_element!(
    polyline = {
        dom_type: web_sys::SvgPolylineElement;

        attributes {
            /// This attribute defines the list of points (pairs of x,y absolute
            /// coordinates) required to draw the polyline Value type: `<number>`+
            /// ; Default value: ""; Animatable: yes
            points: String,

            /// This attribute lets specify the total length for the path, in
            /// user units. Value type: `<number>` ; Default value: none;
            /// Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

parent_element!(polyline);

impl<D: Dom> ConditionalProcessing for Polyline<D> {}
impl<D: Dom> Presentation for Polyline<D> {}

svg_element!(
    /// The `<rect>` element is a basic SVG shape that draws rectangles, defined
    /// by their position, width, and height. The rectangles may have their
    /// corners rounded.
    rect = {
        dom_type: web_sys::SvgRectElement;

        attributes {
            /// The x coordinate of the rect. Value type: `<length>`|`<percentage>` ; Default
            /// value: 0; Animatable: yes
            x: Length,

            /// The y coordinate of the rect. Value type: `<length>`|`<percentage>` ; Default
            /// value: 0; Animatable: yes
            y: Length,

            /// The width of the rect. Value type: auto|`<length>`|`<percentage>` ; Default
            /// value: auto; Animatable: yes
            width: Length,

            /// The height of the rect. Value type: auto|`<length>`|`<percentage>` ; Default
            /// value: auto; Animatable: yes
            height: Length,

            /// The horizontal corner radius of the rect. Defaults to ry if it is specified.
            /// Value type: auto|`<length>`|`<percentage>` ; Default value: auto; Animatable:
            /// yes
            rx: Length,

            /// The vertical corner radius of the rect. Defaults to rx if it is specified.
            /// Value type: auto|`<length>`|`<percentage>` ; Default value: auto; Animatable:
            /// yes
            ry: Length,

            /// The total length of the rectangle's perimeter, in user units. Value type:
            /// `<number>` ; Default value: none; Animatable: yes
            path_length("pathLength"): f64,
        };
    }
);

impl<D: Dom> ConditionalProcessing for Rect<D> {}
impl<D: Dom> Presentation for Rect<D> {}

parent_element!(rect);

svg_element!(
    script = {
        dom_type: web_sys::SvgScriptElement;

        attributes {
            /// This attribute defines CORS settings as define for the HTML
            /// `<script>` element. Value type: `<string>`; Default value: ?;
            /// Animatable: yes
            crossorigin: String,

            /// The URL to the script to load. Value type: `<URL>` ; Default
            /// value: none; Animatable: no
            href: String,

            /// This attribute defines type of the script language to use. Value
            /// type: `<string>`; Default value: application/ecmascript;
            /// Animatable: no
            r#type: String,
        };
    }
);

parent_element!(script);

svg_element!(
    set = {
        dom_type: web_sys::SvgSetElement;

        attributes {
            /// This attribute defines the value to be applied to the target
            /// attribute for the duration of the animation. The value must
            /// match the requirements of the target attribute. Value type:
            /// `<anything>`; Default value: none; Animatable: no
            to: String,
        };
    }
);

parent_element!(set);

impl<D: Dom> AnimationTiming for Set<D> {}
impl<D: Dom> OtherAnimation for Set<D> {}

svg_element!(
    stop = {
        dom_type: web_sys::SvgStopElement;

        attributes {
            /// This attribute defines where the gradient stop is placed along
            /// the gradient vector. Value type: `<number>`|`<percentage>`;
            /// Default value: 0; Animatable: yes
            offset: Length,

            /// This attribute defines the color of the gradient stop. It can be
            /// used as a CSS property. Value type:
            /// currentcolor|`<color>`|`<icccolor>`; Default value: black;
            /// Animatable: yes
            stop_color: String,

            /// This attribute defines the opacity of the gradient stop. It can
            /// be used as a CSS property. Value type: `<opacity>`; Default
            /// value: 1; Animatable: yes
            stop_opacity: f64,
        };
    }
);

parent_element!(stop);

impl<D: Dom> Presentation for Stop<D> {}

svg_element!(
    style = {
        dom_type: web_sys::SvgStyleElement;

        attributes {
            /// This attribute defines type of the style sheet language to use
            /// as a media type string. Value type: `<string>`; Default value:
            /// text/css; Animatable: no
            r#type: String,

            /// This attribute defines to which media the style applies. Value
            /// type: `<string>`; Default value: all; Animatable: no
            media: String,

            /// This attribute the title of the style sheet which can be used to
            /// switch between alternate style sheets. Value type: `<string>`;
            /// Default value: none; Animatable: no
            title: String,
        };
    }
);

parent_element!(style);

svg_element!(
    /// The svg element is a container that defines a new coordinate system and
    /// viewport. It is used as the outermost element of SVG documents, but it
    /// can also be used to embed an SVG fragment inside an SVG or HTML
    /// document.
    ///
    /// Note: The xmlns attribute is only required on the outermost svg element
    /// of SVG documents. It is unnecessary for inner svg elements or inside
    /// HTML documents.
    svg = {
        dom_type: web_sys::SvgsvgElement;

        attributes {
            /// The displayed height of the rectangular viewport. (Not the
            /// height of its coordinate system.)
            /// Value type: `<length>`|`<percentage>` ; Default value: auto;
            /// Animatable: yes
            height: AutoOrLength,

            /// How the svg fragment must be deformed if it is displayed with a
            /// different aspect ratio.
            /// Value type: (none| xMinYMin| xMidYMin| xMaxYMin| xMinYMid| xMidYMid| xMaxYMid| xMinYMax| xMidYMax| xMaxYMax) (meet|slice)? ;
            /// Default value: xMidYMid meet; Animatable: yes
            preserve_aspect_ratio("preserveAspectRatio"): String,

            /// The SVG viewport coordinates for the current SVG fragment.
            /// Value type: `<list-of-numbers>` ; Default value: none;
            /// Animatable: yes
            view_box("viewBox"): String,

            /// The displayed width of the rectangular viewport. (Not the width
            /// of its coordinate system.) Value type: `<length>`|`<percentage>` ;
            /// Default value: auto; Animatable: yes
            width: AutoOrLength,

            /// The displayed x coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: `<length>`|`<percentage>` ;
            /// Default value: 0; Animatable: yes
            x: Length,

            /// The displayed y coordinate of the svg container. No effect on
            /// outermost svg elements. Value type: `<length>`|`<percentage>` ;
            /// Default value: 0; Animatable: yes
            y: Length,
        };
    }
);

impl<D: Dom> ConditionalProcessing for Svg<D> {}
impl<D: Dom> Presentation for Svg<D> {}

parent_element!(svg);

svg_element!(
    switch = {
        dom_type: web_sys::SvgSwitchElement;

        attributes {};
    }
);

parent_element!(switch);

impl<D: Dom> ConditionalProcessing for Switch<D> {}
impl<D: Dom> Presentation for Switch<D> {}

svg_element!(
    symbol = {
        dom_type: web_sys::SvgSymbolElement;

        attributes {
            /// This attribute determines the height of the symbol. Value type:
            /// `<length>`|`<percentage>` ; Default value: auto; Animatable: yes
            height: Length,

            /// This attribute defines how the svg fragment must be deformed if
            /// it is embedded in a container with a different aspect ratio.
            /// Value type: (none| xMinYMin| xMidYMin| xMaxYMin| xMinYMid|
            /// xMidYMid| xMaxYMid| xMinYMax| xMidYMax| xMaxYMax) (meet|slice)?
            /// ; Default value: xMidYMid meet; Animatable: yes
            preserve_aspect_ratio("preserveAspectRatio"): String,

            /// This attribute determines the x coordinate of the reference
            /// point of the symbol. Value type:
            /// `<length>`|`<percentage>`|left|center|right ; Default value: None;
            /// Animatable: yes
            ref_x("refX"): String,

            /// This attribute determines the y coordinate of the reference
            /// point of the symbol. Value type:
            /// `<length>`|`<percentage>`|top|center|bottom ; Default value: None;
            /// Animatable: yes
            ref_y("refY"): String,

            /// This attribute defines the bound of the SVG viewport for the
            /// current symbol. Value type: `<list-of-numbers>` ; Default value:
            /// none; Animatable: yes
            view_box("viewBox"): String,

            /// This attribute determines the width of the symbol. Value type:
            /// `<length>`|`<percentage>` ; Default value: auto; Animatable: yes
            width: Length,

            /// This attribute determines the x coordinate of the symbol. Value
            /// type: `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            x: Length,

            /// This attribute determines the y coordinate of the symbol. Value
            /// type: `<length>`|`<percentage>` ; Default value: 0; Animatable: yes
            y: Length,
        };
    }
);

parent_element!(symbol);

impl<D: Dom> Presentation for Symbol<D> {}

svg_element!(
    text = {
        dom_type: web_sys::SvgTextElement;

        attributes {
            /// The x coordinate of the starting point of the text baseline.
            /// Value type: `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            x: Length,

            /// The y coordinate of the starting point of the text baseline.
            /// Value type: `<length>`|`<percentage>` ; Default value: 0;
            /// Animatable: yes
            y: Length,

            /// Shifts the text position horizontally from a previous text
            /// element. Value type: `<length>`|`<percentage>` ; Default value:
            /// none; Animatable: yes
            dx: Length,

            /// Shifts the text position vertically from a previous text
            /// element. Value type: `<length>`|`<percentage>` ; Default value:
            /// none; Animatable: yes
            dy: Length,

            /// Rotates orientation of each individual glyph. Can rotate glyphs
            /// individually. Value type: `<list-of-number>` ; Default value:
            /// none; Animatable: yes
            rotate: String,

            /// How the text is stretched or compressed to fit the width defined
            /// by the textLength attribute. Value type:
            /// spacing|spacingAndGlyphs; Default value: spacing; Animatable:
            /// yes
            length_adjust("lengthAdjust"): String,

            /// A width that the text should be scaled to fit. Value type:
            /// `<length>`|`<percentage>` ; Default value: none; Animatable: yes
            text_length("textLength"): Length,
        };
    }
);

parent_element!(text);

impl<D: Dom> ConditionalProcessing for Text<D> {}
impl<D: Dom> Presentation for Text<D> {}

svg_element!(
    text_path("textPath") = {
        dom_type: web_sys::SvgTextPathElement;

        attributes {
            /// The URL to the path or basic shape on which to render the text.
            /// If the path attribute is set, href has no effect. Value type:
            /// `<URL>` ; Default value: none; Animatable: yes
            href: String,

            /// Where length adjustment should be applied to the text: the space
            /// between glyphs, or both the space and the glyphs themselves.
            /// Value type: spacing|spacingAndGlyphs; Default value: spacing;
            /// Animatable: yes
            length_adjust("lengthAdjust"): String,

            /// Which method to render individual glyphs along the path. Value
            /// type: align|stretch ; Default value: align; Animatable: yes
            method: String,

            /// The path on which the text should be rendered. Value type:
            /// `<path_data>` ; Default value: none; Animatable: yes
            path: String,

            /// Which side of the path the text should be rendered. Value type:
            /// left|right ; Default value: left; Animatable: yes
            side: String,

            /// How space between glyphs should be handled. Value type:
            /// auto|exact ; Default value: exact; Animatable: yes
            spacing: String,

            /// How far the beginning of the text should be offset from the
            /// beginning of the path. Value type:
            /// `<length>`|`<percentage>`|`<number>` ; Default value: 0; Animatable:
            /// yes
            start_offset("startOffset"): Length,

            /// The width of the space into which the text will render. Value
            /// type: `<length>`|`<percentage>`|`<number>` ; Default value: auto;
            /// Animatable: yes
            text_length("textLength"): Length,
        };
    }
);

parent_element!(text_path);

impl<D: Dom> ConditionalProcessing for TextPath<D> {}
impl<D: Dom> Presentation for TextPath<D> {}

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

        attributes {
            /// The x coordinate of the starting point of the text baseline.
            /// Value type: `<length>`|`<percentage>` ; Default value: none;
            /// Animatable: yes
            x: Length,

            /// The y coordinate of the starting point of the text baseline.
            /// Value type: `<length>`|`<percentage>` ; Default value: none;
            /// Animatable: yes
            y: Length,

            /// Shifts the text position horizontally from a previous text
            /// element. Value type: `<length>`|`<percentage>` ; Default value:
            /// none; Animatable: yes
            dx: Length,

            /// Shifts the text position vertically from a previous text
            /// element. Value type: `<length>`|`<percentage>` ; Default value:
            /// none; Animatable: yes
            dy: Length,

            /// Rotates orientation of each individual glyph. Can rotate glyphs
            /// individually. Value type: `<list-of-number>` ; Default value:
            /// none; Animatable: yes
            rotate: String,

            /// How the text is stretched or compressed to fit the width defined
            /// by the textLength attribute. Value type:
            /// spacing|spacingAndGlyphs; Default value: spacing; Animatable:
            /// yes
            length_adjust("lengthAdjust"): String,

            /// A width that the text should be scaled to fit. Value type:
            /// `<length>`|`<percentage>` ; Default value: none; Animatable: yes
            text_length("textLength"): Length,
        };
    }
);

parent_element!(tspan);

impl<D: Dom> ConditionalProcessing for Tspan<D> {}
impl<D: Dom> Presentation for Tspan<D> {}

svg_element!(
    /// The `<use>` element takes nodes from within the SVG document, and
    /// duplicates them somewhere else.
    r#use = {
        dom_type: web_sys::SvgUseElement;
        attributes {
            /// The URL to an element/fragment that needs to be duplicated.
            /// Value type: `<URL>` ; Default value: none; Animatable: yes
            href: String,
            /// The x coordinate of the use element.
            /// Value type: `<coordinate>` ; Default value: 0; Animatable: yes
            x: Length,
            /// The y coordinate of the use element.
            /// Value type: `<coordinate>` ; Default value: 0; Animatable: yes
            y: Length,
            /// The width of the use element.
            /// Value type: `<length>` ; Default value: 0; Animatable: yes
            width: Length,
            /// The height of the use element.
            /// Value type: `<length>` ; Default value: 0; Animatable: yes
            height: Length,
        };
    }
);

impl<D: Dom> ConditionalProcessing for Use<D> {}
impl<D: Dom> Presentation for Use<D> {}

parent_element!(use);

svg_element!(
    view = {
        dom_type: web_sys::SvgViewElement;

        attributes {
            view_box("viewBox"): String,
            preserve_aspect_ratio("preserveAspectRatio"): String,
        };
    }
);

parent_element!(view);
