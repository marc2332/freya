use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_hooks::{use_init_default_theme, use_init_theme, Theme};

/// Properties for the Switch component.
#[derive(Props)]
pub struct ThemeProviderProps<'a> {
    #[props(optional)]
    pub theme: Option<Theme>,
    pub children: Element<'a>,
}

/// Provides a Theme for all it's children.
#[allow(non_snake_case)]
pub fn ThemeProvider<'a>(cx: Scope<'a, ThemeProviderProps<'a>>) -> Element<'a> {
    if let Some(theme) = cx.props.theme.as_ref() {
        use_init_theme(&cx, theme.clone());
    } else {
        use_init_default_theme(&cx);
    }

    render!(&cx.props.children)
}
