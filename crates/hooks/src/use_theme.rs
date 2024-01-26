use crate::theming::*;
use dioxus_core::prelude::try_consume_context;
use dioxus_hooks::{use_context, use_context_provider};
use dioxus_signals::{Readable, Signal};

/// Provide a custom [`Theme`].
pub fn use_init_theme(theme: Theme) {
    use_context_provider(|| Signal::new(theme));
}

/// Provide the default [`Theme`].
pub fn use_init_default_theme() {
    use_context_provider(|| Signal::new(Theme::default()));
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

/// This macro has three arguments separator by commas.
///
/// 1. The theme property. This should be `&cx.props.theme`.
/// 2. The name of the theme that you want to use.
///
/// # Examples
///
/// ```rust,ignore
/// use freya_hooks::{ButtonTheme, ButtonThemeWith};
///
/// #[derive(Props)]
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
/// }
/// ```
#[macro_export]
macro_rules! use_applied_theme {
    ($theme_prop:expr, $theme_name:ident) => {{
        let mut theme = ::freya_hooks::use_get_theme().$theme_name;

        if let Some(theme_override) = $theme_prop {
            theme.apply_optional(theme_override);
        }

        theme
    }};
}
