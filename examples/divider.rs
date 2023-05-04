#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let sizes = use_state(cx, || (50, 50, 50, 50));

    render!(
        rect {
            height: "100%",
            width: "100%",
            direction: "horizontal",
            padding: "15",
            color: "white",
            rect {
                width: "{sizes.0}%",
                height: "100%",
                rect {
                    background: "red",
                    height: "{sizes.2}%",
                    width: "100%",
                    padding: "15",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 += 5;
                        v.1 -= 5;
                        v.2 += 5;
                        v.3 -= 5;
                    }),
                    label {
                        "Click to increase",
                    }
                }
                rect {
                    background: "green",
                    height: "{sizes.3}%",
                    width: "100%",
                    padding: "15",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 += 5;
                        v.1 -= 5;
                        v.2 -= 5;
                        v.3 += 5;
                    }),
                    label {
                        "Click to increase",
                    }
                }
            }
            rect {
                width: "{sizes.1}%",
                height: "100%",
                rect {
                    background: "blue",
                    height: "{sizes.2}%",
                    width: "100%",
                    padding: "15",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 -= 5;
                        v.1 += 5;
                        v.2 += 5;
                        v.3 -= 5;
                    }),
                    label {
                        "Click to increase",
                    }
                }
                rect {
                    background: "black",
                    height: "{sizes.3}%",
                    width: "100%",
                    padding: "15",
                    onclick: |_| sizes.with_mut(|v| {
                        v.0 -= 5;
                        v.1 += 5;
                        v.2 -= 5;
                        v.3 += 5;
                    }),
                    label {
                        "Click to increase",
                    }
                }
            }
        }
    )
}
