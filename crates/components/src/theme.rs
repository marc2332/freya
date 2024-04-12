use dioxus::prelude::*;
use freya_hooks::{use_init_theme, Theme};

/// Properties for the [`ThemeProvider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    /// Theme to provide.
    pub theme: Option<Theme>,
    /// Inner children to provide a Theme to.
    pub children: Element,
}

/// Provides a `Theme` for all its children.
#[allow(non_snake_case)]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    use_init_theme(props.theme.unwrap_or_default());

    rsx!({ props.children })
}
