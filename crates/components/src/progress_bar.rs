use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, ProgressBarTheme, ProgressBarThemeWith};

/// Properties for the [`ProgressBar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressBarProps {
    /// Theme override.
    pub theme: Option<ProgressBarThemeWith>,
    /// Show a label with the current progress. Default to false.
    #[props(default = false)]
    pub show_progress: bool,
    /// Percentage of the progress bar.
    pub progress: f32,
}

/// Display the progress of something visually. For example: downloading files, fetching data, etc.
///
/// # Styling
/// Inherits the [`ProgressBarTheme`](freya_hooks::ProgressBarTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ProgressBar {
///             progress: 75.0
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn ProgressBar(
    ProgressBarProps {
        theme,
        show_progress,
        progress,
    }: ProgressBarProps,
) -> Element {
    let ProgressBarTheme {
        color,
        background,
        progress_background,
        width,
        height,
    } = use_applied_theme!(&theme, progress_bar);

    rsx!(
        rect {
            width: "{width}",
            height: "{height}",
            padding: "2",
            rect {
                corner_radius: "999",
                width: "100%",
                height: "100%",
                shadow: "0 2 10 1 rgb(0, 0, 0, 0.2)",
                background: "{background}",
                font_size: "13",
                direction: "horizontal",
                border: "1 solid {background}",
                border_align: "outer",
                rect {
                    corner_radius: "999",
                    width: "{progress}%",
                    height: "100%",
                    background: "{progress_background}",
                    main_align: "center",
                    cross_align: "center",
                    overflow: "clip",
                    if show_progress {
                        label {
                            text_align: "center",
                            width: "100%",
                            color: "{color}",
                            max_lines: "1",
                            "{progress.floor()}%"
                        }
                    }
                }
            }
        }
    )
}
