use crate::def_attribute;

def_attribute!(
    /// Attach a canvas reference created from the `use_canvas` or
    /// `use_canvas_with_deps` hooks to enable drawing to an element.
    ///
    /// This attribute allows you to bind a canvas context to a Freya element,
    /// giving you the ability to perform custom rendering operations.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     let (reference, size) = use_node_signal();
    ///     let platform = use_platform();
    ///
    ///     let canvas = use_canvas(move || {
    ///         platform.invalidate_drawing_area(size.peek().area);
    ///         platform.request_animation_frame();
    ///         move |ctx| {
    ///             // Custom drawing code here,
    ///             // you will need to add skia as a dependency and look into how to use a skia canvas
    ///         }
    ///     });
    ///
    ///     rsx!(
    ///         rect {
    ///             background: "white",
    ///             width: "300",
    ///             height: "200",
    ///             canvas_reference: canvas.attribute(),
    ///             reference,
    ///         }
    ///     )
    /// }
    /// ```
    canvas_reference,

    /// Attach a reference to an element to track its layout and metadata.
    ///
    /// This attribute is used in conjunction with hooks like `use_node`, `use_node_signal`,
    /// or other node reference hooks to observe and respond to changes in an element's layout.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// fn app() -> Element {
    ///     // Basic usage with use_node
    ///     let (reference, layout) = use_node();
    ///
    ///     // Alternative usage with use_node_signal for reactive access
    ///     // let (reference, layout_signal) = use_node_signal();
    ///
    ///     rsx!(
    ///         rect {
    ///             width: "100%",
    ///             height: "100%",
    ///             reference,
    ///             label {
    ///                 "Width: {layout.area.width()}, Height: {layout.area.height()}"
    ///             }
    ///         }
    ///     )
    /// }
    /// ```
    reference,

    /// This attribute is typically used with text components or custom editors that need to
    /// control cursor placement and selection programmatically. It's obtained from hooks like
    /// `use_editable` that manage text editing functionality.
    cursor_reference,
);
