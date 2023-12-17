use crate::theming::*;
use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};

/// Provide a custom [`Theme`].
pub fn use_init_theme(cx: &ScopeState, theme: Theme) {
    use_shared_state_provider(cx, || theme);
}

/// Provide the default [`Theme`].
pub fn use_init_default_theme(cx: &ScopeState) {
    use_shared_state_provider(cx, Theme::default);
}

/// Subscribe to [`Theme`] changes.
pub fn use_theme(cx: &ScopeState) -> &UseSharedState<Theme> {
    use_shared_state::<Theme>(cx).unwrap()
}

/// Subscribe to [`Theme`] changes, default theme will be used if there is no provided [`Theme`].
///
/// Primarily used by built-in components that have no control of whether they will inherit a [`Theme`] or not.
pub fn use_get_theme(cx: &ScopeState) -> Theme {
    use_shared_state::<Theme>(cx)
        .map(|v| v.read().clone())
        .unwrap_or_default()
}

/// This macro has three arguments separator by commas.
///
/// 1. The context (`cx: Scope`). Just pass in `cx`.
/// 2. The theme property. This should be `&cx.props.theme`.
/// 3. The name of the theme that you want to use.
///
/// # Examples
///
/// ```rust,ignore
/// use freya_hooks::{ButtonTheme, ButtonThemeWith};
///
/// #[derive(Props)]
/// pub struct ButtonProps {
///     /// Theme override.
///     #[props(optional)]
///     pub theme: Option<ButtonThemeWith>,
///     // ...
/// }
///
/// pub fn Button(cx: Scope<ButtonProps>) -> Element {
///     let ButtonTheme {
///         padding,
///         width,
///         background,
///         ..
///     } = use_applied_theme!(cx, &cx.props.theme, button);
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! use_applied_theme {
    ($cx:expr, $theme_prop:expr, $theme_name:ident) => {{
        let mut theme = ::freya_hooks::use_get_theme($cx).$theme_name;

        if let Some(theme_override) = $theme_prop {
            theme.apply_optional(theme_override);
        }

        theme
    }};
}
