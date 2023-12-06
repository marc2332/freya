use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_get_theme, ProgressBarTheme};

/// [`ProgressBar`] component properties.
#[derive(Props, PartialEq)]
pub struct ProgressBarProps {
    /// Show a label with the current progress. Default to false.
    #[props(default = false)]
    show_progress: bool,

    /// Width of the progress bar. Default to 100%.
    #[props(default = "100%".to_string(), into)]
    width: String,

    /// Height of the progress bar. Default to 20px.
    #[props(default = "20".to_string(), into)]
    height: String,
    /// Percentage of the progress bar.
    pub progress: f32,
}

/// `ProgressBar` component.
///
/// # Props
/// See [`ProgressBarProps`].
///
/// # Styling
/// Inherits the [`ProgressBarTheme`](freya_hooks::ProgressBarTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         ProgressBar {
///             progress: 75.0
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn ProgressBar(cx: Scope<ProgressBarProps>) -> Element {
    let theme = use_get_theme(cx);

    let ProgressBarTheme {
        color,
        background,
        progress_background,
    } = theme.progress_bar;
    let width = &cx.props.width;
    let height = &cx.props.height;
    let show_progress = cx.props.show_progress;
    let progress = cx.props.progress;

    render!(
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
                rect {
                    corner_radius: "999",
                    width: "{progress}%",
                    height: "100%",
                    background: "{progress_background}",
                    main_align: "center",
                    cross_align: "center",
                    overflow: "clip",
                    if show_progress {
                        rsx!(
                            label {
                                text_align: "center",
                                width: "100%",
                                color: "{color}",
                                max_lines: "1",
                                "{progress.floor()}%"
                            }
                        )
                    }
                }
            }
        }
    )
}
