#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}
fn app() -> Element {
    let mut state = use_signal(Vec::new);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            onglobalmousemove: move |e: MouseEvent| {
                state.push(1);
            },
            onglobalclick: move |e: MouseEvent| {
                state.push(2);
            },
            label {
                "{state:?}"
            }
        }
    )
}
