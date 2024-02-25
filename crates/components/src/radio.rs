use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_applied_theme, RadioTheme, RadioThemeWith};

/// Controlled `Radio` component.
///
/// # Styling
/// Inherits the [`RadioTheme`](freya_hooks::RadioTheme) theme.
///
/// # Example
///
/// ```no_run
/// todo!()
/// ```
///
#[allow(non_snake_case)]
#[component]
pub fn Radio(
    /// Indicate whether this radio is selected or not.
    selected: bool,
    /// Theme override.
    theme: Option<RadioThemeWith>,
) -> Element {
    let RadioTheme {
        unselected_fill,
        selected_fill,
    } = use_applied_theme!(&theme, radio);
    let fill = if selected {
        selected_fill
    } else {
        unselected_fill
    };

    rsx!(
        rect {
            width: "18",
            height: "18",
            border: "2 solid {fill}",
            padding: "4",
            main_align: "center",
            cross_align: "center",
            corner_radius: "99",
            if selected {
                rect {
                    width: "10",
                    height: "10",
                    background: "{fill}",
                    corner_radius: "99",
                }
            }
        }
    )
}
