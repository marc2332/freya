use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    cache::AssetAge,
    define_theme,
    get_theme,
    svg_viewer::SvgViewer,
};

define_theme! {
    %[component]
    pub CircularLoader {
        %[fields]
        primary_color: Color,
    }
}

/// Circular loader component.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     CircularLoader::new()
/// }
/// ```
///
/// See the [interactive components demo](https://freyaui.dev/demo).
#[derive(PartialEq)]
pub struct CircularLoader {
    pub(crate) theme: Option<CircularLoaderThemePartial>,
    size: f32,
    accessibility: AccessibilityData,
    key: DiffKey,
}

impl KeyExt for CircularLoader {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl AccessibilityExt for CircularLoader {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
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
            accessibility: AccessibilityData::default(),
            key: DiffKey::None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Component for CircularLoader {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(
            &self.theme,
            CircularLoaderThemePreference,
            "circular_loader"
        );

        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::restart());
            AnimNum::new(0.0, 360.0).time(650)
        });

        SvgViewer::new(
            r#"<svg viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
                <circle class="spin" cx="300" cy="300" fill="none"
                r="250" stroke-width="64" stroke="{color}"
                stroke-dasharray="256 1400"
                stroke-linecap="round" />
            </svg>"#
                .as_bytes(),
        )
        .show_loader(false)
        .asset_age(AssetAge::zero())
        .accessibility(self.accessibility.clone())
        .a11y_role(AccessibilityRole::ProgressIndicator)
        .width(Size::px(self.size))
        .height(Size::px(self.size))
        .stroke(theme.primary_color)
        .rotation(animation.get().value())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
