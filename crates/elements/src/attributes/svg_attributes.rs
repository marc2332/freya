use crate::def_attribute;

def_attribute!(
    svg_data,
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
