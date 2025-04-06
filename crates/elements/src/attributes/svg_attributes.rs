use crate::def_attribute;

def_attribute!(
    /// The `svg_data` attribute lets you provide raw SVG data directly.
    ///
    /// This is similar to the `image_data` attribute but specifically for SVG data.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     let svg_bytes = vec![]; // your SVG bytes here
    ///
    ///     rsx!(
    ///         svg {
    ///             width: "100%",
    ///             height: "100%",
    ///             svg_data: dynamic_bytes(svg_bytes),
    ///         }
    ///     )
    /// }
    /// ```
    svg_data,

    /// The `svg_content` attribute lets you provide SVG content as a string.
    ///
    /// This is useful for including SVG content directly or from external files.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     let svg_content = include_str!("../../../../examples/settings.svg");
    ///
    ///     rsx!(
    ///         svg {
    ///             width: "100%",
    ///             height: "100%",
    ///             svg_content,
    ///         }
    ///     )
    /// }
    /// ```
    svg_content,

    /// The `fill` attributes allows you to specify the fill color for the `svg`.
    ///
    /// You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     let svg_content = include_str!("../../../../examples/settings.svg");
    ///
    ///     rsx!(
    ///         svg {
    ///             fill: "red",
    ///             width: "100%",
    ///             height: "100%",
    ///             svg_content,
    ///         }
    ///     )
    /// }
    /// ```
    fill,

    /// The `stroke` attributes allows you to specify stroke color for the `svg`.
    ///
    /// You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     let svg_content = include_str!("../../../../examples/settings.svg");
    ///
    ///     rsx!(
    ///         svg {
    ///             stroke: "red",
    ///             width: "100%",
    ///             height: "100%",
    ///             svg_content,
    ///         }
    ///     )
    /// }
    /// ```
    stroke,
);
