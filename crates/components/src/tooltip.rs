use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_get_theme, TooltipTheme};

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

    let url = &cx.props.url;
    let TooltipTheme {
        background,
        color,
        border_fill,
    } = &theme.tooltip;

    render!(
        rect {
            padding: "4 10",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            border: "1 solid {border_fill}",
            corner_radius: "10",
            background: "{background}",
            main_align: "center",
            label {
                max_lines: "1",
                color: "{color}",
                "{url}"
            }
        }
    )
}
