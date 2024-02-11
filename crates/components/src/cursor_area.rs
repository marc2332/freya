use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_platform;
use winit::window::CursorIcon;

/// [`CursorArea`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct CursorAreaProps {
    /// Cursor icon that will be used when hovering this area.
    icon: CursorIcon,
    /// Inner children for the CursorArea.
    children: Element,
}

/// `CursorArea` component.
///
/// # Props
/// See [`CursorAreaProps`].
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// # use winit::window::CursorIcon;
/// fn app() -> Element {
///     rsx!(
///         CursorArea {
///             icon: CursorIcon::Progress,
///             label {
///                 height: "100%",
///                 width: "100%",
///                 "Loading"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn CursorArea(CursorAreaProps { children, icon }: CursorAreaProps) -> Element {
    let platform = use_platform();
    let mut is_hovering = use_signal(|| false);

    let onmouseover = move |_| {
        *is_hovering.write() = true;
        platform.set_cursor(icon);
    };

    let onmouseleave = move |_| {
        *is_hovering.write() = false;
        platform.set_cursor(CursorIcon::default());
    };

    use_drop(move || {
        if *is_hovering.peek() {
            platform.set_cursor(CursorIcon::default());
        }
    });

    rsx!(
        rect {
            onmouseover,
            onmouseleave,
            {children}
        }
    )
}
