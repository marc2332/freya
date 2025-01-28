use crate::def_attribute;

def_attribute!(

    /// The `color` attribute lets you specify the color of the text.
    ///
    /// You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             color: "green",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// Another example showing [inheritance](crate::_docs::inheritance):
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             color: "blue",
    ///             label {
    ///                 "Hello, World!"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    color,

    /// You can specify the size of the text using `font_size`.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_size: "50",
    ///             "Hellooooo!"
    ///         }
    ///     )
    /// }
    /// ```
    font_size,

    /// With the `font_family` you can specify what font you want to use for the inner text.
    ///
    /// Check out the [custom font example](https://github.com/marc2332/freya/blob/main/examples/custom_font.rs)
    /// to see how you can load your own fonts.
    ///
    /// <!-- TODO: Example of checking if a font exists with skia_safe -->
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_family: "Inter",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    font_family,

    /// You can choose a style for a text using the `font_style` attribute.
    ///
    /// Accepted values:
    ///
    /// - `upright` (default)
    /// - `italic`
    /// - `oblique`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_style: "italic",
    ///             "Hello, italic World!"
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// You can also specify multiple fonts in order of priority, if one is not found it will fallback to the next one.
    ///
    /// Example:
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_family: "DoesntExist Font, Impact",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    font_style,

    /// You can choose a weight for text using the `font_weight` attribute.
    ///
    /// Accepted values:
    ///
    /// - `invisible`
    /// - `thin`
    /// - `extra-light`
    /// - `light`
    /// - `normal` (default)
    /// - `medium`
    /// - `semi-bold`
    /// - `bold`
    /// - `extra-bold`
    /// - `black`
    /// - `extra-black`
    /// - `50`
    /// - `100`
    /// - `200`
    /// - `300`
    /// - `400`
    /// - `500`
    /// - `600`
    /// - `700`
    /// - `800`
    /// - `900`
    /// - `950`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_weight: "bold",
    ///             "Hello, bold World!"
    ///         }
    ///     )
    /// }
    /// ```
    font_weight,

    /// You can choose a width for a text using the `font_width` attribute.
    ///
    /// ⚠️ Only fonts with variable widths will be affected.
    ///
    /// Accepted values:
    ///
    /// - `ultra-condensed`
    /// - `extra-condensed`
    /// - `condensed`
    /// - `normal` (default)
    /// - `semi-expanded`
    /// - `expanded`
    /// - `extra-expanded`
    /// - `ultra-expanded`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             font_width: "ultra-expanded",
    ///             "Hello, wide World!"
    ///         }
    ///     )
    /// }
    /// ```
    font_width,

    /// You can change the alignment of the text using the `text_align` attribute.
    ///
    /// Accepted values:
    ///
    /// - `center`
    /// - `end`
    /// - `justify`
    /// - `left` (default)
    /// - `right`
    /// - `start`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             text_align: "right",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    text_align,

    /// ### line_height
    ///
    /// Specify the height of the lines of the text.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             line_height: "3",
    ///             "Hello, World! \n Hello, again!"
    ///         }
    ///     )
    /// }
    /// ```
    line_height,

    /// Specify the shadow of a text.
    ///
    /// Syntax: `<x> <y> <size> <color>`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             text_shadow: "0 18 12 rgb(0, 0, 0)",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    text_shadow,

    /// Determines the amount of lines that the text can have. It has unlimited lines by default.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             "Hello, World! \n Hello, World! \n Hello, world!" // Will show all three lines
    ///         }
    ///         label {
    ///             max_lines: "2",
    ///             "Hello, World! \n Hello, World! \n Hello, world!" // Will only show two lines
    ///         }
    ///     )
    /// }
    /// ```
    max_lines,

    /// Specify the decoration in a text.
    ///
    /// Accepted values:
    ///
    /// - `underline`
    /// - `line-through`
    /// - `overline`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             decoration: "line-through",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    decoration,

    /// Specify the decoration's style in a text.
    ///
    /// Accepted values:
    ///
    /// - `solid` (default)
    /// - `double`
    /// - `dotted`
    /// - `dashed`
    /// - `wavy`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             decoration: "line-through",
    ///             decoration_style: "dotted",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    decoration_style,

    /// Specify the decoration’s color in a text.
    ///
    /// You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             decoration: "line-through",
    ///             decoration_color: "orange",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    decoration_color,

    /// Determines how text is treated when it exceeds its [`max_lines`](#max_lines) count. By default uses the `clip` mode, which will cut off any overflowing text, with `ellipsis` mode it will show `...` at the end.
    ///
    /// Accepted values:
    ///
    /// - `clip` (default): Simply cut the text.
    /// - `ellipsis`: Show `…`.
    /// - `[custom-value]: Show a custom value.
    ///
    /// ### Ellipsis example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             max_lines: "3",
    ///             text_overflow: "ellipsis",
    ///             "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// ### Custom value example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             max_lines: "3",
    ///             text_overflow: ".......too long.",
    ///             "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
    ///         }
    ///     )
    /// }
    /// ```
    text_overflow,

    /// Specify the spacing between characters of the text.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             letter_spacing: "10",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    letter_spacing,

    /// Specify the spacing between words of the text.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             word_spacing: "10",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    word_spacing,

    /// Specify the text height behavior.
    ///
    /// Accepted values:
    ///
    /// - `disable-all` (default)
    /// - `all`
    /// - `disable-first-ascent`
    /// - `disable-least-ascent`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         label {
    ///             text_height: "disable-all",
    ///             "Hello, World!"
    ///         }
    ///     )
    /// }
    /// ```
    text_height,
);
