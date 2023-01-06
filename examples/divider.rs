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
            height: "stretch",
            width: "stretch",
            direction: "horizontal",
            padding: "30",
            rect {
                width: "{sizes.0}%",
                height: "stretch",
                rect {
                    background: "red",
                    height: "{sizes.2}%",
                    width: "stretch",
                    padding: "30",
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
                    width: "stretch",
                    padding: "30",
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
                height: "stretch",
                rect {
                    background: "blue",
                    height: "{sizes.2}%",
                    width: "stretch",
                    padding: "30",
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
                    width: "stretch",
                    padding: "30",
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
