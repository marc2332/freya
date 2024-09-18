#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[component]
pub fn BorderBox(border: String) -> Element {
    rsx!(
        rect {
            width: "fill",
            height: "72",
            corner_radius: "8",
            main_align: "center",
            cross_align: "center",
            color: "#ffffff",
            border: "{border}",

            label { "{border}" }
        }
    )
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);

    rsx!(
        Body {
            ScrollView {
                padding: "16",
                spacing: "16",

                BorderBox {
                    border: "1 inner blue",
                }

                BorderBox {
                    border: "4 center radial-gradient(red 0%, blue 100%)",
                }

                BorderBox {
                    border: "2 inner linear-gradient(0deg, #7a5eeb 0%, #c85eeb 33%, #e8ace0 66%, white 100%)",
                }

                BorderBox {
                    border: "2 outer red, 2 inner blue",
                }

                BorderBox {
                    border: "6 inner red, 5 inner orange, 4 inner yellow, 3 inner green, 2 inner blue, 1 inner purple",
                }

                BorderBox {
                    border: "2 0 0 0 inner green",
                }

                BorderBox {
                    border: "2 0 inner #ffffff",
                }

                BorderBox {
                    border: "2 4 6 8 inner orange",
                }

                BorderBox {
                    border: "2 0 0 0 inner red, 0 2 0 0 inner blue",
                }
            }
        }
    )
}
