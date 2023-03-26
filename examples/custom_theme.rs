#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

const CUSTOM_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(230, 0, 0)",
        hover_background: "rgb(150, 0, 0)",
        font_theme: FontTheme { color: "white" },
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
                        "Report"
                    }
                }
            }
        }
    )
}
