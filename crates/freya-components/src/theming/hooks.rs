use freya_core::prelude::{
    State,
    provide_context,
    try_consume_context,
    use_consume,
    use_hook,
};

use crate::theming::{
    component_themes::Theme,
    themes::LIGHT_THEME,
};

/// Provides a custom [`Theme`].
pub fn use_init_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create(theme_cb());
        provide_context(state);
        state
    })
}

/// Provide [LIGHT_THEME] as the default [`Theme`].
pub fn use_init_default_theme() -> State<Theme> {
    use_init_theme(|| LIGHT_THEME)
}

/// Subscribe to [`Theme`] changes.
pub fn use_theme() -> State<Theme> {
    use_consume::<State<Theme>>()
}

/// Subscribe to [`Theme`] changes, default theme will be used if there is no provided [`Theme`].
///
/// Primarily used by built-in components that have no control of whether they will inherit a [`Theme`] or not.
pub fn get_theme_or_default() -> Theme {
    try_consume_context::<State<Theme>>()
        .map(|theme| theme.read().to_owned())
        .unwrap_or_default()
}

/// Indicates what type of surface to use.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SurfaceThemeIndicator {
    #[default]
    Primary,
    Opposite,
}

/// Provide a [SurfaceThemeIndicator] down to the components.
pub fn use_init_surface_theme_indicator(theme: impl FnOnce() -> SurfaceThemeIndicator) {
    use_hook(|| provide_context(theme()))
}

/// Get the inherited [SurfaceThemeIndicator].
pub fn use_surface_theme_indicator() -> SurfaceThemeIndicator {
    use_hook(|| try_consume_context().unwrap_or_default())
}
