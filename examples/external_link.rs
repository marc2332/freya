#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{
    dioxus_elements::{self},
    *,
};

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn ThemeSwitcher(cx: Scope) -> Element {
    let theme = use_theme(&cx);

    let is_enabled = theme.read().name == "dark";

    render!(Switch {
        enabled: is_enabled,
        ontoggled: move |_| {
            if is_enabled {
                *theme.write() = LIGHT_THEME
            } else {
                *theme.write() = DARK_THEME
            }
        }
    })
}

fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            theme: LIGHT_THEME,
            container {
                width: "100%",
                height: "100%",
                color: "black",
                ThemeSwitcher {

                },
                ExternalLink {
                    url: "https://link1.com",
                    label {
                        font_size: "25",
                        "link1"
                    }
                }
                ExternalLink {
                    url: "https://link2.com",
                    label {
                        font_size: "25",
                        "link2"
                    }
                }
                ExternalLink {
                    url: "https://link3.com",
                    label {
                        font_size: "25",
                        "link3"
                    }
                }
            }
        }
    )
}
