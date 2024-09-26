use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_platform;
pub use winit::window::CursorIcon;

/// Properties for the [`CursorArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CursorAreaProps {
    /// Cursor icon that will be used when hovering this area.
    icon: CursorIcon,
    /// Inner children for the CursorArea.
    children: Element,
}

/// Change the cursor icon when it's hovering over this component.
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
#[allow(non_snake_case)]
pub fn CursorArea(CursorAreaProps { children, icon }: CursorAreaProps) -> Element {
    let platform = use_platform();
    let mut is_hovering = use_signal(|| false);

    let onmousemove = move |_| {
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
            onmousemove,
            onmouseleave,
            {children}
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;
    use winit::window::CursorIcon;

    #[tokio::test]
    pub async fn cursor_area() {
        fn cursor_area_app() -> Element {
            rsx!(
                CursorArea {
                    icon: CursorIcon::Progress,
                    rect {
                        height: "50%",
                        width: "100%",
                    }
                }
                CursorArea {
                    icon: CursorIcon::Pointer,
                    rect {
                        height: "50%",
                        width: "100%",
                    }
                }
            )
        }

        let mut utils = launch_test(cursor_area_app);

        // Initial cursor
        assert_eq!(utils.cursor_icon(), CursorIcon::default());

        utils.move_cursor((100., 100.)).await;

        // Cursor after hovering the first half
        assert_eq!(utils.cursor_icon(), CursorIcon::Progress);

        utils.move_cursor((100., 300.)).await;

        // Cursor after hovering the second half
        assert_eq!(utils.cursor_icon(), CursorIcon::Pointer);

        utils.move_cursor((-1., -1.)).await;

        // Cursor after leaving the window
        assert_eq!(utils.cursor_icon(), CursorIcon::default());
    }
}
