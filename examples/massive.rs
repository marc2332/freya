#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let enabled = use_state(cx, || false);

    let is_enabled = if *enabled.get() { "Yes" } else { "No" };

    let cols = 15;
    let rows = 15;

    render!(
        container {
            width: "100%",
            height: "100%",
            padding: "5",
            label {
                height: "35",
                color: "black",
                "Is enabled? {is_enabled}"
            }
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                (0..cols).map(|col| {
                    rsx! {
                        rect {
                            key: "{col}",
                            width: "calc(100% / {cols})",
                            height: "100%",
                            (0..rows).map(|row| {
                                rsx! {
                                    Switch {
                                        key: "{row}{col}",
                                        enabled: *enabled.get(),
                                        ontoggled: |_| {
                                            enabled.set(!enabled.get());
                                        }
                                    }
                                }
                            })
                        }
                    }
                })
            }
        }
    )
}
