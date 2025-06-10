#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Scroll example", (400.0, 400.0));
}

fn app() -> Element {
    use_init_default_theme();
    rsx!(
        Body {
            ScrollView {
                spacing: "8",
                padding: "8",
                height: "auto",
                min_height: 50.,
                max_height: 400.,
                Card {}
                Card {}
                Card {}
                Card {}
            }
        }
    )
}

#[component]
fn Card() -> Element {
    let mut theme = use_theme();
    let is_dark = theme.read().name == "dark";

    rsx!(
        rect {
            height: "9000",
            width: "300",
            background: "rgb(43, 106, 208)",
            corner_radius: "16",
            main_align: "center",
            cross_align: "center",
            Switch {
                enabled: is_dark,
                ontoggled: move |_| {
                    if is_dark {
                        *theme.write() = LIGHT_THEME
                    } else {
                        *theme.write() = DARK_THEME
                    }
                }
            }
        }
    )
}
