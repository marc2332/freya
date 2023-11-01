use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_platform;
use winit::window::CursorIcon;

/// [`CursorArea`] component properties.
#[derive(Props)]
pub struct CursorAreaProps<'a> {
    /// Cursor icon that will be used when hovering this area.
    icon: CursorIcon,
    /// Inner children for the CursorArea.
    children: Element<'a>,
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
/// fn app(cx: Scope) -> Element {
///     render!(
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
pub fn CursorArea<'a>(cx: Scope<'a, CursorAreaProps<'a>>) -> Element<'a> {
    let platform = use_platform(cx);
    let is_hovering = use_ref(cx, || false);
    let icon = cx.props.icon;

    let onmouseover = {
        to_owned![platform];
        move |_| {
            *is_hovering.write_silent() = true;
            platform.set_cursor(icon);
        }
    };

    let onmouseleave = {
        to_owned![platform];
        move |_| {
            *is_hovering.write_silent() = false;
            platform.set_cursor(CursorIcon::default());
        }
    };

    use_on_unmount(cx, {
        to_owned![is_hovering];
        move || {
            if *is_hovering.read() {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    render!(
        rect {
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            &cx.props.children
        }
    )
}
