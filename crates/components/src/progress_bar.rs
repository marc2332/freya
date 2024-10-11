use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_applied_theme,
    ProgressBarTheme,
    ProgressBarThemeWith,
};

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
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ProgressBar { 
///             show_progress: true,
///             progress: 50.0
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           ProgressBar {
/// #               theme: theme_with!(ProgressBarTheme {
/// #                   width: "50%".into(),
/// #               }),
/// #               show_progress: true,
/// #               progress: 50.0
/// #           }
/// #       }
/// #   )
/// # }, (200., 150.).into(), "./images/gallery_progress_bar.png");
/// ```
///
/// # Preview
/// ![ProgressBar Preview][progress_bar]
#[embed_doc_image::embed_doc_image("progress_bar", "images/gallery_progress_bar.png")]
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

    let progress = progress.clamp(0., 100.);

    rsx!(
        rect {
            width: "{width}",
            height: "{height}",
            padding: "2",
            rect {
                corner_radius: "999",
                width: "100%",
                height: "100%",
                background: "{background}",
                font_size: "13",
                direction: "horizontal",
                border: "1 outer {background}",
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
