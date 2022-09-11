#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use components::Switch;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let enabled = use_state(&cx, || false);

    let is_enabled = if *enabled.get() { "Yes" } else { "No" };

    cx.render(rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            label {
                color: "black",
                "Is enabled? {is_enabled}"
            }
            Switch {
                enabled: enabled,
                ontoggled: |_| {
                    enabled.set(!enabled.get());
                }
            }
        }
    ))
}
