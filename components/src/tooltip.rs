use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_get_theme;

/// [`Tooltip`] component properties.
#[derive(Props)]
pub struct TooltipProps<'a> {
    /// Url as the Tooltip destination.
    url: &'a str,
}

/// `Tooltip` component
///
/// # Props
/// See [`TooltipProps`].
///
/// # Styling
/// Inherits the [`TooltipTheme`](freya_hooks::TooltipTheme)
///
#[allow(non_snake_case)]
pub fn Tooltip<'a>(cx: Scope<'a, TooltipProps<'a>>) -> Element {
    let theme = use_get_theme(cx);
    let theme = &theme.tooltip;
    render!(
        rect {
            height: "30",
            padding: "2",
            width: "170",
            direction: "both",
            rect {
                direction: "both",
                width: "100%",
                height: "100%",
                shadow: "0 5 25 3 black",
                radius: "8",
                background: "{theme.background}",
                display: "center",
                label {
                    max_lines: "1",
                    color: "{theme.color}",
                    "{cx.props.url}"
                }
            }
        }
    )
}
