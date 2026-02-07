use freya_core::{
    prelude::{
        Readable,
        State,
        provide_context,
        provide_context_for_scope_id,
        try_consume_context,
        use_consume,
        use_hook,
    },
    scope_id::ScopeId,
};

use crate::theming::component_themes::Theme;

/// Provides a custom [`Theme`].
pub fn use_init_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create(theme_cb());
        provide_context(state);
        state
    })
}

pub fn use_init_root_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create_in_scope(theme_cb(), ScopeId::ROOT);
        provide_context_for_scope_id(state, ScopeId::ROOT);
        state
    })
}

/// Subscribe to [`Theme`] changes.
pub fn use_theme() -> State<Theme> {
    use_consume::<State<Theme>>()
}

/// Subscribe to [`Theme`] changes, default theme will be used if there is no provided [`Theme`].
///
/// Primarily used by built-in components that have no control of whether they will inherit a [`Theme`] or not.
pub fn get_theme_or_default() -> Readable<Theme> {
    try_consume_context::<State<Theme>>()
        .map(|v| v.into())
        .unwrap_or_else(|| Theme::default().into())
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
