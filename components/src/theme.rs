use dioxus::prelude::*;
use freya_hooks::{use_init_default_theme, use_init_theme, Theme};

/// [`ThemeProvider`] component properties.
#[derive(Props)]
pub struct ThemeProviderProps<'a> {
    /// Theme to provide.
    #[props(optional)]
    pub theme: Option<Theme>,
    /// Inner children to provide a Theme to.
    pub children: Element<'a>,
}

/// Provides a `Theme` for all its children.
///
/// # Props
/// See [`ThemeProviderProps`]
///
#[allow(non_snake_case)]
pub fn ThemeProvider<'a>(cx: Scope<'a, ThemeProviderProps<'a>>) -> Element<'a> {
    if let Some(theme) = cx.props.theme.as_ref() {
        use_init_theme(cx, theme.clone());
    } else {
        use_init_default_theme(cx);
    }

    render!(&cx.props.children)
}
