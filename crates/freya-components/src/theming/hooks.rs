use freya_core::prelude::{
    Readable,
    State,
    WritableUtils,
    provide_context,
    try_consume_context,
    use_consume,
    use_hook,
};

use crate::theming::component_themes::Theme;

/// Provides a custom [`Theme`].
/// If a [`Theme`] context already exists, it reuses it instead of creating a new one.
pub fn use_init_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        if let Some(mut existing) = try_consume_context::<State<Theme>>() {
            existing.set(theme_cb());
            existing
        } else {
            let state = State::create(theme_cb());
            provide_context(state);
            state
        }
    })
}

/// Provides a custom [`Theme`].
pub fn use_provide_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create(theme_cb());
        provide_context(state);
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
