#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

macro_rules! cb {
    ($val:expr) => {
        Cow::Borrowed($val)
    };
}

const CUSTOM_THEME: Theme = Theme {
    button: ButtonTheme {
        background: cb!("rgb(230, 0, 0)"),
        hover_background: cb!("rgb(150, 0, 0)"),
        border_fill: cb!("rgb(120, 0, 0)"),
        font_theme: FontTheme {
            color: cb!("white"),
        },
        ..LIGHT_THEME.button
    },
    ..LIGHT_THEME
};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider { theme: CUSTOM_THEME,
            rect { width: "100%", height: "100%",
                Button { label { "Report" } }
            }
        }
    )
}
