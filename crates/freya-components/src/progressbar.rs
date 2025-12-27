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
/// fn app() -> impl IntoElement {
///     ProgressBar::new(50.)
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().padding(8.).center().expanded().child(app())
/// # }, "./images/gallery_progressbar.png").render();
/// ```
///
/// # Preview
/// ![Progressbar Preview][progressbar]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("progressbar", "images/gallery_progressbar.png")
)]
#[derive(Clone, PartialEq)]
pub struct ProgressBar {
    pub(crate) theme: Option<ProgressBarThemePartial>,
    width: Size,
    show_progress: bool,
    progress: f32,
    key: DiffKey,
}

impl KeyExt for ProgressBar {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ProgressBar {
    pub fn new(progress: impl Into<f32>) -> Self {
        Self {
            width: Size::fill(),
            theme: None,
            show_progress: true,
            progress: progress.into(),
            key: DiffKey::None,
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
    fn render(&self) -> impl IntoElement {
        let progressbar_theme = get_theme!(&self.theme, progressbar);

        let progress = self.progress.clamp(0., 100.);

        rect()
            .a11y_alt(format!("Progress {progress}%"))
            .a11y_focusable(true)
            .a11y_role(AccessibilityRole::ProgressIndicator)
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
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
