use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, TooltipTheme, TooltipThemeWith};

/// Properties for the [`Tooltip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Theme override.
    pub theme: Option<TooltipThemeWith>,
    /// Url as the Tooltip destination.
    pub url: String,
}

/// `Tooltip` component
///
/// # Styling
/// Inherits the [`TooltipTheme`](freya_hooks::TooltipTheme)
#[allow(non_snake_case)]
pub fn Tooltip(TooltipProps { url, theme }: TooltipProps) -> Element {
    let theme = use_applied_theme!(&theme, tooltip);
    let TooltipTheme {
        background,
        color,
        border_fill,
    } = theme;

    rsx!(
        rect {
            padding: "4 10",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            border: "1 solid {border_fill}",
            corner_radius: "10",
            background: "{background}",
            main_align: "center",
            label { max_lines: "1", color: "{color}", "{url}" }
        }
    )
}
