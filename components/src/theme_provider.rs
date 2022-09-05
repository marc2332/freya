use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

#[derive(Clone, Debug, PartialEq)]
pub struct ButtonTheme {
    pub background: &'static str,
    pub hover_background: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub button: ButtonTheme,
}

#[allow(non_snake_case)]
pub fn ThemeProvider<'a>(cx: Scope<'a, ThemeProviderProps<'a>>) -> Element {
    let theme = use_state(&cx, || {
        if let Some(theme) = &cx.props.theme {
            theme.clone()
        } else {
            Theme {
                button: ButtonTheme {
                    background: "red",
                    hover_background: "yellow",
                },
            }
        }
    });

    let set_theme = theme.setter();

    use_effect(&cx, &cx.props.theme, move |theme| async move {
        if let Some(ref theme) = theme {
            set_theme(theme.clone());
        }
    });

    let theme = theme.get().clone();

    cx.provide_context(theme);

    cx.render(rsx!(&cx.props.child))
}

#[derive(Props)]
pub struct ThemeProviderProps<'a> {
    child: Element<'a>,
    #[props(optional)]
    theme: Option<Theme>,
}
