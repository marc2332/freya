use crate::TickIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_applied_theme, CheckboxTheme, CheckboxThemeWith};

/// Controlled `Checkbox` component.
///
/// # Styling
/// Inherits the [`CheckboxTheme`](freya_hooks::CheckboxTheme) theme.
///
/// # Example
///
/// ```no_run
/// todo!()
/// ```
///
#[allow(non_snake_case)]
#[component]
pub fn Checkbox(
    /// Indicate whether this checkbox is selected or not.
    selected: bool,
    /// Theme override.
    theme: Option<CheckboxThemeWith>,
) -> Element {
    let CheckboxTheme {
        unselected_fill,
        selected_fill,
        selected_icon_fill,
    } = use_applied_theme!(&theme, checkbox);
    let (fill, border) = if selected {
        (selected_fill.as_ref(), selected_fill.as_ref())
    } else {
        ("transparent", unselected_fill.as_ref())
    };

    rsx!(
        rect {
            width: "18",
            height: "18",
            padding: "4",
            main_align: "center",
            cross_align: "center",
            corner_radius: "4",
            border: "2 solid {border}",
            background: "{fill}",
            if selected {
                TickIcon {
                    fill: selected_icon_fill
                }
            }
        }
    )
}
