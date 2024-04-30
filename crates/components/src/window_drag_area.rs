use dioxus::prelude::*;
use freya_hooks::use_platform;
use freya_elements::{elements as dioxus_elements, events::MouseEvent};

/// Allow dragging the window when the cursor drag this component with a left mouse click.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         WindowDragArea {
///             label {
///                 height: "100%",
///                 width: "100%",
///                 "Drag Me"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
#[component]
pub fn WindowDragArea(
    /// The inner children for the WindowDragArea
    children: Element
) -> Element {
    let platform = use_platform();

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();
        platform.drag_window();
    };

    rsx!(
        rect {
            onmousedown,
            {children}
        }
    )
}