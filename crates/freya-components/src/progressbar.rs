use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    get_theme,
    theming::component_themes::ProgressBarThemePartial,
};

/// ProgressBar component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     ProgressBar::new(50.)
///         .into()
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect()
///         .padding(8.).center().expanded().child(app()).into()
/// # }, (250., 250.).into(), "./images/gallery_progressbar.png");
/// ```
///
/// # Preview
/// ![Progressbar Preview][progressbar]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("progressbar", "images/gallery_progressbar.png")
)]
#[derive(Clone, PartialEq)]
pub struct ProgressBar {
    pub width: Size,
    pub(crate) theme: Option<ProgressBarThemePartial>,
    pub show_progress: bool,
    pub progress: f32,
}

impl ProgressBar {
    pub fn new(progress: impl Into<f32>) -> Self {
        Self {
            width: Size::fill(),
            theme: None,
            show_progress: true,
            progress: progress.into(),
        }
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }

    pub fn show_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }
}

impl Render for ProgressBar {
    fn render(&self) -> Element {
        let progressbar_theme = get_theme!(&self.theme, progressbar);

        let progress = self.progress.clamp(0., 100.);

        rect()
            .horizontal()
            .width(self.width.clone())
            .height(Size::px(progressbar_theme.height))
            .corner_radius(99.)
            .background(progressbar_theme.background)
            .border(
                Border::new()
                    .width(1.)
                    .alignment(BorderAlignment::Outer)
                    .fill(progressbar_theme.background),
            )
            .font_size(13.)
            .child(
                rect()
                    .horizontal()
                    .width(Size::percent(progress))
                    .height(Size::fill())
                    .corner_radius(99.)
                    .background(progressbar_theme.progress_background)
                    .child(
                        label()
                            .width(Size::fill())
                            .color(progressbar_theme.color)
                            .text_align(TextAlign::Center)
                            .text(format!("{}%", self.progress))
                            .max_lines(1),
                    ),
            )
            .into()
    }
}
