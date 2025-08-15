use dioxus_core::{
    try_consume_context,
    use_hook,
};
use dioxus_hooks::{
    use_context,
    use_context_provider,
};
use dioxus_signals::{
    ReadableExt,
    Signal,
};

use crate::theming::*;

/// Provide a custom [`Theme`].
pub fn use_init_theme(theme_cb: impl FnOnce() -> Theme) -> Signal<Theme> {
    use_context_provider(|| Signal::new(theme_cb()))
}

/// Provide the default [`Theme`].
pub fn use_init_default_theme() -> Signal<Theme> {
    use_context_provider(|| Signal::new(Theme::default()))
}

/// Subscribe to [`Theme`] changes.
pub fn use_theme() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

/// Subscribe to [`Theme`] changes, default theme will be used if there is no provided [`Theme`].
///
/// Primarily used by built-in components that have no control of whether they will inherit a [`Theme`] or not.
pub fn use_get_theme() -> Theme {
    try_consume_context::<Signal<Theme>>()
        .map(|v| v.read().clone())
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
pub fn use_init_surface_theme_indicator(
    theme: impl FnOnce() -> SurfaceThemeIndicator,
) -> SurfaceThemeIndicator {
    use_context_provider(theme)
}

/// Get the inherited [SurfaceThemeIndicator].
pub fn use_surface_theme_indicator() -> SurfaceThemeIndicator {
    use_hook(|| try_consume_context().unwrap_or_default())
}

/// This macro has three arguments separator by commas.
///
/// 1. The theme property. This should be `&cx.props.theme`.
/// 2. The name of the theme that you want to use.
///
/// # Examples
///
/// ```rust
/// use freya::prelude::*;
///
/// #[derive(Props, PartialEq, Clone)]
/// pub struct ButtonProps {
///     /// Theme override.
///     pub theme: Option<ButtonThemeWith>,
///     // ...
/// }
///
/// pub fn Button(props: ButtonProps) -> Element {
///     let ButtonTheme {
///         padding,
///         width,
///         background,
///         ..
///     } = use_applied_theme!(&props.theme, button);
///     // ...
///
///     # VNode::empty()
/// }
/// ```
#[macro_export]
macro_rules! use_applied_theme {
    ($theme_prop:expr, $theme_name:ident) => {{
        let mut theme = ::freya_hooks::use_get_theme();
        let mut requested_theme = theme.$theme_name;

        if let Some(theme_override) = $theme_prop {
            requested_theme.apply_optional(theme_override);
        }

        requested_theme.apply_colors(&theme.colors);

        requested_theme
    }};
}
