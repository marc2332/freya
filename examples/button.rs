#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::sleep;

fn main() {
    launch_with_params(app, "Button", (600.0, 350.0));
}

#[component]
fn EnabledButtons() -> Element {
    let mut theme = use_theme();
    rsx!(
        rect {
            width: "flex(1)",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            spacing: "10",
                Button {
                onclick: move |_| {
                    if *theme.read() == DARK_THEME {
                        theme.set(LIGHT_THEME)
                    } else {
                        theme.set(DARK_THEME)
                    }
                },
                label { "Change Theme" }
            }
            FilledButton {
                onpress: move |_| println!("Button Pressed!"),
                label { "Button B" }
            }
            OutlineButton {
                label { "Button C" }
            }
        }
    )
}

#[component]
fn DisabledButtons() -> Element {
    rsx!(
        rect {
            width: "flex(1)",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            spacing: "10",
            Button {
                onclick: move |_| println!("Button Clicked!"),
                enabled: false,
                label { "Button A" }
            }
            FilledButton {
                onpress: move |_| println!("Button Pressed!"),
                enabled: false,
                label { "Button B" }
            }
            OutlineButton {
                enabled: false,
                label { "Button C" }
            }
        }
    )
}

#[component]
fn LoadingButtons() -> Element {
    let mut loading = use_signal(|| false);

    let load = move |_| {
        spawn(async move {
            loading.set(true);
            sleep(Duration::from_secs(2)).await;
            loading.set(false);
        });
    };

    rsx!(
        rect {
            width: "flex(1)",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            spacing: "10",
            Button {
                onclick: load,
                enabled: !loading(),
                rect {
                    direction: "horizontal",
                    cross_align: "center",
                    spacing: "8",
                    if loading() {
                        Loader { size: "16" }
                    }
                    label { "Button A" }
                }
            }
            FilledButton {
                onclick: load,
                enabled: !loading(),
                rect {
                    direction: "horizontal",
                    cross_align: "center",
                    spacing: "8",
                    if loading() {
                        Loader { size: "16" }
                    }
                    label { "Button B" }
                }
            }
            OutlineButton {
                onclick: load,
                enabled: !loading(),
                rect {
                    direction: "horizontal",
                    cross_align: "center",
                    spacing: "8",
                    if loading() {
                        Loader { size: "16" }
                    }
                    label { "Button C" }
                }
            }
        }
    )
}

fn app() -> Element {
    use_init_default_theme();
    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                direction: "horizontal",
                content: "flex",
                padding: "16",
                EnabledButtons {}
                DisabledButtons {}
                LoadingButtons {}
            }
        }
    )
}
