use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    get_theme,
    theming::component_themes::CircularLoaderThemePartial,
};

/// Circular loader component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     CircularLoader::new()
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect()
///         .spacing(8.).center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_circular_loader.png");
/// ```
///
/// # Preview
/// ![Circular Loader Preview][circular_loader]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("circular_loader", "images/gallery_circular_loader.png")
)]
#[derive(PartialEq)]
pub struct CircularLoader {
    pub(crate) theme: Option<CircularLoaderThemePartial>,
    size: f32,
    key: DiffKey,
}

impl KeyExt for CircularLoader {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for CircularLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl CircularLoader {
    pub fn new() -> Self {
        Self {
            size: 32.,
            theme: None,
            key: DiffKey::None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Render for CircularLoader {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, circular_loader);

        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::Restart);
            AnimNum::new(0.0, 360.0).time(650)
        });

        svg(Bytes::from_static(
            r#"<svg viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
                <circle class="spin" cx="300" cy="300" fill="none"
                r="250" stroke-width="64" stroke="{color}"
                stroke-dasharray="256 1400"
                stroke-linecap="round" />
            </svg>"#
                .as_bytes(),
        ))
        .width(Size::px(self.size))
        .height(Size::px(self.size))
        .stroke(theme.primary_color)
        .rotate(animation.get().value())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
