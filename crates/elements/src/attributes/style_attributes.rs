use crate::def_attribute;

def_attribute!(
    /// Specify a color as the background of an element.
    ///
    /// You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red"
    ///         }
    ///     )
    /// }
    /// ```
    background,

    /// Specify the opacity of an element's background color.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             background_opacity: "0.5"
    ///         }
    ///     )
    /// }
    /// ```
    background_opacity,

    /// Specify borders for an element.
    ///
    /// The `border` attribute follows this syntax:
    /// border: `<width(s)> <alignment> <fill>`
    ///
    /// Width specification follows CSS-like patterns:
    /// - Single value: Applied to all sides
    /// - Two values: First for top/bottom, second for left/right
    /// - Three values: First for top, second for left/right, third for bottom
    /// - Four values: Top, right, bottom, left (clockwise from top)
    ///
    /// Alignment must be one of:
    /// - `inner`: Border drawn inside the element bounds
    /// - `outer`: Border drawn outside the element bounds
    /// - `center` (default): Border centered on the element bounds
    ///
    /// *Border alignment* determines how the border is positioned relative to the element's edge. Alignment can be `inner`, `outer`, or `center`.
    ///
    /// Note: Borders exist outside the layout system, which means they will be drawn underneath child elements and may overlap with adjacent elements.
    /// Add appropriate padding or margin to prevent border overlap with other content.
    ///
    /// ### Examples
    ///
    /// A solid, black border with a width of 2 pixels on every side. Border is aligned to the inside of the rect's edge.
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             border: "2 inner black",
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// A solid, red border with different widths on each side. Border is aligned to the center of the rect's edge.
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             border: "1 2 3 4 center red",
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// Borders can take any valid fill type, including gradients.
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             border: "1 inner linear-gradient(red, green, yellow 40%, blue)",
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// Similarly to the `shadow` attribute, multiple borders can be drawn on a single element when separated by a comma. Borders specified later in the list are drawn on top of previous ones.
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             border: "6 outer red, 5 outer orange, 4 outer yellow, 3 outer green, 2 outer blue, 1 outer purple",
    ///         }
    ///     )
    /// }
    /// ```
    border,

    /// Draw a shadow of the element.
    ///
    /// The `shadow` attribute follows this syntax:
    /// shadow: `<x> <y> <intensity> <size> <color>`
    ///
    /// - `x` and `y`: Define the offset position of the shadow
    /// - `intensity`: Controls the shadow's blur amount
    /// - `size`: Specifies the shadow's size/spread
    /// - `color`: Any valid color value for the shadow
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             shadow: "0 0 25 2 rgb(0, 0, 0, 120)"
    ///         }
    ///     )
    /// }
    /// ```
    shadow,

    /// Round the corners of an element by a specified radius.
    ///
    /// The `corner_radius` attribute follows this syntax:
    /// corner_radius: `<all> | <tl-tr> <bl-br> | <tl> <tr> <br> <bl>`
    ///
    /// - Single value: Applied to all corners
    /// - Two values: First for top-left & top-right, second for bottom-left & bottom-right
    /// - Four values: Top-left, top-right, bottom-right, bottom-left (clockwise from top-left)
    ///
    /// This creates rounded corners on rectangular elements like rects or buttons.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             corner_radius: "10" // All corners
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             corner_radius: "10 5" // 10 for top corners, 5 for bottom corners
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             corner_radius: "10 20 30 40" // Different radius for each corner
    ///         }
    ///     )
    /// }
    /// ```
    corner_radius,

    /// Control the smoothing effect for rounded corners to create a "squircle" effect.
    ///
    /// Higher values create more of a squircle shape (rounded square), while lower values
    /// result in a more traditionally rounded corner.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             corner_radius: "10",
    ///             corner_smoothing: "75%"
    ///         }
    ///     )
    /// }
    /// ```
    corner_smoothing,

    /// Control the blend mode of this element.
    ///
    /// Possible values:
    /// - `clear`
    /// - `src`
    /// - `dst`
    /// - `src-over`
    /// - `dst-over`
    /// - `src-in`
    /// - `dst-in`
    /// - `src-out`
    /// - `dst-out`
    /// - `src-a-top`
    /// - `dst-a-top`
    /// - `xor`
    /// - `plus`
    /// - `modulate`
    /// - `screen`
    /// - `overlay`
    /// - `darken`
    /// - `lighten`
    /// - `color-dodge`
    /// - `color-burn`
    /// - `hard-light`
    /// - `soft-light`
    /// - `difference`
    /// - `exclusion`
    /// - `multiply`
    /// - `hue`
    /// - `saturation`
    /// - `color`
    /// - `luminosity`
    blend_mode,

    /// Control the blur effect on this element's background.
    ///
    /// A higher value makes it more blurry.
    ///
    /// It is important to note that the element's background must be at least a bit transparent to appreciate the blur effect.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             backdrop_blur: "10",
    ///             background: "rgb(255, 255, 255, 0.4)"
    ///         }
    ///     )
    /// }
    /// ```
    backdrop_blur,
);
