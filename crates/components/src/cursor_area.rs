use dioxus::prelude::*;
use freya_common::EventMessage;
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
    let icon = cx.props.icon;

    let onmouseover = {
        to_owned![platform];
        move |_| {
            platform.send(EventMessage::SetCursorIcon(icon)).unwrap();
        }
    };

    let onmouseleave = move |_| {
        platform
            .send(EventMessage::SetCursorIcon(CursorIcon::default()))
            .unwrap();
    };

    render!(
        rect {
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            &cx.props.children
        }
    )
}
