use crate::def_attribute;

def_attribute!(
    /// Specify the height for the given element.
    ///
    /// See syntax in [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             height: "50",
    ///             width: "15",
    ///         }
    ///     )
    /// }
    /// ```
    height,

    /// Specify the width for the given element.
    ///
    /// See syntax in [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             height: "50",
    ///             width: "15",
    ///         }
    ///     )
    /// }
    /// ```
    width,

    /// Specify a minimum width for the given element.
    /// This can be useful if you use it alongside a percentage for the target size.
    ///
    /// See syntax for [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             min_width: "100",
    ///             min_height: "100",
    ///             width: "50%",
    ///             height: "50%",
    ///         }
    ///     )
    /// }
    /// ```
    min_width,

    /// Specify a minimum height for the given element.
    /// This can be useful if you use it alongside a percentage for the target size.
    ///
    /// See syntax for [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             min_width: "100",
    ///             min_height: "100",
    ///             width: "50%",
    ///             height: "50%",
    ///         }
    ///     )
    /// }
    /// ```
    min_height,

    /// Specify a maximum width for the given element.
    ///
    /// See syntax for [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             max_width: "50%",
    ///             width: "500",
    ///             height: "500",
    ///         }
    ///     )
    /// }
    /// ```
    max_width,

    /// Specify a maximum height for the given element.
    ///
    /// See syntax for [`Size Units`](crate::_docs::size_unit).
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             max_height: "50%",
    ///             width: "500",
    ///             height: "500",
    ///         }
    ///     )
    /// }
    /// ```
    max_height,

    /// Specify the percentage of width to be visible.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             visible_width: "50%", // 250
    ///             width: "500",
    ///             height: "500",
    ///         }
    ///     )
    /// }
    /// ```
    visible_width,

    /// Specify the percentage of height to be visible.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             background: "red",
    ///             visible_height: "50%", // 250
    ///             width: "500",
    ///             height: "500",
    ///         }
    ///     )
    /// }
    /// ```
    visible_height,

    /// Specify the margin of an element.
    /// You can do so by four different ways, just like in CSS.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             margin: "25", // 25 in all sides
    ///             margin: "100 50", // 100 in top and bottom, and 50 in left and right
    ///             margin: "2 15 25", // 2 in top, 15 in left and right, and 25 in bottom
    ///             margin: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
    ///         }
    ///     )
    /// }
    /// ```
    margin,

    /// Specify the inner paddings of an element. You can do so by four different ways, just like in CSS.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             padding: "25", // 25 in all sides
    ///             padding: "100 50", // 100 in top and bottom, and 50 in left and right
    ///             padding: "2 15 25", // 2 in top, 15 in left and right, and 25 in bottom
    ///             padding: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
    ///         }
    ///     )
    /// }
    /// ```
    padding,

    /// Specify how you want the element to be positioned inside it's parent area.
    ///
    /// Accepted values:
    ///
    /// - `stacked` (default)
    /// - `absolute` (Floating element relative to the parent element)
    /// - `global` (Floating element relative to the window)
    ///
    /// When using the `absolute` or `global` modes, you can also combine them with the following attributes:
    ///
    /// - `position_top`
    /// - `position_right`
    /// - `position_bottom`
    /// - `position_left`
    ///
    /// These only support pixels.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             rect {
    ///                 position: "absolute",
    ///                 position_bottom: "15",
    ///                 position_right: "15",
    ///                 background: "black",
    ///                 width: "100",
    ///                 height: "100",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    position,

    /// Specify the top position of an element when using `position: "absolute"` or `position: "global"`.
    /// This supports pixels only.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             rect {
    ///                 position: "absolute",
    ///                 position_top: "15",
    ///                 position_left: "15",
    ///                 background: "black",
    ///                 width: "100",
    ///                 height: "100",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    position_top,

    /// Specify the right position of an element when using `position: "absolute"` or `position: "global"`.
    /// This supports pixels only.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             rect {
    ///                 position: "absolute",
    ///                 position_top: "15",
    ///                 position_right: "15",
    ///                 background: "black",
    ///                 width: "100",
    ///                 height: "100",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    position_right,

    /// Specify the bottom position of an element when using `position: "absolute"` or `position: "global"`.
    /// This supports pixels only.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             rect {
    ///                 position: "absolute",
    ///                 position_bottom: "15",
    ///                 position_right: "15",
    ///                 background: "black",
    ///                 width: "100",
    ///                 height: "100",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    position_bottom,

    /// Specify the left position of an element when using `position: "absolute"` or `position: "global"`.
    /// This supports pixels only.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             rect {
    ///                 position: "absolute",
    ///                 position_bottom: "15",
    ///                 position_left: "15",
    ///                 background: "black",
    ///                 width: "100",
    ///                 height: "100",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    position_left,

    /// Control how the inner elements stack.
    ///
    /// Accepted values:
    ///
    /// - `vertical` (default)
    /// - `horizontal`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             direction: "vertical",
    ///             rect {
    ///                 width: "100%",
    ///                 height: "50%",
    ///                 background: "red"
    ///             },
    ///             rect {
    ///                 width: "100%",
    ///                 height: "50%",
    ///                 background: "green"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    direction,

    /// Specify how you want the automatic (e.g `width: auto`) bounds in the cross axis to be constrained for the inner elements.
    ///
    /// Accepted values:
    ///
    /// - `normal` (default): Uses parent bounds.
    /// - `fit`: Uses parent bounds but later shrunks to the size of the biggest element inside.
    /// - `flex`: Marks the container as flex container, children of this element will be able to use `size`/`size(n)` in their `width` and `height` attributes.
    ///
    ///
    /// ### `fit`
    ///
    /// The `fit` mode will allow the inner elements using `width: fill-min` to expand to the biggest element inside this element.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             content: "fit",
    ///             height: "100%",
    ///             rect {
    ///                 width: "fill-min", // Will have a width of 300px
    ///                 height: "25%",
    ///                 background: "red",
    ///             }
    ///             rect {
    ///                 width: "150",  // Will have a width of 150px
    ///                 height: "25%",
    ///                 background: "green",
    ///             }
    ///             rect {
    ///                 width: "fill-min",  // Will have a width of 300px
    ///                 height: "25%",
    ///                 background: "blue",
    ///             }
    ///             rect {
    ///                 width: "300",  // Biggest element, will have a width of 300px
    ///                 height: "25%",
    ///                 background: "black",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    content,

    /// ### main_align
    ///
    /// Control how the inner elements are positioned inside the element. You can combine it with the `direction` attribute to create complex flows.
    ///
    /// Accepted values for `main_align`:
    ///
    /// - `start` (default): At the begining of the axis
    /// - `center`: At the center of the axis
    /// - `end`: At the end of the axis
    /// - `space-between`(only for `main_align`): Distributed among the available space
    /// - `space-around` (only for `main_align`): Distributed among the available space with small margins in the sides
    /// - `space-evenly` (only for `main_align`): Distributed among the available space with the same size of margins in the sides and in between the elements
    ///
    /// When using the `vertical` direction it uses the Y axis and in `horizontal` direction it uses the X axis.
    ///
    /// Example on how to center the inner elements in the main axis:
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             main_align: "center",
    ///             rect {
    ///                 width: "50%",
    ///                 height: "50%",
    ///                 background: "red"
    ///             },
    ///         }
    ///     )
    /// }
    /// ```
    main_align,

    /// ### cross_align
    ///
    /// Control how the inner elements are positioned inside the element in the axis perpendicular to the direction.
    ///
    /// Accepted values:
    ///
    /// - `start` (default): At the begining of the axis (same as in `main_align`)
    /// - `center`: At the center of the axis (same as in `main_align`)
    /// - `end`: At the end of the axis (same as in `main_align`)
    ///
    /// When using the `vertical` direction it uses the X axis and in `horizontal` direction it uses the Y axis.
    ///
    /// Example on how to center the inner elements in the cross axis:
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             cross_align: "center",
    ///             rect {
    ///                 width: "50%",
    ///                 height: "50%",
    ///                 background: "red"
    ///             },
    ///         }
    ///     )
    /// }
    /// ```
    cross_align,

    /// Specify a space between the inner elements. Think it as a margin for every element but defined by its parent.
    /// It only applies to the side of the direction.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             direction: "vertical",
    ///             spacing: "20",
    ///             // Not before
    ///             rect {
    ///                 width: "100",
    ///                 height: "100",
    ///                 background: "red",
    ///             }
    ///             // There will be a space between these two elements of 20 pixels
    ///             rect {
    ///                 width: "100",
    ///                 height: "100",
    ///                 background: "blue",
    ///             }
    ///             // Here as well
    ///             rect {
    ///                 width: "100",
    ///                 height: "100",
    ///                 background: "green",
    ///             }
    ///             // But not after
    ///         }
    ///     )
    /// }
    /// ```
    spacing,

    /// Specify how overflow should be handled.
    ///
    /// Accepted values:
    ///
    /// - `clip`
    /// - `none`
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             overflow: "clip",
    ///             width: "100",
    ///             height: "100%",
    ///             rect {
    ///                 width: "500",
    ///                 height: "100%",
    ///                 background: "red",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    overflow,

    /// Moves the content inside of the container horizontally by the specified pixel amount.
    /// This attribute only supports pixels.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "200",
    ///             height: "200",
    ///             background: "gray",
    ///             offset_x: "20", // Shifts content 20 pixels to the right
    ///             rect {
    ///                 width: "100",
    ///                 height: "100",
    ///                 background: "red",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    offset_x,

    /// Moves the content inside of the container vertically by the specified pixel amount.
    /// This attribute only supports pixels.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "200",
    ///             height: "200",
    ///             background: "gray",
    ///             offset_y: "30", // Shifts content 30 pixels down
    ///             rect {
    ///                 width: "100",
    ///                 height: "100",
    ///                 background: "red",
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    offset_y,

    /// Controls the stacking order of elements on the z-axis.
    ///
    /// In Freya, elements are stacked in the order they appear in the DOM, with later elements
    /// appearing on top of earlier ones. The `layer` attribute allows you to explicitly control
    /// this stacking behavior.
    ///
    /// There are two modes:
    /// - `overlay`: Always on top
    /// - Numeric: A lower value positions the element higher in the stack (visually on top), while higher
    ///   values position elements lower in the stack (visually behind), `0` makes the element maintain its default behavior.
    ///   Like with distance, 0 is right on front of you and 1000km away you can't even see it.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     rsx!(
    ///         rect {
    ///             width: "500",
    ///             height: "500",
    ///             // Using an image as the base element
    ///             image {
    ///                 width: "300",
    ///                 height: "200",
    ///                 image_data: static_bytes(include_bytes!("../_docs/rust_logo.png")),
    ///             }
    ///             // Overlay on top of the image using absolute positioning
    ///             rect {
    ///                 position: "absolute",
    ///                 position_top: "0",
    ///                 position_left: "0",
    ///                 width: "300",
    ///                 height: "200",
    ///                 background: "rgba(0, 0, 255, 0.5)", // Semi-transparent blue overlay
    ///                 layer: "-1", // Positions the element in the same layer as its parent, effectively on top of `image`
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    layer,
);
