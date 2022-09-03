use components::*;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use trev::launch;

fn main() {
    launch(app);
}

const LIGHT_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(200, 200, 200)",
        hover_background: "rgb(140, 140, 140)",
    },
};

const DARK_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(35, 35, 35)",
        hover_background: "rgb(115, 115, 115)",
    },
};

fn app(cx: Scope) -> Element {
    let theme = use_state(&cx, || DARK_THEME);

    cx.render(rsx!(ThemeProvider {
        theme: theme.get().clone(),
        child: cx.render(rsx! (
            Button {
                on_click: |_| {
                    theme.set(LIGHT_THEME)
                },
                child: cx.render(rsx!(
                    text {
                        width: "100",
                        "Light"
                    }
                ))
            }
            Button {
                on_click: |_| {
                    theme.set(DARK_THEME)
                },
                child: cx.render(rsx!(
                    text {
                        width: "100",
                        "Dark"
                    }
                ))
            }
        ))
    }))
}
