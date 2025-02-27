use crate::def_attribute;

def_attribute!(
    /// Specify the rotation for this element.
    ///
    /// Syntax is `<0-360>deg`.
    ///
    /// Note: Rotations don't affect neither layout or mouse events, they are merely a rendering effect.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             rotate: "180deg",
    ///             label {
    ///                 "Freya!"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    rotate,

    /// Specify the opacity for this element.
    ///
    /// Accepted values is from `0` to `1`.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             rotation: "0.5",
    ///             label {
    ///                 "Freya!"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    opacity,

    /// Specify the scale for this element.
    ///
    /// Accepted syntax:
    /// - `<f32>`: Same value for both scale x and y.
    /// - `<f32>, <f32>`: Specify the scale x and y separately.
    ///
    /// Note: Scaling doesn't affect neither layout or mouse events, it is only a rendering effect.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             scale: "0.7",
    ///             label {
    ///                 "Freya!"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    scale,

);
