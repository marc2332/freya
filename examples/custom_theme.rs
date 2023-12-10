#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

const CUSTOM_THEME: Theme = Theme {
    button: ButtonTheme {
        background: Cow::Borrowed("rgb(230, 0, 0)"),
        hover_background: Cow::Borrowed("rgb(150, 0, 0)"),
        border_fill: Cow::Borrowed("rgb(120, 0, 0)"),
        corner_radius: Cow::Borrowed("4"),
        height: Cow::Borrowed("auto"),
        width: Cow::Borrowed("auto"),
        margin: Cow::Borrowed("5"),
        padding: Cow::Borrowed("6 12"),
        font_theme: FontTheme {
            color: Cow::Borrowed("white"),
        },
    },
    ..LIGHT_THEME
};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            theme: CUSTOM_THEME,
            rect {
                width: "100%",
                height: "100%",
                Button {
                    label {
                        "Cancel"
                    }
                }
            }
        }
    )
}
